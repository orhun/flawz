use crate::app::{App, AppResult};
use crate::event::EventHandler;
use crate::tui;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use std::io;
use std::panic;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui {
    /// Interface to the Terminal.
    terminal: DefaultTerminal,
    /// Terminal event handler.
    pub events: EventHandler,
}

impl Tui {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: DefaultTerminal, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// Initializes the terminal interface.
    ///
    /// It sets terminal properties that Ratatui's default initialization does not cover.
    pub fn init(&mut self) -> AppResult<()> {
        ratatui::crossterm::execute!(io::stdout(), EnableMouseCapture)?;
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            let _ = ratatui::crossterm::execute!(io::stdout(), DisableMouseCapture);
            panic_hook(panic);
        }));
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    /// [`Draw`] the terminal interface by [`rendering`] the widgets.
    ///
    /// [`Draw`]: ratatui::Terminal::draw
    /// [`rendering`]: crate::tui::render
    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        self.terminal.draw(|frame| tui::render(app, frame))?;
        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It reverts the terminal properties.
    pub fn exit(&mut self) -> AppResult<()> {
        self.terminal.show_cursor()?;
        ratatui::crossterm::execute!(io::stdout(), DisableMouseCapture)?;
        ratatui::try_restore()?;
        Ok(())
    }
}
