use clap::ValueEnum;
use ratatui::style::{Color, Style};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, ValueEnum, Default)]
pub enum BuiltinTheme {
    #[default]
    Dracula,
    Nord,
    OneDark,
    SolarizedDark,
}

impl BuiltinTheme {
    pub fn get_theme(&self) -> Option<Theme> {
        match self {
            BuiltinTheme::Dracula => dracula_theme(),
            BuiltinTheme::Nord => nord_theme(),
            BuiltinTheme::OneDark => one_dark_theme(),
            BuiltinTheme::SolarizedDark => solarized_dark_theme(),
        }
    }
}

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

/// <https://draculatheme.com/contribute>
fn dracula_theme() -> Option<Theme> {
    Some(Theme {
        background: Style::default()
            .bg(Color::from_str("#282A36").ok()?)
            .fg(Color::from_str("#F8F8F2").ok()?),
        foreground: Style::default().fg(Color::from_str("#F8F8F2").ok()?),
        header: Style::default()
            .bg(Color::from_str("#BD93F9").ok()?)
            .fg(Color::from_str("#282A36").ok()?),
        footer: Style::default()
            .bg(Color::from_str("#282A36").ok()?)
            .fg(Color::from_str("#8BE9FD").ok()?),
        selected: Style::default()
            .bg(Color::from_str("#44475A").ok()?)
            .fg(Color::from_str("#FFB86C").ok()?),
        borders: Style::default().fg(Color::from_str("#44475A").ok()?),
        indicator: Style::default().fg(Color::from_str("#6272A4").ok()?),
        highlight: Style::default().fg(Color::from_str("#F1FA8C").ok()?),
        index: Style::default().fg(Color::from_str("#BD93F9").ok()?),
        input: Style::default().fg(Color::from_str("#50FA7B").ok()?),
        input_empty: Style::default().fg(Color::from_str("#FF79C6").ok()?),
        scrollbar: Style::default().fg(Color::from_str("#6272A4").ok()?),
    })
}

/// <https://www.nordtheme.com/docs/colors-and-palettes>
fn nord_theme() -> Option<Theme> {
    Some(Theme {
        background: Style::default()
            .bg(Color::from_str("#2E3440").ok()?)
            .fg(Color::from_str("#D8DEE9").ok()?),
        foreground: Style::default().fg(Color::from_str("#D8DEE9").ok()?),
        header: Style::default()
            .bg(Color::from_str("#3B4252").ok()?)
            .fg(Color::from_str("#E5E9F0").ok()?),
        footer: Style::default()
            .bg(Color::from_str("#2E3440").ok()?)
            .fg(Color::from_str("#81A1C1").ok()?),
        selected: Style::default()
            .bg(Color::from_str("#434C5E").ok()?)
            .fg(Color::from_str("#8FBCBB").ok()?),
        borders: Style::default().fg(Color::from_str("#4C566A").ok()?),
        indicator: Style::default().fg(Color::from_str("#88C0D0").ok()?),
        highlight: Style::default().fg(Color::from_str("#EBCB8B").ok()?),
        index: Style::default().fg(Color::from_str("#E5E9F0").ok()?),
        input: Style::default().fg(Color::from_str("#A3BE8C").ok()?),
        input_empty: Style::default().fg(Color::from_str("#BF616A").ok()?),
        scrollbar: Style::default().fg(Color::from_str("#4C566A").ok()?),
    })
}

/// <https://www.color-hex.com/color-palette/1017619>
fn one_dark_theme() -> Option<Theme> {
    Some(Theme {
        background: Style::default()
            .bg(Color::from_str("#282C34").ok()?)
            .fg(Color::from_str("#ABB2BF").ok()?),
        foreground: Style::default().fg(Color::from_str("#ABB2BF").ok()?),
        header: Style::default()
            .bg(Color::from_str("#2C323C").ok()?)
            .fg(Color::from_str("#D19A66").ok()?),
        footer: Style::default()
            .bg(Color::from_str("#282C34").ok()?)
            .fg(Color::from_str("#61AFEF").ok()?),
        selected: Style::default()
            .bg(Color::from_str("#3E4451").ok()?)
            .fg(Color::from_str("#E06C75").ok()?),
        borders: Style::default().fg(Color::from_str("#4B5363").ok()?),
        indicator: Style::default().fg(Color::from_str("#4B5363").ok()?),
        highlight: Style::default().fg(Color::from_str("#98C379").ok()?),
        index: Style::default().fg(Color::from_str("#D19A66").ok()?),
        input: Style::default().fg(Color::from_str("#56B6C2").ok()?),
        input_empty: Style::default().fg(Color::from_str("#E06C75").ok()?),
        scrollbar: Style::default().fg(Color::from_str("#4B5363").ok()?),
    })
}

/// <https://ethanschoonover.com/solarized/>
fn solarized_dark_theme() -> Option<Theme> {
    Some(Theme {
        background: Style::default()
            .bg(Color::from_str("#002B36").ok()?)
            .fg(Color::from_str("#839496").ok()?),
        foreground: Style::default().fg(Color::from_str("#839496").ok()?),
        header: Style::default()
            .bg(Color::from_str("#073642").ok()?)
            .fg(Color::from_str("#93A1A1").ok()?),
        footer: Style::default()
            .bg(Color::from_str("#002B36").ok()?)
            .fg(Color::from_str("#586E75").ok()?),
        selected: Style::default()
            .bg(Color::from_str("#073642").ok()?)
            .fg(Color::from_str("#B58900").ok()?),
        borders: Style::default().fg(Color::from_str("#586E75").ok()?),
        indicator: Style::default().fg(Color::from_str("#268BD2").ok()?),
        highlight: Style::default().fg(Color::from_str("#93A1A1").ok()?),
        index: Style::default().fg(Color::from_str("#93A1A1").ok()?),
        input: Style::default().fg(Color::from_str("#859900").ok()?),
        input_empty: Style::default().fg(Color::from_str("#DC322F").ok()?),
        scrollbar: Style::default().fg(Color::from_str("#586E75").ok()?),
    })
}
