use ratatui::style::{Color, Style};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub background: Style,
    pub foreground: Style,
    pub header: Style,
    pub footer: Style,
    pub borders: Style,
    pub indicator: Style,
    pub highlight: Style,
    pub index: Style,
    pub input: Style,
    pub input_empty: Style,
    pub selected: Style,
    pub scrollbar: Style,
}

impl Default for Theme {
    fn default() -> Self {
        dracula_theme()
    }
}

// <https://draculatheme.com/contribute>
fn dracula_theme() -> Theme {
    Theme {
        background: Style::default()
            .bg(Color::from_str("#282A36").unwrap())
            .fg(Color::from_str("#F8F8F2").unwrap()),
        foreground: Style::default().fg(Color::from_str("#F8F8F2").unwrap()),
        header: Style::default()
            .bg(Color::from_str("#BD93F9").unwrap())
            .fg(Color::from_str("#282A36").unwrap()),
        footer: Style::default()
            .bg(Color::from_str("#282A36").unwrap())
            .fg(Color::from_str("#8BE9FD").unwrap()),
        selected: Style::default()
            .bg(Color::from_str("#44475A").unwrap())
            .fg(Color::from_str("#FFB86C").unwrap()),
        borders: Style::default().fg(Color::from_str("#44475A").unwrap()),
        indicator: Style::default().fg(Color::from_str("#6272A4").unwrap()),
        highlight: Style::default().fg(Color::from_str("#F1FA8C").unwrap()),
        index: Style::default().fg(Color::from_str("#BD93F9").unwrap()),
        input: Style::default().fg(Color::from_str("#50FA7B").unwrap()),
        input_empty: Style::default().fg(Color::from_str("#FF79C6").unwrap()),
        scrollbar: Style::default().fg(Color::from_str("#6272A4").unwrap()),
    }
}
