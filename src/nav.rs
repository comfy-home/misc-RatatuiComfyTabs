//! Copyright © 2026 ComfyHome™
//! All rights reserved.
//!
//! Licensed under the ComfyGit SA-PS:DA License
//!
//! For details, see the LICENSE file in the repository root.

use ratatui_core::layout::{Position, Rect};
use ratatui_core::style::Style;
use ratatui_core::symbols;

use crate::DEFAULT_INDICATOR;
use crate::config::{
    HorizontalPosition, OverflowPolicy, TabBarAlign, TabBarEnd, TabMargin, TabOrientation,
    TabPadding, TabReorderPolicy, VerticalPosition,
};
use crate::layout::{
    auto_horizontal_tab_width, auto_vertical_tab_height, compute_viewport, effective_padding,
    horizontal_strip_height, tab_entry_rect, vertical_rail_width,
};
use crate::reorder::can_drag_index;

/// Tab navigation rendered as individually bordered boxes.
///
/// Adjacent tabs sit flush (no gap). The active tab opens into the adjacent content
/// via rounded junction corners. Inactive tabs use T-junctions so the baseline stays
/// continuous.
///
/// ## Default tab sizing
///
/// Horizontal tab **width** (columns):
///
/// `2 + padding.left + label_display_width + padding.right`
///
/// Vertical tab **height** (rows):
///
/// `2 + padding.top + label_line_count + padding.bottom`
///
/// Label width uses [`unicode_width`](https://docs.rs/unicode-width) display width (wide
/// characters such as CJK count as two columns). Override per-tab sizes with
/// [`tab_widths`](TabNav::tab_widths) or [`tab_heights`](TabNav::tab_heights).
///
/// ## Overflow
///
/// Default [`OverflowPolicy::Scroll`] renders a sliding window driven by
/// [`TabNavState::scroll_offset`]. Scroll overflow is marked with `⯇` / `⯈` (horizontal) or
/// `⯅` / `⯆` (vertical) inside the first/last visible tab. Use [`OverflowPolicy::Truncate`]
/// to omit tabs that do not fit. Truncate mode may show `…` on the baseline when
/// [`overflow_affordance`](TabNav::overflow_affordance) is enabled.
///
/// ## Stateful rendering
///
/// Implement [`StatefulWidget`](ratatui_core::widgets::StatefulWidget) with [`TabNavState`](crate::TabNavState) to keep selection and scroll between frames.
/// Use [`TabNavState::select_direction_visible`](crate::TabNavState::select_direction_visible) or [`TabAxis::direction`](crate::TabAxis::direction) to reduce navigation
/// boilerplate.
///
/// ## Mouse wheel
///
/// When [`mouse_wheel`](Self::mouse_wheel) is enabled (default), call
/// [`TabNavState::handle_mouse_wheel`](crate::TabNavState::handle_mouse_wheel) with the pointer
/// position over the strip to cycle tabs.
///
/// ## Mouse click
///
/// When [`mouse_click`](Self::mouse_click) is enabled (default), call
/// [`TabNavState::handle_mouse_click`](crate::TabNavState::handle_mouse_click) with the pointer
/// position over a visible tab to select it. Use [`tab_index_at`](Self::tab_index_at) for custom
/// hit testing.
///
/// ## Layout helpers
///
/// Use [`tab_rects`](TabNav::tab_rects) for mouse hit targets or adjacent layout without
/// duplicating the sizing math.
///
/// - [`TabOrientation::Horizontal`]: baseline along the bottom ([`HorizontalPosition::Top`]) or
///   top ([`HorizontalPosition::Bottom`]). Indicator defaults to `Some("▸")`. Default
///   [`TabMargin::ZERO`] and [`TabPadding::horizontal_default`].
/// - [`TabOrientation::Vertical`]: baseline along the right edge ([`VerticalPosition::Left`]) or
///   left ([`VerticalPosition::Right`]). Indicator defaults to `None`. Default
///   [`TabMargin::vertical_default`] and [`TabPadding::vertical_default`].
#[must_use]
pub struct TabNav<'a> {
    pub(crate) tabs: &'a [&'a str],
    pub(crate) selected: usize,
    pub(crate) orientation: TabOrientation,
    pub(crate) horizontal_position: HorizontalPosition,
    pub(crate) vertical_position: VerticalPosition,
    pub(crate) tab_bar_align: TabBarAlign,
    pub(crate) margin: Option<TabMargin>,
    pub(crate) padding: Option<TabPadding>,
    pub(crate) tab_bar_end: Option<TabBarEnd>,
    pub(crate) all_caps: bool,
    pub(crate) style: Style,
    pub(crate) highlight_style: Style,
    pub(crate) highlight_bold: bool,
    pub(crate) border_style: Style,
    pub(crate) indicator: Option<&'a str>,
    pub(crate) indicator_explicit: bool,
    pub(crate) border_set: symbols::border::Set<'a>,
    pub(crate) tab_sizes: Option<&'a [u16]>,
    pub(crate) overflow: OverflowPolicy,
    pub(crate) scroll_offset: usize,
    pub(crate) overflow_affordance: bool,
    pub(crate) mouse_wheel: bool,
    pub(crate) mouse_click: bool,
    pub(crate) reorder_policy: TabReorderPolicy,
    pub(crate) tab_pinned: Option<&'a [bool]>,
    pub(crate) mouse_reorder: bool,
    pub(crate) reorder_drag_style: Option<Style>,
    pub(crate) selection_flash_style: Option<Style>,
    pub(crate) selection_flash_enabled: bool,
}

