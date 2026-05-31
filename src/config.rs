//! Copyright © 2026 ComfyHome™
//! All rights reserved.
//!
//! Licensed under the ComfyGit SA-PS:DA License
//!
//! For details, see the LICENSE file in the repository root.

/// Inset for the tab strip within its render area.
///
/// Values are applied along the strip's **flow axis** (the axis tabs advance on):
///
/// - **Horizontal** tabs: [`start`](Self::start) = left margin, [`end`](Self::end) = right margin
///   (columns). Default: [`TabMargin::ZERO`].
/// - **Vertical** tabs: `start` = top margin, `end` = bottom margin (rows). Default:
///   [`TabMargin::vertical_default`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TabMargin {
    pub start: u16,
    pub end: u16,
}

impl TabMargin {
    /// No inset: `margin: 0 0`.
    pub const ZERO: Self = Self { start: 0, end: 0 };

    /// Horizontal strip inset: `margin: <left> <right>` (columns).
    pub const fn horizontal(left: u16, right: u16) -> Self {
        Self {
            start: left,
            end: right,
        }
    }

    /// Vertical strip inset: `margin: <top> <bottom>` (rows).
    pub const fn vertical(top: u16, bottom: u16) -> Self {
        Self {
            start: top,
            end: bottom,
        }
    }

    /// Default vertical inset: `margin: 0 0` (same as [`TabMargin::ZERO`]).
    pub const fn vertical_default() -> Self {
        Self::ZERO
    }
}

/// End-cap style for the tab strip baseline.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabBarEnd {
    /// Continuous baseline with no corner caps.
    #[default]
    NoEnd,
    /// Square caps: horizontal `├`/`┐`; vertical top junction `┬`/`─` and bottom `└`.
    Sqr,
    /// Rounded caps: horizontal `├`/`╮`; vertical top junction `┬`/`─` and bottom `╰`.
    Rnd,
}

/// Interior spacing inside each tab box.
///
/// CSS-like `padding: top bottom left right` where **top/bottom** are rows and **left/right**
/// are columns.
///
/// Defaults depend on orientation — see [`TabPadding::horizontal_default`] and
/// [`TabPadding::vertical_default`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TabPadding {
    pub top: u16,
    pub bottom: u16,
    pub left: u16,
    pub right: u16,
}

impl TabPadding {
    /// `padding: top bottom left right`.
    pub const fn new(top: u16, bottom: u16, left: u16, right: u16) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    /// Horizontal default: `padding: 0 0 3 3`.
    pub const fn horizontal_default() -> Self {
        Self::new(0, 0, 3, 3)
    }

    /// Vertical default: `padding: 1 1 1 1`.
    pub const fn vertical_default() -> Self {
        Self::new(1, 1, 1, 1)
    }

    /// Equal padding on all sides.
    pub const fn uniform(value: u16) -> Self {
        Self::new(value, value, value, value)
    }

    /// CSS-like two-value padding: `padding: <vertical> <horizontal>` (top/bottom, then left/right).
    pub const fn axes(vertical: u16, horizontal: u16) -> Self {
        Self::new(vertical, vertical, horizontal, horizontal)
    }
}

/// Tab strip layout orientation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabOrientation {
    /// Tabs in a row above content.
    #[default]
    Horizontal,
    /// Tabs in a column beside content (left rail).
    Vertical,
}

/// Behaviour when tabs exceed available strip space.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OverflowPolicy {
    /// Show tabs from the start until space runs out. Hidden tabs are omitted.
    #[default]
    Truncate,
    /// Render a sliding window. Use [`TabNavState::scroll_offset`] (or
    /// [`TabNav::scroll_offset`] for stateless rendering) as the index of the first visible tab.
    Scroll,
}

/// Primary-axis navigation step for keyboard or mouse wheel handlers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabDirection {
    /// Previous tab (left in horizontal mode, up in vertical mode).
    Previous,
    /// Next tab (right in horizontal mode, down in vertical mode).
    Next,
}

/// Physical axis input mapped to [`TabDirection`] by orientation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabAxis {
    /// Left arrow / `h` (horizontal) or up arrow / `k` (vertical).
    Decrease,
    /// Right arrow / `l` (horizontal) or down arrow / `j` (vertical).
    Increase,
}

impl TabAxis {
    /// Maps a decrease/increase axis to previous/next tab selection.
    pub const fn direction(self) -> TabDirection {
        match self {
            Self::Decrease => TabDirection::Previous,
            Self::Increase => TabDirection::Next,
        }
    }
}

/// Mouse wheel step for [`TabNavState::handle_mouse_wheel`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabWheelDirection {
    /// Scroll wheel toward the user / up (previous tab).
    Up,
    /// Scroll wheel away from the user / down (next tab).
    Down,
}

impl TabWheelDirection {
    pub(crate) const fn tab_direction(self) -> TabDirection {
        match self {
            Self::Up => TabDirection::Previous,
            Self::Down => TabDirection::Next,
        }
    }

    /// Pick the scroll axis for tab switching.
    ///
    /// Horizontal strips prefer touchpad left/right (`horizontal`) then vertical wheel.
    /// Vertical strips prefer vertical wheel then horizontal.
    pub fn from_axes(
        vertical: Option<Self>,
        horizontal: Option<Self>,
        orientation: TabOrientation,
    ) -> Option<Self> {
        match orientation {
            TabOrientation::Horizontal => horizontal.or(vertical),
            TabOrientation::Vertical => vertical.or(horizontal),
        }
    }
}
