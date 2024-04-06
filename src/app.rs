use crate::{error::Error, widgets::SelectableList};
use nvd_cve::cve::Cve;
use tui_input::Input;

/// Type alias for the standard [`Result`] type.
pub type AppResult<T> = std::result::Result<T, Error>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// List of CVEs.
    pub cves: Vec<Cve>,
    /// List of CVE's for rendering.
    pub list: SelectableList<Cve>,
    /// Input.
    pub input: Input,
    /// Enable input.
    pub input_mode: bool,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(cves: Vec<Cve>) -> Self {
        Self {
            running: true,
            cves: cves.clone(),
            list: SelectableList::with_items(cves),
            input: Input::default(),
            input_mode: false,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
