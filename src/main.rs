use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use tui_jan::app::{App, AppResult};
use tui_jan::error::Error;
use tui_jan::event::{Event, EventHandler};
use tui_jan::handler::handle_key_events;
use tui_jan::tui::Tui;

use nvd_cve::cache::{get_all, sync_blocking, CacheConfig};
use nvd_cve::client::{BlockingHttpClient, ReqwestBlockingClient};

fn main() -> AppResult<()> {
    let mut config = CacheConfig::new();
    config.feeds = vec!["2024".to_string()];

    let client = ReqwestBlockingClient::new(&config.url, None, None, None);

    if let Err(error) = sync_blocking(&config, client) {
        eprintln!("Fatal Error while syncing feeds: {:?}", error);
        std::process::exit(1);
    }

    let cves = get_all(&config).map_err(Error::CacheError)?;

    // Create an application.
    let mut app = App::new(cves);

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
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
