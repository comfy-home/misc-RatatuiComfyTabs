//! Copyright © 2026 ComfyHome™
//! All rights reserved.
//!
//! Licensed under the ComfyGit SA-PS:DA License
//!
//! For details, see the LICENSE file in the repository root.

mod config;
mod label;
mod layout;
mod nav;
mod render;
mod reorder;
mod state;

#[cfg(test)]
mod tests;

pub use config::{
    HorizontalPosition, OverflowPolicy, TabAxis, TabBarEnd, TabDirection, TabMargin,
    TabOrientation, TabPadding, TabReorderPolicy, TabWheelDirection, VerticalPosition,
};
pub use label::vertical_label;
pub use nav::TabNav;
pub use reorder::{TabReorder, remap_selected_index, remap_selected_index_with_pins, try_reorder};
pub use state::{SELECTION_FLASH_SEGMENT, SELECTION_FLASH_TOTAL, TabNavState, TabReorderDrag};

/// Unified border-set names for [`TabNav::border_set`].
///
/// | Name | Ratatui alias | Corner style |
/// |------|---------------|--------------|
/// | [`Rnd`](self::tab_border::Rnd) | `symbols::border::ROUNDED` | Rounded |
/// | [`Sqr`](self::tab_border::Sqr) | `symbols::border::PLAIN` | Square |
pub mod tab_border {
    pub use ratatui_core::symbols::border::PLAIN as Sqr;
    pub use ratatui_core::symbols::border::ROUNDED as Rnd;
}

pub(crate) const DEFAULT_INDICATOR: &str = "▸";
pub(crate) const TAB_BORDER: u16 = 1;
