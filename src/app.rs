use crate::{cve::Cve, error::Error, theme::Theme, widgets::SelectableList};
use tui_input::Input;

/// Type alias for the standard [`Result`] type.
pub type AppResult<T> = std::result::Result<T, Error>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Application theme.
    pub theme: Theme,
    /// List of CVEs.
    pub cves: Vec<Cve>,
    /// List of CVE's for rendering.
    pub list: SelectableList<Cve>,
    /// Input.
    pub input: Input,
    /// Enable input.
    pub input_mode: bool,
    /// Show details.
    pub show_details: bool,
    /// Scroll index for details.
    pub scroll_index: usize,
    /// Whether if the details is currently scrollable.
    pub scroll_details: bool,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(cves: Vec<Cve>, theme: Theme, query: String) -> Self {
        Self {
            running: true,
            theme,
            cves: cves.clone(),
            list: SelectableList::with_items(cves),
            input: if query.is_empty() {
                Input::default()
            } else {
                Input::new(query)
            },
            input_mode: false,
            show_details: false,
            scroll_index: 0,
            scroll_details: false,
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
