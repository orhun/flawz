//! A Terminal UI for browsing CVEs.

#![warn(missing_docs, clippy::unwrap_used)]

/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Application theme.
pub mod theme;

/// Custom widgets.
pub mod widgets;

/// Event handler.
pub mod handler;

/// Error implementation.
pub mod error;

/// Command-line arguments.
pub mod args;

/// CVE wrapper.
pub mod cve;
