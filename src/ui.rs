use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Styled, Stylize},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Clear, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Table, TableState,
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
        Layout::vertical([Constraint::Min(1), Constraint::Percentage(100)]).split(frame.size());
    render_header(app, frame, rects[0]);
    render_list(app, frame, rects[1]);
    render_cursor(app, frame, rects[1]);
    render_details(app, frame, rects[1]);
}

fn render_list(app: &mut App, frame: &mut Frame<'_>, area: Rect) {
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
                .top_margin(1)
        })
        .collect::<Vec<Row>>();
    let block = Block::bordered()
        .style(if app.show_details {
            app.theme.dim
        } else {
            app.theme.background
        })
        .border_style(app.theme.borders)
        .border_type(BorderType::Double)
        .title_bottom(
            if items_len != 0 {
                Line::from(vec![
                    "|".set_style(app.theme.separator),
                    format!("{}/{}", selected_index.saturating_add(1), items_len)
                        .set_style(app.theme.index),
                    "|".set_style(app.theme.separator),
                ])
            } else {
                Line::default()
            }
            .right_aligned(),
        )
        .title_bottom(
            Line::from(
                KEY_BINDINGS
                    .iter()
                    .enumerate()
                    .flat_map(|(i, (keys, desc))| {
                        vec![
                            "<".set_style(app.theme.separator),
                            keys.join("-").set_style(app.theme.footer),
                            ": ".set_style(app.theme.separator),
                            Span::from(*desc).set_style(app.theme.footer),
                            ">".set_style(app.theme.separator),
                            if i != KEY_BINDINGS.len() - 1 { " " } else { "" }.into(),
                        ]
                    })
                    .collect::<Vec<Span>>(),
            )
            .centered(),
        )
        .title_bottom(if !app.input.value().is_empty() || app.input_mode {
            Line::from(vec![
                "|".set_style(app.theme.separator),
                "Search: ".set_style(app.theme.highlight).bold(),
                app.input.value().set_style(if items.is_empty() {
                    app.theme.input_empty
                } else {
                    app.theme.input
                }),
                if app.input_mode { " " } else { "" }.into(),
                "|".set_style(app.theme.separator),
            ])
        } else {
            Line::default()
        });
    frame.render_stateful_widget(
        Table::new(items, &[Constraint::Min(13), Constraint::Percentage(100)])
            .header(Row::new(vec![
                "Name".set_style(app.theme.highlight).bold(),
                "Description".set_style(app.theme.highlight).bold(),
            ]))
            .block(block)
            .highlight_style(app.theme.selected.bold()),
        area,
        &mut table_state,
    );
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .style(app.theme.scrollbar)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        area.inner(&Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut ScrollbarState::new(items_len).position(selected_index),
    );
}

fn render_header(app: &mut App, frame: &mut Frame<'_>, area: Rect) {
    let title = Paragraph::new(
        format!(
            " {} - {} ",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_DESCRIPTION")
        )
        .bold(),
    )
    .block(Block::default().style(app.theme.header))
    .alignment(Alignment::Left);
    frame.render_widget(title, area);

    let text = format!("v{} with ♥ by @orhun ", env!("CARGO_PKG_VERSION"));
    let meta = Paragraph::new(text)
        .block(Block::default().style(app.theme.header))
        .alignment(Alignment::Right);
    frame.render_widget(meta, area);
}

/// Renders the cursor.
fn render_cursor(app: &mut App, frame: &mut Frame<'_>, area: Rect) {
    if app.input_mode {
        let (x, y) = (
            area.x
                + Input::default()
                    .with_value(format!("Search: {}", app.input.value()))
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
fn render_details(app: &mut App, frame: &mut Frame<'_>, area: Rect) {
    if let (true, Some(cve)) = (app.show_details, app.list.selected()) {
        let description = cve
            .description
            .clone()
            .unwrap_or_default()
            .trim()
            .to_string();
        let mut lines = vec![vec![
            "Assigner".set_style(app.theme.foreground).bold(),
            ": ".set_style(app.theme.separator),
            cve.assigner.to_string().set_style(app.theme.foreground),
        ]
        .into()];
        let max_row_width = (area.width - 2) / 2;
        if (Line::raw(&description).width() as u16) < max_row_width {
            lines.push(
                vec![
                    "Description".set_style(app.theme.foreground).bold(),
                    ": ".set_style(app.theme.separator),
                    description.set_style(app.theme.foreground),
                ]
                .into(),
            );
        } else {
            lines.push(
                vec![
                    "Description".set_style(app.theme.foreground).bold(),
                    ": ".set_style(app.theme.separator),
                ]
                .into(),
            );
            lines.extend(
                textwrap::wrap(&description, textwrap::Options::new(max_row_width as usize))
                    .into_iter()
                    .map(|v| Line::from(v.to_string()).style(app.theme.foreground))
                    .collect::<Vec<Line>>(),
            );
        }
        for reference in &cve.references {
            let reference_line = vec![
                "Reference".set_style(app.theme.foreground).bold(),
                ": ".set_style(app.theme.separator),
                reference.to_string().set_style(app.theme.foreground),
            ]
            .into();
            lines.push(reference_line);
        }
        if lines.len() > area.height.saturating_sub(2) as usize {
            lines = lines.into_iter().skip(app.scroll_index).collect();
        }
        let popup = Popup::new(
            vec![
                "|".set_style(app.theme.separator),
                cve.id.to_string().set_style(app.theme.highlight).bold(),
                "|".set_style(app.theme.separator),
            ],
            lines.clone(),
        )
        .style(app.theme.background);
        frame.render_widget(popup.to_widget(), area);
        app.scroll_details = lines.len() > area.height.saturating_sub(2) as usize;
        if app.scroll_details {
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .style(app.theme.scrollbar)
                    .begin_symbol(Some("↑"))
                    .end_symbol(Some("↓")),
                area.inner(&Margin {
                    vertical: 1,
                    horizontal: (area.width.saturating_sub(
                        lines.iter().map(|v| v.width()).max().unwrap_or_default() as u16,
                    ) / 2),
                }),
                &mut ScrollbarState::new(lines.len().saturating_sub(area.height as usize) + 2)
                    .position(app.scroll_index),
            );
        }
    }
}
