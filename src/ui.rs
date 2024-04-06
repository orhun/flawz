use crate::app::App;
use ratatui::{
    layout::Constraint,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Row, Table, TableState},
    Frame,
};

/// Maximum number of elements to show in the table.
const TABLE_PAGE_LIMIT: usize = 50;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.size();
    let selected_index = app.cves.state.selected().unwrap_or_default();
    let items_len = app.cves.items.len();
    let page = selected_index / TABLE_PAGE_LIMIT;
    let mut table_state = TableState::default();
    table_state.select(Some(selected_index % TABLE_PAGE_LIMIT));
    let items = app
        .cves
        .items
        .iter()
        .skip(page * TABLE_PAGE_LIMIT)
        .take(TABLE_PAGE_LIMIT)
        .map(|cve| {
            Row::new(vec![
                cve.cve_data_meta.id.to_string(),
                match cve
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
                },
            ])
            .height(2)
            .bottom_margin(2)
        });
    frame.render_stateful_widget(
        Table::new(items, &[Constraint::Min(13), Constraint::Percentage(100)])
            .header(Row::new(vec!["Name", "Description"]))
            .block(
                Block::bordered()
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
                    ),
            )
            .highlight_style(Style::default().fg(Color::Green)),
        area,
        &mut table_state,
    );
}
