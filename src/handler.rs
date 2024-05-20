use crate::app::{App, AppResult};
use crate::event::Event as TuiEvent;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use std::sync::mpsc::Sender;
use tui_input::{backend::crossterm::EventHandler, Input};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App,
    sender: &Sender<TuiEvent>,
) -> AppResult<()> {
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
        sender.send(TuiEvent::Search)?;
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
        KeyCode::Char(' ') => {
            if app.show_details {
                let references = &app.list.selected().unwrap().references;
                // Iterates over the references and find the first http reference, if any
                if let Some(first_http_reference) =
                    references.iter().find(|r| r.starts_with("http"))
                {
                    match webbrowser::open(first_http_reference) {
                        Ok(_) => {} // Opened the browser successfully, nothing to do
                        Err(err) => {
                            println!("Failed to open browser: {err:?}");
                        }
                    }
                }
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

/// Handles the mouse events and updates the state of [`App`].
pub fn handle_mouse_events(
    mouse_event: MouseEvent,
    app: &mut App,
    _: &Sender<TuiEvent>,
) -> AppResult<()> {
    match mouse_event.kind {
        MouseEventKind::ScrollDown => {
            if app.show_details && app.scroll_details {
                app.scroll_index = app.scroll_index.saturating_add(1);
            } else {
                app.list.next();
                app.show_details = false;
            }
        }
        MouseEventKind::ScrollUp => {
            if app.show_details && app.scroll_details {
                app.scroll_index = app.scroll_index.saturating_sub(1);
            } else {
                app.list.previous();
                app.show_details = false;
            }
        }
        _ => {}
    }
    Ok(())
}
