//! Copyright © 2026 ComfyHome™
//! All rights reserved.
//!
//! Licensed under the ComfyGit SA-PS License
//!
//! For details, see the LICENSE file in the repository root.

use ratatui_core::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    symbols,
    widgets::{StatefulWidget, Widget},
};
use unicode_width::UnicodeWidthChar;

const DEFAULT_INDICATOR: &str = "▸";
const TAB_BORDER: u16 = 1;

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
    Angl,
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

/// Mutable tab selection and scroll state for [`StatefulWidget`] rendering.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TabNavState {
    /// Index of the highlighted tab.
    pub selected: usize,
    /// Index of the first visible tab when [`OverflowPolicy::Scroll`] is active.
    pub scroll_offset: usize,
}

impl TabNavState {
    /// Creates state with `selected` and `scroll_offset` at zero.
    pub const fn new(selected: usize) -> Self {
        Self {
            selected,
            scroll_offset: 0,
        }
    }

    /// Sets the selected tab, clamped to `tab_count`.
    pub fn select(&mut self, index: usize, tab_count: usize) {
        if tab_count == 0 {
            self.selected = 0;
            self.scroll_offset = 0;
            return;
        }
        self.selected = index.min(tab_count - 1);
    }

    /// Moves selection along the strip's primary axis.
    pub fn select_direction(&mut self, direction: TabDirection, tab_count: usize) {
        if tab_count == 0 {
            return;
        }
        self.selected = match direction {
            TabDirection::Previous => self.selected.saturating_sub(1),
            TabDirection::Next => (self.selected + 1).min(tab_count - 1),
        };
    }

    /// Wraps selection at the ends of the tab list.
    pub fn select_direction_wrapping(&mut self, direction: TabDirection, tab_count: usize) {
        if tab_count == 0 {
            return;
        }
        self.selected = match direction {
            TabDirection::Previous => (self.selected + tab_count - 1) % tab_count,
            TabDirection::Next => (self.selected + 1) % tab_count,
        };
    }

    /// Scrolls the window one tab toward the start (scroll mode only).
    pub fn scroll_prev(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    /// Scrolls the window one tab toward the end when more tabs are hidden.
    pub fn scroll_next(&mut self, nav: &TabNav<'_>, area: Rect) {
        if nav.overflow != OverflowPolicy::Scroll {
            return;
        }
        let viewport = compute_viewport(nav, area, self.scroll_offset);
        if viewport.clipped_after {
            self.scroll_offset += 1;
        }
    }

    /// Adjusts [`scroll_offset`](Self::scroll_offset) so [`selected`](Self::selected) is visible.
    pub fn ensure_selected_visible(&mut self, nav: &TabNav<'_>, area: Rect) {
        if nav.tabs.is_empty() || nav.overflow != OverflowPolicy::Scroll {
            return;
        }

        if compute_viewport(nav, area, self.scroll_offset)
            .entries
            .iter()
            .any(|entry| entry.index == self.selected)
        {
            return;
        }

        if self.selected < self.scroll_offset {
            self.scroll_offset = self.selected;
            return;
        }

        while self.scroll_offset < nav.tabs.len() {
            if compute_viewport(nav, area, self.scroll_offset)
                .entries
                .iter()
                .any(|entry| entry.index == self.selected)
            {
                break;
            }
            self.scroll_offset += 1;
        }
    }

    /// Selects along the primary axis and keeps the selection visible in scroll mode.
    pub fn select_direction_visible(
        &mut self,
        direction: TabDirection,
        nav: &TabNav<'_>,
        area: Rect,
    ) {
        self.select_direction(direction, nav.tabs.len());
        self.ensure_selected_visible(nav, area);
    }

    /// Wraps selection and keeps it visible in scroll mode.
    pub fn select_direction_wrapping_visible(
        &mut self,
        direction: TabDirection,
        nav: &TabNav<'_>,
        area: Rect,
    ) {
        self.select_direction_wrapping(direction, nav.tabs.len());
        self.ensure_selected_visible(nav, area);
    }
}

/// Converts a single-line label into a vertical stack of characters (one per row).
///
/// ```
/// use ratatui_comfy_tabs::vertical_label;
///
/// assert_eq!(vertical_label("Hi"), "H\ni");
/// ```
pub fn vertical_label(text: &str) -> String {
    text.chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

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
/// Default [`OverflowPolicy::Truncate`] omits tabs that do not fit. [`OverflowPolicy::Scroll`]
/// renders a sliding window driven by [`TabNavState::scroll_offset`]. Optional edge affordances
/// (`‹` / `›` / `…`) mark clipped tabs when [`overflow_affordance`](TabNav::overflow_affordance)
/// is enabled.
///
/// ## Stateful rendering
///
/// Implement [`StatefulWidget`] with [`TabNavState`] to keep selection and scroll between frames.
/// Use [`TabNavState::select_direction_visible`] or [`TabAxis::direction`] to reduce navigation
/// boilerplate.
///
/// ## Layout helpers
///
/// Use [`tab_rects`](TabNav::tab_rects) for mouse hit targets or adjacent layout without
/// duplicating the sizing math.
///
/// - [`TabOrientation::Horizontal`]: baseline along the bottom. Indicator defaults to
///   `Some("▸")`. Default [`TabMargin::ZERO`] and [`TabPadding::horizontal_default`].
/// - [`TabOrientation::Vertical`]: baseline along the right edge. Indicator defaults to
///   `None`. Default [`TabMargin::vertical_default`] and [`TabPadding::vertical_default`].
#[must_use]
pub struct TabNav<'a> {
    tabs: &'a [&'a str],
    selected: usize,
    orientation: TabOrientation,
    margin: Option<TabMargin>,
    padding: Option<TabPadding>,
    tab_bar_end: Option<TabBarEnd>,
    all_caps: bool,
    style: Style,
    highlight_style: Style,
    highlight_bold: bool,
    border_style: Style,
    indicator: Option<&'a str>,
    indicator_explicit: bool,
    border_set: symbols::border::Set<'a>,
    /// Primary-axis size per tab (width when horizontal, height when vertical).
    tab_sizes: Option<&'a [u16]>,
    overflow: OverflowPolicy,
    scroll_offset: usize,
    overflow_affordance: bool,
}

