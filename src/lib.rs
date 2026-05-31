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
    widgets::Widget,
};

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
        Self { start: top, end: bottom }
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
    /// Square caps: horizontal `┌`/`┐`; vertical top junction `┬`/`─` and bottom `└`.
    Angl,
    /// Rounded caps: horizontal `╭`/`╮`; vertical top junction `┬`/`─` and bottom `╰`.
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

/// Converts a single-line label into a vertical stack of characters (one per row).
///
/// ```
/// use ratatui_comfy_tabs::vertical_label;
///
/// assert_eq!(vertical_label("Hi"), "H\ni");
/// ```
pub fn vertical_label(text: &str) -> String {
    text.chars().map(|c| c.to_string()).collect::<Vec<_>>().join("\n")
}

/// Tab navigation rendered as individually bordered boxes.
///
/// Adjacent tabs sit flush (no gap). The active tab opens into the adjacent content
/// via rounded junction corners. Inactive tabs use T-junctions so the baseline stays
/// continuous.
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
    style: Style,
    highlight_style: Style,
    highlight_bold: bool,
    border_style: Style,
    indicator: Option<&'a str>,
    indicator_explicit: bool,
    border_set: symbols::border::Set<'a>,
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
            style: Style::new(),
            highlight_style: Style::new(),
            highlight_bold: true,
            border_style: Style::new(),
            indicator: Some(DEFAULT_INDICATOR),
            indicator_explicit: false,
            border_set: symbols::border::ROUNDED,
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

    /// Minimum height for a horizontal tab strip with the current padding.
    pub fn horizontal_strip_height(&self) -> u16 {
        let pad = effective_padding(self);
        TAB_BORDER * 2 + pad.top + 1 + pad.bottom
    }

    /// Width of the vertical tab rail (widest tab) with the current padding.
    pub fn vertical_rail_width(&self) -> u16 {
        self.tabs
            .iter()
            .map(|label| tab_width(label, effective_padding(self)))
            .max()
            .unwrap_or_else(|| tab_width("", effective_padding(self)))
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

fn label_display_width(label: &str) -> u16 {
    match label.lines().map(|line| line.len()).max() {
        Some(width) => width as u16,
        None => 0,
    }
}

fn tab_width(label: &str, pad: TabPadding) -> u16 {
    TAB_BORDER * 2 + pad.left + label_display_width(label) + pad.right
}

fn tab_height(label: &str, pad: TabPadding) -> u16 {
    TAB_BORDER * 2 + pad.top + label_line_count(label) + pad.bottom
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
    (
        left + TAB_BORDER + pad.left,
        top + TAB_BORDER + pad.top,
    )
}

impl Widget for TabNav<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.tabs.is_empty() {
            return;
        }

        match self.orientation {
            TabOrientation::Horizontal => render_horizontal(self, area, buf),
            TabOrientation::Vertical => render_vertical(self, area, buf),
        }
    }
}

