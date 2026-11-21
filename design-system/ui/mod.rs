//! Design system for the Wiloprem egui desktop client.
//!
//! This module is compiled into the `wiloprem` crate through a `#[path]` attribute in
//! `src/gui.rs`, so tokens, palettes and widgets live next to the design documentation
//! (`design-system/zalo-rust-desktop/MASTER.md`) and nothing in `src/gui/` hardcodes
//! colors, spacing or radii.
//!
//! Layering rules:
//! - `tokens` and `typography` are plain constants, free of any egui state.
//! - `palette` maps tokens to the two themes (Discord-style dark and light).
//! - `widgets` paint stateless controls from a `ThemePalette` and return an `egui::Response`;
//!   they never touch application state, the worker channel or SQLite.

pub(crate) mod icons;
pub(crate) mod palette;
pub(crate) mod tokens;
pub(crate) mod typography;
pub(crate) mod widgets;