impl<'a> TabNav<'a> {
    /// Creates a new `TabNav` with the given tab labels and selected index.
    pub fn new(tabs: &'a [&'a str], selected: usize) -> Self {
        Self {
            tabs,
            selected,
            orientation: TabOrientation::Horizontal,
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
            border_set: symbols::border::ROUNDED,
            tab_sizes: None,
            overflow: OverflowPolicy::Truncate,
            scroll_offset: 0,
            overflow_affordance: true,
        }
    }

    /// Horizontal strip above content, or vertical rail beside content.
    pub fn orientation(mut self, orientation: TabOrientation) -> Self {
        self.orientation = orientation;
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

    /// Border character set. Default: [`symbols::border::ROUNDED`].
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

    /// Overflow behaviour when tabs exceed strip space. Default: [`OverflowPolicy::Truncate`].
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

        let margin = effective_margin(self);
        let pad = effective_padding(self);

        match self.orientation {
            TabOrientation::Horizontal => {
                let strip_height = self.horizontal_strip_height();
                if area.height < strip_height || area.width <= margin.start + margin.end {
                    return Vec::new();
                }
                compute_viewport(self, area, scroll_offset)
                    .entries
                    .into_iter()
                    .map(|entry| Rect {
                        x: entry.offset,
                        y: area.y,
                        width: entry.size,
                        height: strip_height,
                    })
                    .collect()
            }
            TabOrientation::Vertical => {
                let rail_width = self.vertical_rail_width().min(area.width);
                if rail_width < TAB_BORDER * 2 + pad.left + pad.right
                    || area.height <= margin.start + margin.end
                {
                    return Vec::new();
                }
                compute_viewport(self, area, scroll_offset)
                    .entries
                    .into_iter()
                    .map(|entry| Rect {
                        x: area.x,
                        y: entry.offset,
                        width: rail_width,
                        height: entry.size,
                    })
                    .collect()
            }
        }
    }

    /// Minimum height for a horizontal tab strip with the current padding.
    pub fn horizontal_strip_height(&self) -> u16 {
        let pad = effective_padding(self);
        TAB_BORDER * 2 + pad.top + 1 + pad.bottom
    }

    /// Width of the vertical tab rail (widest tab) with the current padding.
    pub fn vertical_rail_width(&self) -> u16 {
        let pad = effective_padding(self);
        self.tabs
            .iter()
            .map(|label| auto_horizontal_tab_width(label, pad, self.all_caps))
            .max()
            .unwrap_or_else(|| auto_horizontal_tab_width("", pad, self.all_caps))
    }
}

fn effective_margin(nav: &TabNav<'_>) -> TabMargin {
    nav.margin.unwrap_or(match nav.orientation {
        TabOrientation::Horizontal => TabMargin::ZERO,
        TabOrientation::Vertical => TabMargin::vertical_default(),
    })
}

fn effective_padding(nav: &TabNav<'_>) -> TabPadding {
    nav.padding.unwrap_or(match nav.orientation {
        TabOrientation::Horizontal => TabPadding::horizontal_default(),
        TabOrientation::Vertical => TabPadding::vertical_default(),
    })
}

fn effective_tab_bar_end(nav: &TabNav<'_>) -> TabBarEnd {
    nav.tab_bar_end.unwrap_or(TabBarEnd::NoEnd)
}

fn label_line_count(label: &str) -> u16 {
    if label.is_empty() {
        0
    } else {
        label.lines().count() as u16
    }
}

fn char_display_width(ch: char, all_caps: bool) -> u16 {
    label_char(ch, all_caps).width().unwrap_or(0) as u16
}

fn label_display_width(label: &str, all_caps: bool) -> u16 {
    label
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| char_display_width(ch, all_caps))
                .sum::<u16>()
        })
        .max()
        .unwrap_or(0)
}

fn auto_horizontal_tab_width(label: &str, pad: TabPadding, all_caps: bool) -> u16 {
    TAB_BORDER * 2 + pad.left + label_display_width(label, all_caps) + pad.right
}

fn auto_vertical_tab_height(label: &str, pad: TabPadding) -> u16 {
    TAB_BORDER * 2 + pad.top + label_line_count(label) + pad.bottom
}

fn primary_tab_size(nav: &TabNav<'_>, index: usize, label: &str, pad: TabPadding) -> u16 {
    nav.tab_sizes
        .and_then(|sizes| sizes.get(index).copied())
        .unwrap_or_else(|| match nav.orientation {
            TabOrientation::Horizontal => auto_horizontal_tab_width(label, pad, nav.all_caps),
            TabOrientation::Vertical => auto_vertical_tab_height(label, pad),
        })
}

struct TabEntry {
    index: usize,
    offset: u16,
    size: u16,
}

struct TabViewport {
    entries: Vec<TabEntry>,
    clipped_before: bool,
    clipped_after: bool,
    before_affordance_at: Option<u16>,
    after_affordance_at: Option<u16>,
}

