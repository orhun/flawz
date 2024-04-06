use ratatui::{
    layout::Constraint,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Row, Table},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.size();
    let items = app.cves.items.iter().map(|cve| {
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
                        if app.cves.items.len() != 0 {
                            Line::from(vec![
                                "|".fg(Color::Rgb(100, 100, 100)),
                                format!(
                                    "{}/{}",
                                    app.cves.state.selected().unwrap_or_default(),
                                    app.cves.items.len()
                                )
                                .white()
                                .bold(),
                                "|".fg(Color::Rgb(100, 100, 100)),
                            ])
                        } else {
                            Line::default()
                        }
                        .right_aligned(),
                    ),
            )
            .highlight_style(Style::default().fg(Color::Green)),
        area,
        &mut app.cves.state,
    );
}
