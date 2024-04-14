use clap::Parser;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::{io, thread};
use tui_jan::app::{App, AppResult};
use tui_jan::args::Args;
use tui_jan::cve::Cve;
use tui_jan::error::Error;
use tui_jan::event::{Event, EventHandler};
use tui_jan::handler::handle_key_events;
use tui_jan::tui::Tui;
use tui_jan::widgets::SelectableList;

use nvd_cve::cache::{get_all, sync_blocking, CacheConfig};
use nvd_cve::client::{BlockingHttpClient, ReqwestBlockingClient};

fn main() -> AppResult<()> {
    // Parse command-line arguments.
    let args = Args::parse();

    // Fetch CVEs.
    let config = CacheConfig {
        url: args.url.to_string(),
        feeds: args.feeds()?,
        db: args.db.unwrap_or_else(CacheConfig::default_db_path),
        show_progress: true,
        force_update: args.force_update,
    };
    if !args.offline {
        let client = ReqwestBlockingClient::new(&config.url, None, None, None);
        sync_blocking(&config, client).map_err(Error::CacheError)?;
    }
    let cves = get_all(&config).map_err(Error::CacheError)?;

    // Create an application.
    let mut app = App::new(cves.into_iter().map(Cve::from).collect());

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app, &tui.events.sender)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            Event::Search => {
                let query = app.input.value().to_lowercase();
                let items = app.cves.clone();
                let sender = tui.events.sender.clone();
                thread::spawn(move || {
                    let items = items
                        .into_iter()
                        .filter(|cve| {
                            query.is_empty()
                                || cve.id.to_lowercase().contains(&query)
                                || cve
                                    .description
                                    .clone()
                                    .unwrap_or_default()
                                    .to_lowercase()
                                    .contains(&query)
                        })
                        .collect();
                    sender
                        .send(Event::SearchResult(SelectableList::with_items(items)))
                        .unwrap();
                });
            }
            Event::SearchResult(v) => {
                app.list = v;
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