impl<'a> TabNav<'a> {
    /// Creates a new `TabNav` with the given tab labels and selected index.
    pub fn new(tabs: &'a [&'a str], selected: usize) -> Self {
        Self {
            tabs,
            selected,
            orientation: TabOrientation::Horizontal,
            horizontal_position: HorizontalPosition::default(),
            vertical_position: VerticalPosition::default(),
            tab_bar_align: TabBarAlign::default(),
            margin: None,
            padding: None,
            tab_bar_end: None,
            all_caps: false,
            style: Style::new(),
            highlight_style: Style::new(),
            highlight_bold: true,
            border_style: Style::new(),
            indicator: Some(DEFAULT_INDICATOR),
            indicator_explicit: false,
            border_set: crate::tab_border::Rnd,
            tab_sizes: None,
            overflow: OverflowPolicy::Scroll,
            scroll_offset: 0,
            overflow_affordance: true,
            mouse_wheel: true,
            mouse_click: true,
            reorder_policy: TabReorderPolicy::default(),
            tab_pinned: None,
            mouse_reorder: false,
            reorder_drag_style: None,
            selection_flash_style: None,
            selection_flash_enabled: true,
        }
    }

    /// Horizontal strip above content, or vertical rail beside content.
    pub fn orientation(mut self, orientation: TabOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Horizontal strip above ([`HorizontalPosition::Top`]) or below
    /// ([`HorizontalPosition::Bottom`]) adjacent content. Default: [`HorizontalPosition::Top`].
    pub fn horizontal_position(mut self, position: HorizontalPosition) -> Self {
        self.horizontal_position = position;
        self
    }

    /// Vertical rail on the left ([`VerticalPosition::Left`]) or right
    /// ([`VerticalPosition::Right`]) of adjacent content. Default: [`VerticalPosition::Left`].
    pub fn vertical_position(mut self, position: VerticalPosition) -> Self {
        self.vertical_position = position;
        self
    }

    /// Alignment along the strip flow axis within the allocated area.
    /// Default: [`TabBarAlign::Start`] (left in horizontal mode, top in vertical mode).
    pub fn tab_bar_align(mut self, align: TabBarAlign) -> Self {
        self.tab_bar_align = align;
        self
    }

    /// Strip inset along the flow axis. Defaults depend on [`TabOrientation`].
    pub fn margin(mut self, margin: TabMargin) -> Self {
        self.margin = Some(margin);
        self
    }

    /// Interior spacing inside each tab box. Defaults depend on [`TabOrientation`].
    pub fn padding(mut self, padding: TabPadding) -> Self {
        self.padding = Some(padding);
        self
    }

    /// Baseline end-cap style. Default: [`TabBarEnd::NoEnd`].
    pub fn tab_bar_end(mut self, end: TabBarEnd) -> Self {
        self.tab_bar_end = Some(end);
        self
    }

    /// Render tab labels in uppercase. Default: `false`.
    pub fn all_caps(mut self, all_caps: bool) -> Self {
        self.all_caps = all_caps;
        self
    }

    /// Style for inactive tab labels.
    pub fn style(mut self, style: impl Into<Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Style for the active tab label.
    pub fn highlight_style(mut self, style: impl Into<Style>) -> Self {
        self.highlight_style = style.into();
        self
    }

    /// Whether to auto-apply bold to the active tab. Default: `true`.
    pub fn highlight_bold(mut self, bold: bool) -> Self {
        self.highlight_bold = bold;
        self
    }

    /// Style for borders and the baseline.
    pub fn border_style(mut self, style: impl Into<Style>) -> Self {
        self.border_style = style.into();
        self
    }

    /// Symbol shown beside the active tab label.
    pub fn indicator(mut self, indicator: Option<&'a str>) -> Self {
        self.indicator = indicator;
        self.indicator_explicit = true;
        self
    }

    /// Border character set. Default: [`crate::tab_border::Rnd`].
    pub fn border_set(mut self, set: symbols::border::Set<'a>) -> Self {
        self.border_set = set;
        self
    }

    /// Override horizontal tab widths (columns). Missing entries fall back to auto width.
    ///
    /// Has no effect when [`orientation`](Self::orientation) is [`TabOrientation::Vertical`]
    /// unless paired with [`tab_heights`](Self::tab_heights) after switching orientation.
    pub fn tab_widths(mut self, widths: &'a [u16]) -> Self {
        self.tab_sizes = Some(widths);
        self
    }

    /// Override vertical tab heights (rows). Missing entries fall back to auto height.
    pub fn tab_heights(mut self, heights: &'a [u16]) -> Self {
        self.tab_sizes = Some(heights);
        self
    }

    /// Overflow behaviour when tabs exceed strip space. Default: [`OverflowPolicy::Scroll`].
    pub fn overflow(mut self, policy: OverflowPolicy) -> Self {
        self.overflow = policy;
        self
    }

    /// First visible tab index for stateless [`OverflowPolicy::Scroll`] rendering.
    pub fn scroll_offset(mut self, offset: usize) -> Self {
        self.scroll_offset = offset;
        self
    }

    /// Draw `‹` / `›` / `…` at clipped edges when tabs are hidden. Default: `true`.
    pub fn overflow_affordance(mut self, enabled: bool) -> Self {
        self.overflow_affordance = enabled;
        self
    }

    /// Allow mouse wheel tab switching while the pointer is over the strip. Default: `true`.
    pub fn mouse_wheel(mut self, enabled: bool) -> Self {
        self.mouse_wheel = enabled;
        self
    }

    /// Whether mouse wheel tab switching is enabled for this widget.
    pub const fn mouse_wheel_enabled(&self) -> bool {
        self.mouse_wheel
    }

    /// Allow mouse click tab selection on visible tab boxes. Default: `true`.
    pub fn mouse_click(mut self, enabled: bool) -> Self {
        self.mouse_click = enabled;
        self
    }

    /// Whether mouse click tab selection is enabled for this widget.
    pub const fn mouse_click_enabled(&self) -> bool {
        self.mouse_click
    }

    /// Tab reorder policy. Default: [`TabReorderPolicy::AllPinned`] (no drag reorder).
    pub fn reorder_policy(mut self, policy: TabReorderPolicy) -> Self {
        self.reorder_policy = policy;
        self
    }

    /// Per-tab pin flags for [`TabReorderPolicy::SomePinned`] (`true` = fixed slot).
    pub fn tab_pinned(mut self, pinned: &'a [bool]) -> Self {
        self.tab_pinned = Some(pinned);
        self
    }

    /// Enable mouse drag-and-drop reordering (requires a non-[`TabReorderPolicy::AllPinned`] policy).
    pub fn mouse_reorder(mut self, enabled: bool) -> Self {
        self.mouse_reorder = enabled;
        self
    }

    /// Style for the tab being dragged (label and borders). Default: foreground indexed color **46**.
    pub fn reorder_drag_style(mut self, style: Style) -> Self {
        self.reorder_drag_style = Some(style);
        self
    }

    /// Border-only flash when selection changes. Default: foreground indexed color **46**.
    pub fn selection_flash_style(mut self, style: Style) -> Self {
        self.selection_flash_style = Some(style);
        self
    }

    /// Enable or disable the selection border flash. Default: `true`.
    pub fn selection_flash(mut self, enabled: bool) -> Self {
        self.selection_flash_enabled = enabled;
        self
    }

    /// Active reorder policy for this widget.
    pub const fn reorder_policy_value(&self) -> TabReorderPolicy {
        self.reorder_policy
    }

    /// Whether drag reordering is enabled (policy + [`mouse_reorder`](Self::mouse_reorder)).
    pub fn reorder_enabled(&self) -> bool {
        self.mouse_reorder && self.reorder_policy != TabReorderPolicy::AllPinned
    }

    /// Whether the tab at `index` may be dragged.
    pub fn can_drag_tab(&self, index: usize) -> bool {
        can_drag_index(index, self.reorder_policy, self.tab_pinned)
    }

    /// Auto-computed width for tab `index` using the current padding (ignores [`tab_widths`]).
    pub fn auto_tab_width(&self, index: usize) -> Option<u16> {
        let label = self.tabs.get(index)?;
        Some(auto_horizontal_tab_width(
            label,
            effective_padding(self),
            self.all_caps,
        ))
    }

    /// Auto-computed height for tab `index` using the current padding (ignores [`tab_heights`]).
    pub fn auto_tab_height(&self, index: usize) -> Option<u16> {
        let label = self.tabs.get(index)?;
        Some(auto_vertical_tab_height(label, effective_padding(self)))
    }

    /// Layout rectangle for each visible tab (same geometry as rendering).
    ///
    /// Returns one [`Rect`] per visible tab in tab order. Empty when `area` is too small or
    /// there are no tabs. Respects [`overflow`](Self::overflow) and `scroll_offset`.
    pub fn tab_rects(&self, area: Rect) -> Vec<Rect> {
        self.tab_rects_with_scroll(area, self.scroll_offset)
    }

    /// Like [`tab_rects`](Self::tab_rects) but uses an explicit scroll offset (scroll mode).
    pub fn tab_rects_with_scroll(&self, area: Rect, scroll_offset: usize) -> Vec<Rect> {
        if self.tabs.is_empty() {
            return Vec::new();
        }

        compute_viewport(self, area, scroll_offset)
            .entries
            .iter()
            .filter_map(|entry| tab_entry_rect(self, area, entry))
            .collect()
    }

    /// Minimum height for a horizontal tab strip with the current padding.
    pub fn horizontal_strip_height(&self) -> u16 {
        horizontal_strip_height(self)
    }

    /// Width of the vertical tab rail (widest tab) with the current padding.
    pub fn vertical_rail_width(&self) -> u16 {
        vertical_rail_width(self)
    }

    /// Whether a pointer position should receive mouse wheel tab switching.
    ///
    /// True when `position` lies inside `strip_area` or any visible [`tab_rects`](Self::tab_rects).
    pub fn wheel_hover(
        &self,
        strip_area: Rect,
        scroll_offset: usize,
        mouse_column: u16,
        mouse_row: u16,
    ) -> bool {
        let position = Position::new(mouse_column, mouse_row);
        if strip_area.contains(position) {
            return true;
        }

        self.tab_rects_with_scroll(strip_area, scroll_offset)
            .iter()
            .any(|rect| rect.contains(position))
    }

    /// Visible tab index under the pointer, if any.
    ///
    /// Pass the same `area` and `scroll_offset` used for rendering. Returns `None` when the
    /// pointer is outside every visible tab box.
    pub fn tab_index_at(
        &self,
        area: Rect,
        scroll_offset: usize,
        mouse_column: u16,
        mouse_row: u16,
    ) -> Option<usize> {
        if self.tabs.is_empty() {
            return None;
        }

        let position = Position::new(mouse_column, mouse_row);
        compute_viewport(self, area, scroll_offset)
            .entries
            .iter()
            .rev()
            .find_map(|entry| {
                tab_entry_rect(self, area, entry)
                    .filter(|rect| rect.contains(position))
                    .map(|_| entry.index)
            })
    }
}
