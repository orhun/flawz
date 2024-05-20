use clap::Parser;
use flawz::app::{App, AppResult};
use flawz::args::Args;
use flawz::cve::Cve;
use flawz::error::Error;
use flawz::event::{Event, EventHandler};
use flawz::handler::{handle_key_events, handle_mouse_events};
use flawz::terminal::Tui;
use flawz::widgets::SelectableList;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::path::Path;
use std::{io, thread};

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
    if !args.offline && !Path::new(&config.db).exists() || args.force_update {
        let client = ReqwestBlockingClient::new(&config.url, None, None, None);
        sync_blocking(&config, client).map_err(Error::CacheError)?;
    }
    let cves = get_all(&config).map_err(Error::CacheError)?;

    // Create an application.
    let mut app = App::new(
        cves.into_iter().map(Cve::from).collect(),
        args.theme
            .get_theme()
            .ok_or_else(|| Error::ParseColorError)?,
        args.query.clone().unwrap_or_default(),
    );

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    if !args.query.unwrap_or_default().is_empty() {
        tui.events.sender.send(Event::Search)?;
    }

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Key(key_event) => handle_key_events(key_event, &mut app, &tui.events.sender)?,
            Event::Mouse(mouse_event) => {
                handle_mouse_events(mouse_event, &mut app, &tui.events.sender)?
            }
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
