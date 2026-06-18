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

/// Whether tabs in the strip may be reordered by drag-and-drop.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabReorderPolicy {
    /// Fixed order (default). Matches legacy behaviour — no drag reordering.
    #[default]
    AllPinned,
    /// Every tab may be dragged to another slot.
    NonePinned,
    /// Per-tab [`TabNav::tab_pinned`] mask: `true` tabs stay fixed; others may move.
    SomePinned,
}

/// End-cap style for the tab strip baseline.
///
/// Cap glyphs depend on strip position and [`TabBarAlign`]:
///
/// - [`TabBarAlign::Start`]: leading/trailing caps at the flow-axis margins; horizontal
///   leading stays `├`/`│` with only the trailing cap flipping for [`HorizontalPosition::Bottom`].
/// - [`TabBarAlign::Center`]: leading cap mirrors the start trailing cap horizontally
///   (`┐`→`┌` / `┘`→`└`, or rounded equivalents); trailing unchanged. Vertical center
///   caps sit on the margin only, not on the first tab.
/// - [`TabBarAlign::End`]: leading/trailing caps swap and mirror relative to [`TabBarAlign::Start`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabBarEnd {
    /// Continuous baseline with no corner caps.
    #[default]
    NoEnd,
    /// Square end caps (see enum docs for position-specific glyphs).
    Sqr,
    /// Rounded end caps (see enum docs for position-specific glyphs).
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

/// Horizontal strip placement relative to adjacent content.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HorizontalPosition {
    /// Strip along the top edge; active tab opens downward (default).
    #[default]
    Top,
    /// Strip along the bottom edge; active tab opens upward.
    Bottom,
}

/// Horizontal strip placement plus an optional cap on simultaneously visible tabs.
///
/// Pass [`HorizontalPosition::Top`] (or [`HorizontalPosition::Bottom`]) directly to
/// [`TabNav::horizontal_position`](crate::TabNav::horizontal_position), or call
/// [`.max(n)`](HorizontalPosition::max) to limit the sliding window:
///
/// ```ignore
/// TabNav::new(&labels, 0)
///     .horizontal_position(HorizontalPosition::Top.max(5));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HorizontalPositionConfig {
    /// Strip edge and open direction.
    pub position: HorizontalPosition,
    /// When set, at most this many tabs are visible; additional tabs require scrolling.
    pub max_visible: Option<usize>,
}

impl HorizontalPosition {
    /// Caps the visible tab window to `max` tabs (scroll mode semantics).
    pub const fn max(self, max: usize) -> HorizontalPositionConfig {
        HorizontalPositionConfig {
            position: self,
            max_visible: Some(max),
        }
    }
}

impl From<HorizontalPosition> for HorizontalPositionConfig {
    fn from(position: HorizontalPosition) -> Self {
        Self {
            position,
            max_visible: None,
        }
    }
}

impl HorizontalPositionConfig {
    /// Strip edge without an explicit max (same as [`From<HorizontalPosition>`]).
    pub const fn new(position: HorizontalPosition) -> Self {
        Self {
            position,
            max_visible: None,
        }
    }

    /// Returns the strip edge.
    pub const fn position(self) -> HorizontalPosition {
        self.position
    }

    /// Returns the configured visible-tab cap, if any.
    pub const fn max_visible(self) -> Option<usize> {
        self.max_visible
    }
}

/// Tab strip alignment within its allocated flow axis.
///
/// Horizontal mode: [`Start`](Self::Start) = left, [`Center`](Self::Center) = centred,
/// [`End`](Self::End) = right.
///
/// Vertical mode: [`Start`](Self::Start) = top, [`Center`](Self::Center) = centred,
/// [`End`](Self::End) = bottom.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabBarAlign {
    /// Leading edge of the flow axis (left / top).
    #[default]
    Start,
    /// Centred when the strip is narrower than the allocated area.
    Center,
    /// Trailing edge of the flow axis (right / bottom).
    End,
}

/// Vertical rail placement relative to adjacent content.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VerticalPosition {
    /// Rail along the left edge; active tab opens right (default).
    #[default]
    Left,
    /// Rail along the right edge; active tab opens left.
    Right,
}

/// Vertical rail placement plus an optional cap on simultaneously visible tabs.
///
/// Pass [`VerticalPosition::Left`] (or [`VerticalPosition::Right`]) directly to
/// [`TabNav::vertical_position`](crate::TabNav::vertical_position), or call
/// [`.max(n)`](VerticalPosition::max) to limit the sliding window:
///
/// ```ignore
/// TabNav::new(&labels, 0)
///     .orientation(TabOrientation::Vertical)
///     .vertical_position(VerticalPosition::Left.max(2));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VerticalPositionConfig {
    /// Rail edge and open direction.
    pub position: VerticalPosition,
    /// When set, at most this many tabs are visible; additional tabs require scrolling.
    pub max_visible: Option<usize>,
}

impl VerticalPosition {
    /// Caps the visible tab window to `max` tabs (scroll mode semantics).
    pub const fn max(self, max: usize) -> VerticalPositionConfig {
        VerticalPositionConfig {
            position: self,
            max_visible: Some(max),
        }
    }
}

impl From<VerticalPosition> for VerticalPositionConfig {
    fn from(position: VerticalPosition) -> Self {
        Self {
            position,
            max_visible: None,
        }
    }
}

impl VerticalPositionConfig {
    /// Rail edge without an explicit max (same as [`From<VerticalPosition>`]).
    pub const fn new(position: VerticalPosition) -> Self {
        Self {
            position,
            max_visible: None,
        }
    }

    /// Returns the rail edge.
    pub const fn position(self) -> VerticalPosition {
        self.position
    }

    /// Returns the configured visible-tab cap, if any.
    pub const fn max_visible(self) -> Option<usize> {
        self.max_visible
    }
}

/// Behaviour when tabs exceed available strip space.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OverflowPolicy {
    /// Show tabs from the start until space runs out. Hidden tabs are omitted.
    Truncate,
    /// Render a sliding window. Use [`TabNavState::scroll_offset`] (or
    /// [`TabNav::scroll_offset`] for stateless rendering) as the index of the first visible tab.
    #[default]
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
