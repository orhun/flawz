use crate::app::App;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Clear, Row, Table, TableState},
    Frame,
};
use tui_input::Input;

/// Maximum number of elements to show in the table.
const TABLE_PAGE_LIMIT: usize = 50;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.size();
    let selected_index = app.list.state.selected().unwrap_or_default();
    let items_len = app.list.items.len();
    let page = selected_index / TABLE_PAGE_LIMIT;
    let mut table_state = TableState::default();
    table_state.select(Some(selected_index % TABLE_PAGE_LIMIT));
    let items = app
        .list
        .items
        .iter()
        .skip(page * TABLE_PAGE_LIMIT)
        .take(TABLE_PAGE_LIMIT)
        .map(|cve| {
            let description = match cve
                .description
                .description_data
                .iter()
                .find(|desc| desc.lang == String::from("en"))
            {
                Some(v) => textwrap::wrap(
                    &v.value,
                    textwrap::Options::new(area.width.saturating_sub(15) as usize),
                )
                .join("\n"),
                None => "No description available.".into(),
            };
            Row::new(vec![cve.cve_data_meta.id.to_string(), description])
                .height(2)
                .bottom_margin(2)
        });
    let block = Block::bordered()
        .border_style(Style::default().fg(Color::Rgb(100, 100, 100)))
        .title_bottom(
            if items_len != 0 {
                Line::from(vec![
                    "|".fg(Color::Rgb(100, 100, 100)),
                    format!("{}/{}", selected_index.saturating_add(1), items_len)
                        .white()
                        .bold(),
                    "|".fg(Color::Rgb(100, 100, 100)),
                ])
            } else {
                Line::default()
            }
            .right_aligned(),
        )
        .title_top(
            Line::from(vec![
                "|".fg(Color::Rgb(100, 100, 100)),
                env!("CARGO_PKG_NAME").white().bold(),
                "|".fg(Color::Rgb(100, 100, 100)),
            ])
            .centered(),
        )
        .title_bottom(if !app.input.value().is_empty() || app.input_mode {
            Line::from(vec![
                "|".fg(Color::Rgb(100, 100, 100)),
                "Search: ".bold().white(),
                app.input.value().white(),
                if app.input_mode { " " } else { "" }.into(),
                "|".fg(Color::Rgb(100, 100, 100)),
            ])
        } else {
            Line::default()
        });
    frame.render_stateful_widget(
        Table::new(items, &[Constraint::Min(13), Constraint::Percentage(100)])
            .header(Row::new(vec!["Name", "Description"]))
            .block(block)
            .highlight_style(Style::default().fg(Color::Green)),
        area,
        &mut table_state,
    );
    render_cursor(app, area, frame);
}

/// Renders the cursor.
fn render_cursor(state: &mut App, area: Rect, frame: &mut Frame<'_>) {
    if state.input_mode {
        let (x, y) = (
            area.x
                + Input::default()
                    .with_value(format!("Search: {}", state.input.value()))
                    .visual_cursor() as u16
                + 2,
            area.bottom().saturating_sub(1),
        );
        frame.render_widget(
            Clear,
            Rect {
                x,
                y,
                width: 1,
                height: 1,
            },
        );
        frame.set_cursor(x, y);
    }
}
