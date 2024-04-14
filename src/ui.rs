use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block, Clear, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table,
        TableState,
    },
    Frame,
};
use tui_input::Input;
use tui_popup::Popup;

/// Maximum number of elements to show in the table.
const TABLE_PAGE_LIMIT: usize = 50;

/// Key bindings.
const KEY_BINDINGS: &[(&[&str], &str)] = &[
    (&["Enter"], "Details"),
    (&["s", "/"], "Search"),
    (&["↕", "j/k"], "Next/Prev"),
    (&["q"], "Quit"),
];

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let rects =
        Layout::vertical([Constraint::Percentage(100), Constraint::Min(1)]).split(frame.size());
    frame.render_widget(
        Paragraph::new(
            Line::from(
                KEY_BINDINGS
                    .iter()
                    .flat_map(|(keys, desc)| {
                        vec![
                            "<".fg(Color::Rgb(100, 100, 100)),
                            keys.join("-").green(),
                            ": ".fg(Color::Rgb(100, 100, 100)),
                            Span::from(*desc),
                            "> ".fg(Color::Rgb(100, 100, 100)),
                        ]
                    })
                    .collect::<Vec<Span>>(),
            )
            .alignment(Alignment::Center),
        ),
        rects[1],
    );
    let area = rects[0];
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
            let description = match &cve.description {
                Some(v) => textwrap::wrap(
                    v,
                    textwrap::Options::new(area.width.saturating_sub(15) as usize),
                )
                .join("\n"),
                None => "No description available.".into(),
            };
            Row::new(vec![cve.id.to_string(), description])
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
            .header(Row::new(vec![
                "Name".white().bold(),
                "Description".white().bold(),
            ]))
            .block(block)
            .highlight_style(Style::default().fg(Color::Green)),
        area,
        &mut table_state,
    );
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        area.inner(&Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut ScrollbarState::new(items_len).position(selected_index),
    );
    render_cursor(app, area, frame);
    render_details(app, area, frame);
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

/// Render the details popup.
fn render_details(app: &mut App, area: Rect, frame: &mut Frame<'_>) {
    if let (true, Some(cve)) = (app.show_details, app.list.selected()) {
        let description = cve
            .description
            .clone()
            .unwrap_or_default()
            .trim()
            .to_string();
        let mut lines = vec![
            vec![
                "ID".white().bold(),
                ": ".fg(Color::Rgb(100, 100, 100)),
                cve.id.to_string().into(),
            ]
            .into(),
            vec![
                "Assigner".white().bold(),
                ": ".fg(Color::Rgb(100, 100, 100)),
                cve.assigner.to_string().into(),
            ]
            .into(),
        ];
        let max_row_width = (area.width - 2) / 2;
        if (Line::raw(&description).width() as u16) < max_row_width {
            lines.push(
                vec![
                    "Description".white().bold(),
                    ": ".fg(Color::Rgb(100, 100, 100)),
                    description.into(),
                ]
                .into(),
            );
        } else {
            lines.push(
                vec![
                    "Description".white().bold(),
                    ": ".fg(Color::Rgb(100, 100, 100)),
                ]
                .into(),
            );
            lines.extend(
                textwrap::wrap(&description, textwrap::Options::new(max_row_width as usize))
                    .into_iter()
                    .map(|v| v.to_string().into())
                    .collect::<Vec<Line>>(),
            );
        }
        for reference in &cve.references {
            let reference_line = vec![
                "Reference".white().bold(),
                ": ".fg(Color::Rgb(100, 100, 100)),
                reference.to_string().into(),
            ]
            .into();
            lines.push(reference_line);
        }
        if lines.len() > area.height.saturating_sub(2) as usize {
            lines = lines.into_iter().skip(app.scroll_index).collect();
        }
        let popup = Popup::new(
            vec![
                "|".fg(Color::Rgb(100, 100, 100)),
                "Details".white().bold(),
                "|".fg(Color::Rgb(100, 100, 100)),
            ],
            lines.clone(),
        );
        frame.render_widget(popup.to_widget(), area);
        app.scroll_details = lines.len() > area.height.saturating_sub(2) as usize;
        if app.scroll_details {
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("↑"))
                    .end_symbol(Some("↓")),
                area.inner(&Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                &mut ScrollbarState::new(lines.len().saturating_sub(area.height as usize) + 2)
                    .position(app.scroll_index),
            );
        }
    }
}