fn flow_bounds(nav: &TabNav<'_>, area: Rect) -> Option<(u16, u16)> {
    if nav.tabs.is_empty() {
        return None;
    }

    let margin = effective_margin(nav);
    match nav.orientation {
        TabOrientation::Horizontal => {
            let strip_height = nav.horizontal_strip_height();
            if area.height < strip_height || area.width <= margin.start + margin.end {
                return None;
            }
            Some((area.x + margin.start, area.right() - margin.end))
        }
        TabOrientation::Vertical => {
            let rail_width = nav.vertical_rail_width().min(area.width);
            let pad = effective_padding(nav);
            if rail_width < TAB_BORDER * 2 + pad.left + pad.right
                || area.height <= margin.start + margin.end
            {
                return None;
            }
            Some((area.y + margin.start, area.bottom() - margin.end))
        }
    }
}

fn compute_viewport(nav: &TabNav<'_>, area: Rect, scroll_offset: usize) -> TabViewport {
    let pad = effective_padding(nav);
    let Some((flow_start, flow_end)) = flow_bounds(nav, area) else {
        return TabViewport {
            entries: Vec::new(),
            clipped_before: false,
            clipped_after: false,
            before_affordance_at: None,
            after_affordance_at: None,
        };
    };

    let total = nav.tabs.len();
    let mut first_index = 0usize;
    let mut clipped_before = false;
    let mut content_start = flow_start;

    if nav.overflow == OverflowPolicy::Scroll {
        first_index = scroll_offset.min(total.saturating_sub(1));
        clipped_before = first_index > 0;
        if clipped_before && nav.overflow_affordance {
            content_start = content_start.saturating_add(1);
        }
    }

    let mut entries = Vec::with_capacity(total);
    let mut pos = content_start;

    for index in first_index..total {
        let size = primary_tab_size(nav, index, nav.tabs[index], pad);
        let has_more = index + 1 < total;

        if pos.saturating_add(size) > flow_end {
            break;
        }

        if nav.overflow == OverflowPolicy::Scroll
            && has_more
            && nav.overflow_affordance
            && pos.saturating_add(size).saturating_add(1) > flow_end
        {
            break;
        }

        entries.push(TabEntry {
            index,
            offset: pos,
            size,
        });
        pos = pos.saturating_add(size);
    }

    let clipped_after = entries.last().is_some_and(|entry| entry.index + 1 < total);

    let before_affordance_at = (clipped_before && nav.overflow_affordance).then_some(flow_start);
    let after_affordance_at = if clipped_after && nav.overflow_affordance {
        Some(flow_end.saturating_sub(1))
    } else {
        None
    };

    TabViewport {
        entries,
        clipped_before,
        clipped_after,
        before_affordance_at,
        after_affordance_at,
    }
}

fn effective_indicator<'a>(nav: &TabNav<'a>) -> Option<&'a str> {
    if nav.indicator_explicit {
        nav.indicator
    } else if nav.orientation == TabOrientation::Vertical {
        None
    } else {
        Some(DEFAULT_INDICATOR)
    }
}

fn label_origin(left: u16, top: u16, pad: TabPadding) -> (u16, u16) {
    (left + TAB_BORDER + pad.left, top + TAB_BORDER + pad.top)
}

impl Widget for TabNav<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let selected = self.selected;
        let scroll_offset = self.scroll_offset;
        render_tab_nav(&self, area, buf, selected, scroll_offset);
    }
}

impl StatefulWidget for TabNav<'_> {
    type State = TabNavState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        render_tab_nav(&self, area, buf, state.selected, state.scroll_offset);
    }
}

fn render_tab_nav(
    nav: &TabNav<'_>,
    area: Rect,
    buf: &mut Buffer,
    selected: usize,
    scroll_offset: usize,
) {
    if nav.tabs.is_empty() {
        return;
    }

    match nav.orientation {
        TabOrientation::Horizontal => render_horizontal(nav, area, buf, selected, scroll_offset),
        TabOrientation::Vertical => render_vertical(nav, area, buf, selected, scroll_offset),
    }
}

fn render_horizontal(
    nav: &TabNav<'_>,
    area: Rect,
    buf: &mut Buffer,
    selected: usize,
    scroll_offset: usize,
) {
    let margin = effective_margin(nav);
    let pad = effective_padding(nav);
    let strip_height = nav.horizontal_strip_height();

    if area.height < strip_height || area.width <= margin.start + margin.end {
        return;
    }

    let border = &nav.border_set;
    let bs = nav.border_style;

    let content_left = area.x + margin.start;
    let content_right = area.right() - margin.end;
    let top_y = area.y;
    let bot_y = area.y + strip_height - 1;
    let label_y = area.y + TAB_BORDER + pad.top;

    draw_horizontal_baseline(content_left, content_right, bot_y, border, bs, buf);

    let viewport = compute_viewport(nav, area, scroll_offset);

    for entry in &viewport.entries {
        let label = nav.tabs[entry.index];
        let active = entry.index == selected;
        let left_x = entry.offset;
        let right_x = entry.offset + entry.size - 1;
        let text_style = tab_text_style(nav, active);

        draw_top_border(left_x, right_x, top_y, border, bs, buf);
        draw_horizontal_side_borders(left_x, right_x, top_y, bot_y, border, bs, buf);
        draw_horizontal_label(
            left_x,
            right_x,
            label_y,
            label,
            pad,
            nav.all_caps,
            text_style,
            buf,
        );

        if active {
            if let Some(sym) = effective_indicator(nav) {
                let indicator_x = if pad.left > 0 {
                    left_x + TAB_BORDER + pad.left - 1
                } else {
                    left_x + TAB_BORDER
                };
                buf[(indicator_x, label_y)]
                    .set_symbol(sym)
                    .set_style(text_style);
            }
            draw_active_bottom(left_x, right_x, bot_y, border, bs, buf);
        } else {
            draw_inactive_horizontal_bottom(left_x, right_x, bot_y, bs, buf);
        }
    }

    draw_horizontal_overflow_affordances(&viewport, bot_y, bs, buf);

    apply_horizontal_tab_bar_end(
        content_left,
        content_right,
        bot_y,
        effective_tab_bar_end(nav),
        bs,
        buf,
    );
}

