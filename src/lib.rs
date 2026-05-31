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
mod state;

#[cfg(test)]
mod tests;

pub use config::{
    OverflowPolicy, TabAxis, TabBarEnd, TabDirection, TabMargin, TabOrientation, TabPadding,
    TabWheelDirection,
};
pub use label::vertical_label;
pub use nav::TabNav;
pub use state::TabNavState;

pub(crate) const DEFAULT_INDICATOR: &str = "▸";
pub(crate) const TAB_BORDER: u16 = 1;
