use crate::{
    app::{App, AppResult},
    widgets::SelectableList,
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tui_input::{backend::crossterm::EventHandler, Input};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if app.input_mode {
        if key_event.code == KeyCode::Char('q')
            || key_event.code == KeyCode::Esc
            || (key_event.code == KeyCode::Backspace && app.input.value().is_empty())
        {
            app.input = Input::default();
            app.input_mode = false;
        } else if key_event.code == KeyCode::Enter {
            app.input_mode = false;
        } else {
            app.input.handle_event(&Event::Key(key_event));
        }
        let query = app.input.value().to_lowercase();
        app.list = SelectableList::with_items(
            app.cves
                .clone()
                .into_iter()
                .filter(|cve| {
                    app.input.value().is_empty()
                        || cve.cve_data_meta.id.to_lowercase().contains(&query)
                        || cve
                            .description
                            .description_data
                            .iter()
                            .find(|desc| desc.lang == *"en")
                            .map(|v| v.value.to_string())
                            .unwrap_or_default()
                            .contains(&query)
                })
                .collect(),
        );
        return Ok(());
    }
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            if app.show_details {
                app.show_details = false;
            } else {
                app.quit();
            }
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.show_details && app.scroll_details {
                app.scroll_index = app.scroll_index.saturating_add(1);
            } else {
                app.list.next();
                app.show_details = false;
            }
            return Ok(());
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.show_details && app.scroll_details {
                app.scroll_index = app.scroll_index.saturating_sub(1);
            } else {
                app.list.previous();
                app.show_details = false;
            }
            return Ok(());
        }
        KeyCode::Char('/') | KeyCode::Char('s') => {
            app.input_mode = true;
        }
        KeyCode::Backspace => {
            if !app.input.value().is_empty() {
                app.input_mode = true;
                app.input.handle_event(&Event::Key(key_event));
            }
        }
        KeyCode::Enter => {
            app.scroll_index = 0;
            app.show_details = !app.show_details;
        }
        _ => {}
    }
    app.show_details = key_event == KeyCode::Enter.into();
    Ok(())
}