fn render_vertical(
    nav: &TabNav<'_>,
    area: Rect,
    buf: &mut Buffer,
    selected: usize,
    scroll_offset: usize,
) {
    let margin = effective_margin(nav);
    let pad = effective_padding(nav);
    let rail_width = nav.vertical_rail_width().min(area.width);

    if rail_width < TAB_BORDER * 2 + pad.left + pad.right
        || area.height <= margin.start + margin.end
    {
        return;
    }

    let border = &nav.border_set;
    let bs = nav.border_style;
    let left_x = area.x;
    let right_x = area.x + rail_width - 1;
    let content_top = area.y + margin.start;
    let content_bottom = area.bottom() - margin.end;

    draw_vertical_baseline(
        left_x,
        right_x,
        content_top,
        content_bottom,
        border,
        bs,
        buf,
    );

    let positions = compute_viewport(nav, area, scroll_offset);
    let mut first_rendered: Option<(usize, u16)> = None;

    for entry in &positions.entries {
        let label = nav.tabs[entry.index];
        if first_rendered.is_none() {
            first_rendered = Some((entry.index, entry.offset));
        }
        let active = entry.index == selected;
        let top_y = entry.offset;
        let bot_y = entry.offset + entry.size - 1;
        let text_style = tab_text_style(nav, active);

        draw_top_border(left_x, right_x, top_y, border, bs, buf);
        draw_vertical_side_borders(left_x, right_x, top_y, bot_y, border, bs, buf);
        draw_vertical_label(
            left_x,
            right_x,
            top_y,
            label,
            pad,
            nav.all_caps,
            text_style,
            buf,
        );
        draw_bottom_border(left_x, right_x, bot_y, border, bs, buf);

        if active {
            if let Some(sym) = effective_indicator(nav) {
                let (label_x, label_y) = label_origin(left_x, top_y, pad);
                if label_y > top_y + TAB_BORDER {
                    buf[(label_x, label_y - 1)]
                        .set_symbol(sym)
                        .set_style(text_style);
                }
            }
            draw_active_right(left_x, right_x, top_y, bot_y, border, bs, buf);
        } else {
            draw_inactive_vertical_right(left_x, right_x, top_y, bot_y, bs, buf);
        }
    }

    draw_vertical_overflow_affordances(&positions, right_x, bs, buf);

    if let Some((first_index, first_top)) = first_rendered {
        apply_vertical_tab_bar_end(
            first_index == selected,
            first_top,
            right_x,
            content_bottom,
            effective_tab_bar_end(nav),
            bs,
            buf,
        );
    }
}

fn draw_horizontal_overflow_affordances(
    viewport: &TabViewport,
    baseline_y: u16,
    style: Style,
    buf: &mut Buffer,
) {
    if let Some(x) = viewport.before_affordance_at {
        buf[(x, baseline_y)].set_symbol("‹").set_style(style);
    }
    if let Some(x) = viewport.after_affordance_at {
        let symbol = if viewport.clipped_before {
            "›"
        } else {
            "…"
        };
        buf[(x, baseline_y)].set_symbol(symbol).set_style(style);
    }
}

fn draw_vertical_overflow_affordances(
    viewport: &TabViewport,
    rail_x: u16,
    style: Style,
    buf: &mut Buffer,
) {
    if let Some(y) = viewport.before_affordance_at {
        buf[(rail_x, y)].set_symbol("↑").set_style(style);
    }
    if let Some(y) = viewport.after_affordance_at {
        let symbol = if viewport.clipped_before {
            "↓"
        } else {
            "…"
        };
        buf[(rail_x, y)].set_symbol(symbol).set_style(style);
    }
}

fn tab_text_style(nav: &TabNav<'_>, active: bool) -> Style {
    if active {
        let mut style = nav.highlight_style;
        if nav.highlight_bold {
            style = style.add_modifier(Modifier::BOLD);
        }
        style
    } else {
        nav.style
    }
}

fn draw_horizontal_baseline(
    start: u16,
    end: u16,
    y: u16,
    border: &symbols::border::Set,
    style: Style,
    buf: &mut Buffer,
) {
    for x in start..end {
        buf[(x, y)]
            .set_symbol(border.horizontal_top)
            .set_style(style);
    }
}

fn apply_horizontal_tab_bar_end(
    start: u16,
    end: u16,
    y: u16,
    end_style: TabBarEnd,
    style: Style,
    buf: &mut Buffer,
) {
    if end_style == TabBarEnd::NoEnd || end <= start {
        return;
    }

    let (left_cap, right_cap) = match end_style {
        TabBarEnd::NoEnd => return,
        TabBarEnd::Angl => ("├", "┐"),
        TabBarEnd::Rnd => ("├", "╮"),
    };

    buf[(start, y)].set_symbol(left_cap).set_style(style);
    buf[(end - 1, y)].set_symbol(right_cap).set_style(style);
}

fn apply_vertical_tab_bar_end(
    first_active: bool,
    first_top: u16,
    right_x: u16,
    content_bottom: u16,
    end_style: TabBarEnd,
    style: Style,
    buf: &mut Buffer,
) {
    if end_style == TabBarEnd::NoEnd || content_bottom == 0 {
        return;
    }

    buf[(right_x, first_top)]
        .set_symbol(if first_active { "─" } else { "┬" })
        .set_style(style);

    let bottom_cap = match end_style {
        TabBarEnd::Angl => "└",
        TabBarEnd::Rnd => "╰",
        TabBarEnd::NoEnd => return,
    };
    buf[(right_x, content_bottom - 1)]
        .set_symbol(bottom_cap)
        .set_style(style);
}

