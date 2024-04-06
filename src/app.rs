use crate::{error::Error, widgets::SelectableList};
use nvd_cve::cve::Cve;

/// Type alias for the standard [`Result`] type.
pub type AppResult<T> = std::result::Result<T, Error>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// List of CVE's.
    pub cves: SelectableList<Cve>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(cves: Vec<Cve>) -> Self {
        Self {
            running: true,
            cves: SelectableList::with_items(cves),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