fn render_horizontal(nav: TabNav<'_>, area: Rect, buf: &mut Buffer) {
    let margin = effective_margin(&nav);
    let pad = effective_padding(&nav);
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

    let positions = compute_horizontal_tab_positions(nav.tabs, pad, content_left, content_right);

    for (i, (label, &(tx, tw))) in nav.tabs.iter().zip(&positions).enumerate() {
        let active = i == nav.selected;
        let left_x = tx;
        let right_x = tx + tw - 1;
        let text_style = tab_text_style(&nav, active);

        draw_top_border(left_x, right_x, top_y, border, bs, buf);
        draw_horizontal_side_borders(left_x, right_x, top_y, bot_y, border, bs, buf);
        draw_horizontal_label(left_x, right_x, label_y, label, pad, text_style, buf);

        if active {
            if let Some(sym) = effective_indicator(&nav) {
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

    apply_horizontal_tab_bar_end(
        content_left,
        content_right,
        bot_y,
        effective_tab_bar_end(&nav),
        bs,
        buf,
    );
}

fn render_vertical(nav: TabNav<'_>, area: Rect, buf: &mut Buffer) {
    let margin = effective_margin(&nav);
    let pad = effective_padding(&nav);
    let rail_width = nav.vertical_rail_width().min(area.width);

    if rail_width < TAB_BORDER * 2 + pad.left + pad.right || area.height <= margin.start + margin.end {
        return;
    }

    let border = &nav.border_set;
    let bs = nav.border_style;
    let left_x = area.x;
    let right_x = area.x + rail_width - 1;
    let content_top = area.y + margin.start;
    let content_bottom = area.bottom() - margin.end;

    draw_vertical_baseline(left_x, right_x, content_top, content_bottom, border, bs, buf);

    let positions = compute_vertical_tab_positions(nav.tabs, pad, content_top, content_bottom);
    let mut first_rendered: Option<(usize, u16)> = None;

    for (i, (label, &(ty, th))) in nav.tabs.iter().zip(&positions).enumerate() {
        if first_rendered.is_none() {
            first_rendered = Some((i, ty));
        }
        let active = i == nav.selected;
        let top_y = ty;
        let bot_y = ty + th - 1;
        let text_style = tab_text_style(&nav, active);

        draw_top_border(left_x, right_x, top_y, border, bs, buf);
        draw_vertical_side_borders(left_x, right_x, top_y, bot_y, border, bs, buf);
        draw_vertical_label(left_x, right_x, top_y, label, pad, text_style, buf);
        draw_bottom_border(left_x, right_x, bot_y, border, bs, buf);

        if active {
            if let Some(sym) = effective_indicator(&nav) {
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

    if let Some((first_index, first_top)) = first_rendered {
        apply_vertical_tab_bar_end(
            first_index == nav.selected,
            first_top,
            right_x,
            content_bottom,
            effective_tab_bar_end(&nav),
            bs,
            buf,
        );
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
        TabBarEnd::Angl => ("┌", "┐"),
        TabBarEnd::Rnd => ("╭", "╮"),
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

fn compute_horizontal_tab_positions(
    tabs: &[&str],
    pad: TabPadding,
    start: u16,
    end: u16,
) -> Vec<(u16, u16)> {
    let mut positions = Vec::with_capacity(tabs.len());
    let mut x = start;

    for label in tabs {
        let w = tab_width(label, pad);
        if x + w > end {
            break;
        }
        positions.push((x, w));
        x += w;
    }

    positions
}

fn compute_vertical_tab_positions(
    tabs: &[&str],
    pad: TabPadding,
    start_y: u16,
    end_y: u16,
) -> Vec<(u16, u16)> {
    let mut positions = Vec::with_capacity(tabs.len());
    let mut y = start_y;

    for label in tabs {
        let h = tab_height(label, pad);
        if y + h > end_y {
            break;
        }
        positions.push((y, h));
        y += h;
    }

    positions
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

fn draw_horizontal_label(
    left: u16,
    right: u16,
    y: u16,
    label: &str,
    pad: TabPadding,
    style: Style,
    buf: &mut Buffer,
) {
    let label_x = left + TAB_BORDER + pad.left;

    for (j, ch) in label.chars().enumerate() {
        let cx = label_x + j as u16;
        if cx > right.saturating_sub(TAB_BORDER + pad.right) {
            break;
        }
        buf[(cx, y)].set_char(ch).set_style(style);
    }
}

fn draw_vertical_label(
    left: u16,
    right: u16,
    top: u16,
    label: &str,
    pad: TabPadding,
    style: Style,
    buf: &mut Buffer,
) {
    let (label_x, label_y) = label_origin(left, top, pad);
    let max_y = top + tab_height(label, pad) - TAB_BORDER;

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
            buf[(x, y)].set_char(ch).set_style(style);
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

fn draw_inactive_vertical_right(_left: u16, right: u16, top: u16, bottom: u16, style: Style, buf: &mut Buffer) {
    buf[(right, top)].set_symbol("┤").set_style(style);
    buf[(right, bottom)].set_symbol("┤").set_style(style);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui_core::buffer::Buffer;
    use ratatui_core::layout::Rect;

    fn render_horizontal(tabs: &[&str], selected: usize, width: u16) -> Buffer {
        let area = Rect::new(0, 0, width, 3);
        let mut buf = Buffer::empty(area);
        TabNav::new(tabs, selected).render(area, &mut buf);
        buf
    }

    fn render_vertical(tabs: &[&str], selected: usize, width: u16, height: u16) -> Buffer {
        let area = Rect::new(0, 0, width, height);
        let mut buf = Buffer::empty(area);
        TabNav::new(tabs, selected)
            .orientation(TabOrientation::Vertical)
            .render(area, &mut buf);
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
        assert_eq!(effective_padding(&horizontal), TabPadding::horizontal_default());

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
        TabNav::new(&[], 0).render(area, &mut buf);
        assert_eq!(buf, expected);
    }

    #[test]
    fn insufficient_height_renders_nothing() {
        let area = Rect::new(0, 0, 40, 2);
        let mut buf = Buffer::empty(area);
        let expected = buf.clone();
        TabNav::new(&["Tab"], 0).render(area, &mut buf);
        assert_eq!(buf, expected);
    }

    #[test]
    fn insufficient_width_renders_nothing_vertical() {
        let area = Rect::new(0, 0, 2, 20);
        let mut buf = Buffer::empty(area);
        let expected = buf.clone();
        TabNav::new(&["T\na\nb"], 0)
            .orientation(TabOrientation::Vertical)
            .render(area, &mut buf);
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
        TabNav::new(&["Tab"], 0)
            .indicator(None)
            .render(area, &mut buf);
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
        TabNav::new(&["Tab"], 0)
            .border_set(symbols::border::PLAIN)
            .render(area, &mut buf);
        let top_line = line_str(&buf, 0);
        assert!(top_line.starts_with("┌"));
    }

    #[test]
    fn horizontal_tab_width_calculation() {
        let pad = TabPadding::horizontal_default();
        assert_eq!(tab_width("Hi", pad), 10);
        assert_eq!(tab_width("", pad), 8);
        assert_eq!(tab_width("Nodes", pad), 13);
    }

    #[test]
    fn vertical_tab_height_calculation() {
        let pad = TabPadding::vertical_default();
        assert_eq!(tab_height("Hi", pad), 5);
        assert_eq!(tab_height("", pad), 4);
        assert_eq!(tab_height("A\nB\nC", pad), 7);
    }

    #[test]
    fn horizontal_margin_shifts_tabs() {
        let area = Rect::new(0, 0, 20, 3);
        let mut buf = Buffer::empty(area);
        TabNav::new(&["Tab"], 0)
            .margin(TabMargin::horizontal(2, 0))
            .render(area, &mut buf);
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
        TabNav::new(&["Tab"], 0)
            .tab_bar_end(TabBarEnd::Angl)
            .render(area, &mut buf);
        let bot_line = line_str(&buf, 2);
        assert!(bot_line.starts_with('┌'));
        assert!(bot_line.ends_with('┐'));
    }

    #[test]
    fn horizontal_tab_bar_end_rnd() {
        let area = Rect::new(0, 0, 20, 3);
        let mut buf = Buffer::empty(area);
        TabNav::new(&["Tab"], 0)
            .tab_bar_end(TabBarEnd::Rnd)
            .render(area, &mut buf);
        let bot_line = line_str(&buf, 2);
        assert!(bot_line.starts_with('╭'));
        assert!(bot_line.ends_with('╮'));
    }

    #[test]
    fn vertical_tab_bar_end_rnd() {
        let label = vertical_label("Tab");
        let tabs = [label.as_str()];
        let nav = TabNav::new(&tabs, 0)
            .orientation(TabOrientation::Vertical)
            .tab_bar_end(TabBarEnd::Rnd);
        let width = nav.vertical_rail_width();
        let height = tab_height(tabs[0], TabPadding::vertical_default());
        let area = Rect::new(0, 0, width, height);
        let mut buf = Buffer::empty(area);
        nav.render(area, &mut buf);
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
        let height = tab_height(tabs[0], TabPadding::vertical_default());
        let buf = render_vertical(&tabs, 0, width, height);
        let label_col = col_str(&buf, 2);
        assert!(!label_col.contains('▸'));
    }

    #[test]
    fn vertical_single_tab_renders_stacked_label() {
        let label = vertical_label("Log");
        let label = label.as_str();
        let pad = TabPadding::vertical_default();
        let width = tab_width(label, pad);
        let height = tab_height(label, pad);
        let area = Rect::new(0, 0, width, height);
        let mut buf = Buffer::empty(area);
        TabNav::new(&[label], 0)
            .orientation(TabOrientation::Vertical)
            .indicator(Some(DEFAULT_INDICATOR))
            .render(area, &mut buf);
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
        let height = tab_height(tabs[0], TabPadding::vertical_default());
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
        let height = tab_height(tabs[0], TabPadding::vertical_default());
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
        let width = tab_width(first, pad);
        let height = tab_height(first, pad) + tab_height(second, pad);
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
        let width = tab_width(tall, pad);
        let height = tab_height(tall, pad);
        let buf = render_vertical(&[tall, also], 0, width, height);
        let col = col_str(&buf, 2);

        assert!(col.contains('A'));
        assert!(!col.contains('X'));
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