fn draw_vertical_baseline(
    _left: u16,
    right: u16,
    start_y: u16,
    end_y: u16,
    border: &symbols::border::Set,
    style: Style,
    buf: &mut Buffer,
) {
    for y in start_y..end_y {
        buf[(right, y)]
            .set_symbol(border.vertical_left)
            .set_style(style);
    }
}

fn draw_top_border(
    left: u16,
    right: u16,
    y: u16,
    border: &symbols::border::Set,
    style: Style,
    buf: &mut Buffer,
) {
    buf[(left, y)].set_symbol(border.top_left).set_style(style);

    for x in (left + 1)..right {
        buf[(x, y)]
            .set_symbol(border.horizontal_top)
            .set_style(style);
    }

    buf[(right, y)]
        .set_symbol(border.top_right)
        .set_style(style);
}

fn draw_bottom_border(
    left: u16,
    right: u16,
    y: u16,
    border: &symbols::border::Set,
    style: Style,
    buf: &mut Buffer,
) {
    buf[(left, y)]
        .set_symbol(border.bottom_left)
        .set_style(style);

    for x in (left + 1)..right {
        buf[(x, y)]
            .set_symbol(border.horizontal_bottom)
            .set_style(style);
    }

    buf[(right, y)]
        .set_symbol(border.bottom_right)
        .set_style(style);
}

fn draw_horizontal_side_borders(
    left: u16,
    right: u16,
    top: u16,
    bottom: u16,
    border: &symbols::border::Set,
    style: Style,
    buf: &mut Buffer,
) {
    for y in (top + 1)..bottom {
        buf[(left, y)]
            .set_symbol(border.vertical_left)
            .set_style(style);

        buf[(right, y)]
            .set_symbol(border.vertical_right)
            .set_style(style);
    }
}

fn draw_vertical_side_borders(
    left: u16,
    _right: u16,
    top: u16,
    bottom: u16,
    border: &symbols::border::Set,
    style: Style,
    buf: &mut Buffer,
) {
    for y in (top + 1)..bottom {
        buf[(left, y)]
            .set_symbol(border.vertical_left)
            .set_style(style);
    }
}

fn label_char(ch: char, all_caps: bool) -> char {
    if all_caps {
        ch.to_uppercase().next().unwrap_or(ch)
    } else {
        ch
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_horizontal_label(
    left: u16,
    right: u16,
    y: u16,
    label: &str,
    pad: TabPadding,
    all_caps: bool,
    style: Style,
    buf: &mut Buffer,
) {
    let label_x = left + TAB_BORDER + pad.left;
    let max_x = right.saturating_sub(TAB_BORDER + pad.right);
    let mut col = label_x;

    for ch in label.chars() {
        let width = char_display_width(ch, all_caps);
        if width == 0 || col > max_x {
            break;
        }
        if col.saturating_add(width).saturating_sub(1) > max_x {
            break;
        }
        buf[(col, y)]
            .set_char(label_char(ch, all_caps))
            .set_style(style);
        col = col.saturating_add(width);
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_vertical_label(
    left: u16,
    right: u16,
    top: u16,
    label: &str,
    pad: TabPadding,
    all_caps: bool,
    style: Style,
    buf: &mut Buffer,
) {
    let (label_x, label_y) = label_origin(left, top, pad);
    let max_y = top + auto_vertical_tab_height(label, pad) - TAB_BORDER;

    for (row, line) in label.lines().enumerate() {
        let y = label_y + row as u16;
        if y >= max_y {
            break;
        }
        for (col, ch) in line.chars().enumerate() {
            let x = label_x + col as u16;
            if x > right.saturating_sub(TAB_BORDER + pad.right) {
                break;
            }
            buf[(x, y)]
                .set_char(label_char(ch, all_caps))
                .set_style(style);
        }
    }
}

fn draw_active_bottom(
    left: u16,
    right: u16,
    y: u16,
    border: &symbols::border::Set,
    style: Style,
    buf: &mut Buffer,
) {
    buf[(left, y)]
        .set_symbol(border.bottom_right)
        .set_style(style);

    for x in (left + 1)..right {
        buf[(x, y)].set_symbol(" ").set_style(style);
    }

    buf[(right, y)]
        .set_symbol(border.bottom_left)
        .set_style(style);
}

fn draw_inactive_horizontal_bottom(left: u16, right: u16, y: u16, style: Style, buf: &mut Buffer) {
    buf[(left, y)].set_symbol("┴").set_style(style);
    buf[(right, y)].set_symbol("┴").set_style(style);
}

fn draw_active_right(
    _left: u16,
    right: u16,
    top: u16,
    bottom: u16,
    border: &symbols::border::Set,
    style: Style,
    buf: &mut Buffer,
) {
    buf[(right, top)]
        .set_symbol(border.bottom_right)
        .set_style(style);

    for y in (top + 1)..bottom {
        buf[(right, y)].set_symbol(" ").set_style(style);
    }

    buf[(right, bottom)]
        .set_symbol(border.top_right)
        .set_style(style);
}

fn draw_inactive_vertical_right(
    _left: u16,
    right: u16,
    top: u16,
    bottom: u16,
    style: Style,
    buf: &mut Buffer,
) {
    buf[(right, top)].set_symbol("┤").set_style(style);
    buf[(right, bottom)].set_symbol("┤").set_style(style);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui_core::buffer::Buffer;
    use ratatui_core::layout::Rect;

    fn draw(nav: TabNav<'_>, area: Rect, buf: &mut Buffer) {
        ratatui_core::widgets::Widget::render(nav, area, buf);
    }

    fn render_horizontal(tabs: &[&str], selected: usize, width: u16) -> Buffer {
        let area = Rect::new(0, 0, width, 3);
        let mut buf = Buffer::empty(area);
        draw(TabNav::new(tabs, selected), area, &mut buf);
        buf
    }

    fn render_vertical(tabs: &[&str], selected: usize, width: u16, height: u16) -> Buffer {
        let area = Rect::new(0, 0, width, height);
        let mut buf = Buffer::empty(area);
        draw(
            TabNav::new(tabs, selected).orientation(TabOrientation::Vertical),
            area,
            &mut buf,
        );
        buf
    }

    #[test]
    fn vertical_label_helper() {
        assert_eq!(vertical_label("ROTATED"), "R\nO\nT\nA\nT\nE\nD");
        assert_eq!(vertical_label(""), "");
        assert_eq!(vertical_label("Hi"), "H\ni");
    }

    #[test]
    fn default_margin_and_padding() {
        let horizontal = TabNav::new(&["Tab"], 0);
        assert_eq!(effective_margin(&horizontal), TabMargin::ZERO);
        assert_eq!(
            effective_padding(&horizontal),
            TabPadding::horizontal_default()
        );

        let vertical = TabNav::new(&["T\na\nb"], 0).orientation(TabOrientation::Vertical);
        assert_eq!(effective_margin(&vertical), TabMargin::vertical_default());
        assert_eq!(effective_padding(&vertical), TabPadding::vertical_default());
    }

    #[test]
    fn margin_and_padding_overrides() {
        let nav = TabNav::new(&["Tab"], 0)
            .margin(TabMargin::horizontal(2, 4))
            .padding(TabPadding::uniform(1));
        assert_eq!(effective_margin(&nav), TabMargin::horizontal(2, 4));
        assert_eq!(effective_padding(&nav), TabPadding::uniform(1));
    }

    #[test]
    fn empty_tabs_renders_nothing() {
        let area = Rect::new(0, 0, 40, 3);
        let mut buf = Buffer::empty(area);
        let expected = buf.clone();
        draw(TabNav::new(&[], 0), area, &mut buf);
        assert_eq!(buf, expected);
    }

    #[test]
    fn insufficient_height_renders_nothing() {
        let area = Rect::new(0, 0, 40, 2);
        let mut buf = Buffer::empty(area);
        let expected = buf.clone();
        draw(TabNav::new(&["Tab"], 0), area, &mut buf);
        assert_eq!(buf, expected);
    }

    #[test]
    fn insufficient_width_renders_nothing_vertical() {
        let area = Rect::new(0, 0, 2, 20);
        let mut buf = Buffer::empty(area);
        let expected = buf.clone();
        draw(
            TabNav::new(&["T\na\nb"], 0).orientation(TabOrientation::Vertical),
            area,
            &mut buf,
        );
        assert_eq!(buf, expected);
    }

    #[test]
    fn single_tab_renders_three_rows() {
        let buf = render_horizontal(&["Hi"], 0, 30);
        let top_line = line_str(&buf, 0);
        let mid_line = line_str(&buf, 1);
        let bot_line = line_str(&buf, 2);

        assert!(top_line.starts_with("╭"));
        assert!(top_line.contains("╮"));
        assert!(mid_line.contains("Hi"));
        assert!(mid_line.contains("▸"));
        assert!(bot_line.starts_with("╯"));
    }

    #[test]
    fn inactive_tab_has_junction_corners() {
        let buf = render_horizontal(&["A", "B"], 1, 30);
        let bot_line = line_str(&buf, 2);
        assert!(bot_line.starts_with("┴"));
    }

    #[test]
    fn indicator_appears_on_active_tab() {
        let buf = render_horizontal(&["Tab"], 0, 20);
        let mid_line = line_str(&buf, 1);
        assert!(mid_line.contains("▸"));
    }

    #[test]
    fn no_indicator_when_disabled() {
        let area = Rect::new(0, 0, 20, 3);
        let mut buf = Buffer::empty(area);
        draw(TabNav::new(&["Tab"], 0).indicator(None), area, &mut buf);
        let mid_line = line_str(&buf, 1);
        assert!(!mid_line.contains("▸"));
    }

    #[test]
    fn overflow_tabs_are_omitted() {
        let buf = render_horizontal(&["Long", "Overflow"], 0, 20);
        let mid_line = line_str(&buf, 1);
        assert!(mid_line.contains("Long"));
        assert!(!mid_line.contains("Overflow"));
    }

    #[test]
    fn square_borders() {
        let area = Rect::new(0, 0, 20, 3);
        let mut buf = Buffer::empty(area);
        draw(
            TabNav::new(&["Tab"], 0).border_set(symbols::border::PLAIN),
            area,
            &mut buf,
        );
        let top_line = line_str(&buf, 0);
        assert!(top_line.starts_with("┌"));
    }

    #[test]
    fn horizontal_tab_width_calculation() {
        let pad = TabPadding::horizontal_default();
        assert_eq!(auto_horizontal_tab_width("Hi", pad, false), 10);
        assert_eq!(auto_horizontal_tab_width("", pad, false), 8);
        assert_eq!(auto_horizontal_tab_width("Nodes", pad, false), 13);
    }

    #[test]
    fn vertical_tab_height_calculation() {
        let pad = TabPadding::vertical_default();
        assert_eq!(auto_vertical_tab_height("Hi", pad), 5);
        assert_eq!(auto_vertical_tab_height("", pad), 4);
        assert_eq!(auto_vertical_tab_height("A\nB\nC", pad), 7);
    }

    #[test]
    fn horizontal_margin_shifts_tabs() {
        let area = Rect::new(0, 0, 20, 3);
        let mut buf = Buffer::empty(area);
        draw(
            TabNav::new(&["Tab"], 0).margin(TabMargin::horizontal(2, 0)),
            area,
            &mut buf,
        );
        let top_line = line_str(&buf, 0);
        assert!(top_line.starts_with("  ╭"));
    }

    #[test]
    fn two_active_tabs_layout() {
        let buf = render_horizontal(&["A", "B"], 0, 30);
        let mid_line = line_str(&buf, 1);
        assert!(mid_line.contains("A"));
        assert!(mid_line.contains("B"));
    }

    #[test]
    fn horizontal_tab_bar_end_angl() {
        let area = Rect::new(0, 0, 20, 3);
        let mut buf = Buffer::empty(area);
        draw(
            TabNav::new(&["Tab"], 0).tab_bar_end(TabBarEnd::Angl),
            area,
            &mut buf,
        );
        let bot_line = line_str(&buf, 2);
        assert!(bot_line.starts_with('├'));
        assert!(bot_line.ends_with('┐'));
    }

    #[test]
    fn horizontal_tab_bar_end_rnd() {
        let area = Rect::new(0, 0, 20, 3);
        let mut buf = Buffer::empty(area);
        draw(
            TabNav::new(&["Tab"], 0).tab_bar_end(TabBarEnd::Rnd),
            area,
            &mut buf,
        );
        let bot_line = line_str(&buf, 2);
        assert!(bot_line.starts_with('├'));
        assert!(bot_line.ends_with('╮'));
    }

    #[test]
    fn all_caps_renders_uppercase_labels() {
        let area = Rect::new(0, 0, 30, 3);
        let mut buf = Buffer::empty(area);
        draw(TabNav::new(&["Example"], 0).all_caps(true), area, &mut buf);
        let mid_line = line_str(&buf, 1);
        assert!(mid_line.contains("EXAMPLE"));
        assert!(!mid_line.contains("Example"));
    }

    #[test]
    fn vertical_tab_bar_end_rnd() {
        let label = vertical_label("Tab");
        let tabs = [label.as_str()];
        let nav = TabNav::new(&tabs, 0)
            .orientation(TabOrientation::Vertical)
            .tab_bar_end(TabBarEnd::Rnd);
        let width = nav.vertical_rail_width();
        let height = auto_vertical_tab_height(tabs[0], TabPadding::vertical_default());
        let area = Rect::new(0, 0, width, height);
        let mut buf = Buffer::empty(area);
        draw(nav, area, &mut buf);
        let right_col = col_str(&buf, width - 1);
        assert!(right_col.starts_with('─'));
        assert!(right_col.ends_with('╰'));
    }

    #[test]
    fn vertical_default_indicator_disabled() {
        let label = vertical_label("Tab");
        let tabs = [label.as_str()];
        let nav = TabNav::new(&tabs, 0).orientation(TabOrientation::Vertical);
        let width = nav.vertical_rail_width();
        let height = auto_vertical_tab_height(tabs[0], TabPadding::vertical_default());
        let buf = render_vertical(&tabs, 0, width, height);
        let label_col = col_str(&buf, 2);
        assert!(!label_col.contains('▸'));
    }

    #[test]
    fn vertical_single_tab_renders_stacked_label() {
        let label = vertical_label("Log");
        let label = label.as_str();
        let pad = TabPadding::vertical_default();
        let width = auto_horizontal_tab_width(label, pad, false);
        let height = auto_vertical_tab_height(label, pad);
        let area = Rect::new(0, 0, width, height);
        let mut buf = Buffer::empty(area);
        draw(
            TabNav::new(&[label], 0)
                .orientation(TabOrientation::Vertical)
                .indicator(Some(DEFAULT_INDICATOR)),
            area,
            &mut buf,
        );
        let label_col = col_str(&buf, 2);

        assert!(label_col.contains("L"));
        assert!(label_col.contains("o"));
        assert!(label_col.contains("g"));
        assert_eq!(buf[(2, 1)].symbol(), "▸");
        assert_eq!(buf[(2, 2)].symbol(), "L");
    }

    #[test]
    fn vertical_active_tab_top_border_uses_top_right_corner() {
        let label = vertical_label("Tab");
        let tabs = [label.as_str()];
        let nav = TabNav::new(&tabs, 0).orientation(TabOrientation::Vertical);
        let width = nav.vertical_rail_width();
        let height = auto_vertical_tab_height(tabs[0], TabPadding::vertical_default());
        let buf = render_vertical(&tabs, 0, width, height);
        let top_line = line_str(&buf, 0);

        assert!(top_line.starts_with('╭'));
        assert!(top_line.ends_with('╯'));
    }

    #[test]
    fn vertical_active_tab_opens_right() {
        let label = vertical_label("Tab");
        let tabs = [label.as_str()];
        let nav = TabNav::new(&tabs, 0).orientation(TabOrientation::Vertical);
        let width = nav.vertical_rail_width();
        let height = auto_vertical_tab_height(tabs[0], TabPadding::vertical_default());
        let buf = render_vertical(&tabs, 0, width, height);
        let active_col: String = (0..height)
            .map(|y| buf[(width - 1, y)].symbol().to_string())
            .collect();
        let glyphs: Vec<char> = active_col.chars().collect();

        assert_eq!(glyphs.first(), Some(&'╯'));
        assert!(glyphs[1..glyphs.len() - 1].iter().all(|&ch| ch == ' '));
        assert_eq!(glyphs.last(), Some(&'╮'));
    }

    #[test]
    fn vertical_inactive_tab_has_right_junction() {
        let first = vertical_label("One");
        let second = vertical_label("Two");
        let first = first.as_str();
        let second = second.as_str();
        let pad = TabPadding::vertical_default();
        let width = auto_horizontal_tab_width(first, pad, false);
        let height = auto_vertical_tab_height(first, pad) + auto_vertical_tab_height(second, pad);
        let buf = render_vertical(&[first, second], 1, width, height);
        let right_col = col_str(&buf, width - 1);

        assert!(right_col.contains('┤'));
    }

    #[test]
    fn vertical_overflow_tabs_are_omitted() {
        let tall = vertical_label("ABCDEFGHIJ");
        let tall = tall.as_str();
        let also = vertical_label("X");
        let also = also.as_str();
        let pad = TabPadding::vertical_default();
        let width = auto_horizontal_tab_width(tall, pad, false);
        let height = auto_vertical_tab_height(tall, pad);
        let buf = render_vertical(&[tall, also], 0, width, height);
        let col = col_str(&buf, 2);

        assert!(col.contains('A'));
        assert!(!col.contains('X'));
    }

    #[test]
    fn tab_rects_match_auto_horizontal_widths() {
        let tabs = ["Files", "Search"];
        let nav = TabNav::new(&tabs, 0);
        let area = Rect::new(5, 2, 40, 3);
        let rects = nav.tab_rects(area);

        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0].x, area.x);
        assert_eq!(rects[0].width, nav.auto_tab_width(0).unwrap());
        assert_eq!(rects[0].height, nav.horizontal_strip_height());
        assert_eq!(rects[1].x, rects[0].x + rects[0].width);
        assert_eq!(rects[1].width, nav.auto_tab_width(1).unwrap());
    }

    #[test]
    fn tab_widths_override_auto_layout() {
        let tabs = ["A", "B"];
        let widths = [20, 12];
        let nav = TabNav::new(&tabs, 0).tab_widths(&widths);
        let rects = nav.tab_rects(Rect::new(0, 0, 40, 3));

        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0].width, 20);
        assert_eq!(rects[1].width, 12);
        assert_ne!(rects[0].width, nav.auto_tab_width(0).unwrap());
    }

    #[test]
    fn tab_rects_respect_margin_and_overflow() {
        let nav = TabNav::new(&["Long", "Overflow"], 0).margin(TabMargin::horizontal(2, 0));
        let area = Rect::new(0, 0, 20, 3);
        let rects = nav.tab_rects(area);

        assert_eq!(rects.len(), 1);
        assert_eq!(rects[0].x, 2);
    }

    #[test]
    fn tab_rects_vertical_match_auto_heights() {
        let label = vertical_label("Tab");
        let tabs = [label.as_str()];
        let nav = TabNav::new(&tabs, 0).orientation(TabOrientation::Vertical);
        let width = nav.vertical_rail_width();
        let height = nav.auto_tab_height(0).unwrap();
        let area = Rect::new(0, 0, width, height + 4);
        let rects = nav.tab_rects(area);

        assert_eq!(rects.len(), 1);
        assert_eq!(rects[0].y, area.y);
        assert_eq!(rects[0].width, width);
        assert_eq!(rects[0].height, height);
    }

    #[test]
    fn unicode_label_uses_display_width() {
        let pad = TabPadding::horizontal_default();
        assert_eq!(auto_horizontal_tab_width("日本", pad, false), 12);
        assert_eq!(
            auto_horizontal_tab_width("日本", pad, false),
            auto_horizontal_tab_width("abcd", pad, false)
        );
    }

    #[test]
    fn scroll_mode_shows_later_tabs() {
        let tabs = ["One", "Two", "Three", "Four"];
        let area = Rect::new(0, 0, 28, 3);
        let mut buf = Buffer::empty(area);
        draw(
            TabNav::new(&tabs, 3)
                .overflow(OverflowPolicy::Scroll)
                .scroll_offset(2),
            area,
            &mut buf,
        );
        let mid_line = line_str(&buf, 1);
        assert!(!mid_line.contains("One"));
        assert!(mid_line.contains("Three") || mid_line.contains("Four"));
    }

    #[test]
    fn truncate_shows_overflow_affordance() {
        let area = Rect::new(0, 0, 20, 3);
        let mut buf = Buffer::empty(area);
        draw(TabNav::new(&["Long", "Overflow"], 0), area, &mut buf);
        let bot_line = line_str(&buf, 2);
        assert!(bot_line.contains('…') || bot_line.contains('›'));
    }

    #[test]
    fn stateful_widget_uses_state_selection() {
        let area = Rect::new(0, 0, 30, 3);
        let mut buf = Buffer::empty(area);
        let mut state = TabNavState::new(1);
        ratatui_core::widgets::StatefulWidget::render(
            TabNav::new(&["A", "B"], 0),
            area,
            &mut buf,
            &mut state,
        );
        let mid_line = line_str(&buf, 1);
        assert!(mid_line.contains('B'));
    }

    #[test]
    fn ensure_selected_visible_scrolls_window() {
        let tabs = ["A", "B", "C", "D", "E"];
        let nav = TabNav::new(&tabs, 0).overflow(OverflowPolicy::Scroll);
        let area = Rect::new(0, 0, 24, 3);
        let mut state = TabNavState::new(4);
        state.ensure_selected_visible(&nav, area);
        assert!(state.scroll_offset > 0);
        let viewport = compute_viewport(&nav, area, state.scroll_offset);
        assert!(viewport.entries.iter().any(|entry| entry.index == 4));
    }

    fn line_str(buf: &Buffer, y: u16) -> String {
        let area = buf.area();
        (area.x..area.right())
            .map(|x| buf[(x, y)].symbol().to_string())
            .collect()
    }

    fn col_str(buf: &Buffer, x: u16) -> String {
        let area = buf.area();
        (area.y..area.bottom())
            .map(|y| buf[(x, y)].symbol().to_string())
            .collect()
    }
}
