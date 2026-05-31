//! A tab navigation widget for [Ratatui](https://ratatui.rs) with bordered boxes and rounded corners.
//!
//! Each tab renders as an individual bordered box. The active tab opens into the adjacent content
//! panel via rounded junction corners while inactive tabs maintain a continuous baseline.
//!
//! # Example
//!
//! ```rust
//! use ratatui::style::{Color, Style};
//! use ratatui_comfy_tabs::TabNav;
//!
//! let widget = TabNav::new(&["Files", "Search", "Settings"], 0)
//!     .highlight_style(Style::new().fg(Color::Cyan))
//!     .border_style(Style::new().fg(Color::DarkGray));
//! ```

use ratatui_core::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    symbols,
    widgets::Widget,
};

const DEFAULT_INDICATOR: &str = "▸";
const TAB_PAD: u16 = 3;
const TAB_BORDER: u16 = 1;
const TAB_FRAME: u16 = TAB_BORDER + TAB_PAD + TAB_PAD + TAB_BORDER; // 8

/// Tab strip layout orientation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabOrientation {
    /// Tabs in a row above content. Requires 3 rows of height.
    #[default]
    Horizontal,
    /// Tabs in a column beside content (left rail). Requires 3 columns of width.
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
/// - [`TabOrientation::Horizontal`]: baseline along the bottom; requires height ≥ 3.
///   Indicator defaults to `Some("▸")`.
/// - [`TabOrientation::Vertical`]: baseline along the right edge; requires width ≥ 3.
///   Indicator defaults to `None`. Use [`vertical_label`] or embed `\n` in labels.
#[must_use]
pub struct TabNav<'a> {
    tabs: &'a [&'a str],
    selected: usize,
    orientation: TabOrientation,
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
    ///
    /// All styles default to `Style::new()` (unstyled). The active tab is bold
    /// by default. Override with [`highlight_bold`](Self::highlight_bold).
    pub fn new(tabs: &'a [&'a str], selected: usize) -> Self {
        Self {
            tabs,
            selected,
            orientation: TabOrientation::Horizontal,
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

    /// Style for inactive tab labels.
    pub fn style(mut self, style: impl Into<Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Style for the active tab label. Bold is applied on top unless disabled
    /// via [`highlight_bold`](Self::highlight_bold).
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
    ///
    /// Default: `Some("▸")` for horizontal tabs, `None` for vertical tabs.
    /// Pass `Some("…")` or `None` to override either default.
    pub fn indicator(mut self, indicator: Option<&'a str>) -> Self {
        self.indicator = indicator;
        self.indicator_explicit = true;
        self
    }

    /// Border character set. Default: [`symbols::border::ROUNDED`].
    /// Pass [`symbols::border::PLAIN`] for square corners.
    pub fn border_set(mut self, set: symbols::border::Set<'a>) -> Self {
        self.border_set = set;
        self
    }
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

/// Horizontal: `│  ▸ Label  │` → border + pad + label + pad + border
fn horizontal_tab_width(label: &str) -> u16 {
    label_display_width(label) + TAB_FRAME
}

/// Vertical: fixed rail width, variable height per label line count.
fn vertical_tab_height(label: &str) -> u16 {
    label_line_count(label) + TAB_FRAME
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

fn label_origin(left: u16, top: u16) -> (u16, u16) {
    (left + TAB_BORDER + TAB_PAD, top + TAB_BORDER + TAB_PAD)
}

fn vertical_rail_width(tabs: &[&str]) -> u16 {
    tabs
        .iter()
        .map(|label| horizontal_tab_width(label))
        .max()
        .unwrap_or(TAB_FRAME)
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
    if area.height < 3 || area.width == 0 {
        return;
    }

    let border = &nav.border_set;
    let bs = nav.border_style;

    let top_y = area.y;
    let mid_y = area.y + 1;
    let bot_y = area.y + 2;

    draw_horizontal_baseline(area.x, area.right(), bot_y, border, bs, buf);

    let positions = compute_horizontal_tab_positions(nav.tabs, area.x, area.right());

    for (i, (label, &(tx, tw))) in nav.tabs.iter().zip(&positions).enumerate() {
        let active = i == nav.selected;
        let left_x = tx;
        let right_x = tx + tw - 1;
        let text_style = tab_text_style(&nav, active);

        draw_top_border(left_x, right_x, top_y, border, bs, buf);
        draw_horizontal_side_borders(left_x, right_x, mid_y, border, bs, buf);
        draw_horizontal_label(left_x, right_x, mid_y, label, text_style, buf);

        if active {
            if let Some(sym) = effective_indicator(&nav) {
                buf[(left_x + TAB_PAD, mid_y)]
                    .set_symbol(sym)
                    .set_style(text_style);
            }
            draw_active_bottom(left_x, right_x, bot_y, border, bs, buf);
        } else {
            draw_inactive_horizontal_bottom(left_x, right_x, bot_y, bs, buf);
        }
    }
}

fn render_vertical(nav: TabNav<'_>, area: Rect, buf: &mut Buffer) {
    if area.width < 3 || area.height == 0 {
        return;
    }

    let border = &nav.border_set;
    let bs = nav.border_style;
    let rail_width = vertical_rail_width(nav.tabs).min(area.width);
    let left_x = area.x;
    let right_x = area.x + rail_width - 1;

    draw_vertical_baseline(left_x, right_x, area.y, area.bottom(), border, bs, buf);

    let positions =
        compute_vertical_tab_positions(nav.tabs, area.y, area.bottom());

    for (i, (label, &(ty, th))) in nav.tabs.iter().zip(&positions).enumerate() {
        let active = i == nav.selected;
        let top_y = ty;
        let bot_y = ty + th - 1;
        let text_style = tab_text_style(&nav, active);

        draw_top_border(left_x, right_x, top_y, border, bs, buf);
        draw_vertical_side_borders(left_x, right_x, top_y, bot_y, border, bs, buf);
        draw_vertical_label(left_x, right_x, top_y, label, text_style, buf);
        draw_bottom_border(left_x, right_x, bot_y, border, bs, buf);

        if active {
            if let Some(sym) = effective_indicator(&nav) {
                let (label_x, label_y) = label_origin(left_x, top_y);
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

fn compute_horizontal_tab_positions(tabs: &[&str], start: u16, end: u16) -> Vec<(u16, u16)> {
    let mut positions = Vec::with_capacity(tabs.len());
    let mut x = start;

    for label in tabs {
        let w = horizontal_tab_width(label);
        if x + w > end {
            break;
        }
        positions.push((x, w));
        x += w;
    }

    positions
}

fn compute_vertical_tab_positions(tabs: &[&str], start_y: u16, end_y: u16) -> Vec<(u16, u16)> {
    let mut positions = Vec::with_capacity(tabs.len());
    let mut y = start_y;

    for label in tabs {
        let h = vertical_tab_height(label);
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
    y: u16,
    border: &symbols::border::Set,
    style: Style,
    buf: &mut Buffer,
) {
    buf[(left, y)]
        .set_symbol(border.vertical_left)
        .set_style(style);

    buf[(right, y)]
        .set_symbol(border.vertical_right)
        .set_style(style);
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

fn draw_horizontal_label(left: u16, right: u16, y: u16, label: &str, style: Style, buf: &mut Buffer) {
    let label_x = left + TAB_BORDER + TAB_PAD;

    for (j, ch) in label.chars().enumerate() {
        let cx = label_x + j as u16;
        if cx >= right {
            break;
        }
        buf[(cx, y)].set_char(ch).set_style(style);
    }
}

fn draw_vertical_label(left: u16, right: u16, top: u16, label: &str, style: Style, buf: &mut Buffer) {
    let label_x = left + TAB_BORDER + TAB_PAD;
    let label_y = top + TAB_BORDER + TAB_PAD;

    for (row, line) in label.lines().enumerate() {
        let y = label_y + row as u16;
        if y >= top + vertical_tab_height(label) - TAB_BORDER {
            break;
        }
        for (col, ch) in line.chars().enumerate() {
            let x = label_x + col as u16;
            if x >= right {
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
    // Mirror horizontal active bottom: open toward content on the right.
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
        assert_eq!(horizontal_tab_width("Hi"), 10);
        assert_eq!(horizontal_tab_width(""), 8);
        assert_eq!(horizontal_tab_width("Nodes"), 13);
    }

    #[test]
    fn vertical_tab_height_calculation() {
        assert_eq!(vertical_tab_height("Hi"), 9);
        assert_eq!(vertical_tab_height(""), 8);
        assert_eq!(vertical_tab_height("A\nB\nC"), 11);
    }

    #[test]
    fn two_active_tabs_layout() {
        let buf = render_horizontal(&["A", "B"], 0, 30);
        let mid_line = line_str(&buf, 1);
        assert!(mid_line.contains("A"));
        assert!(mid_line.contains("B"));
    }

    #[test]
    fn vertical_default_indicator_disabled() {
        let label = vertical_label("Tab");
        let label = label.as_str();
        let width = vertical_rail_width(&[label]);
        let height = vertical_tab_height(label);
        let buf = render_vertical(&[label], 0, width, height);
        let label_col = col_str(&buf, 4);
        assert!(!label_col.contains('▸'));
    }

    #[test]
    fn vertical_single_tab_renders_stacked_label() {
        let label = vertical_label("Log");
        let label = label.as_str();
        let width = vertical_rail_width(&[label]);
        let height = vertical_tab_height(label);
        let area = Rect::new(0, 0, width, height);
        let mut buf = Buffer::empty(area);
        TabNav::new(&[label], 0)
            .orientation(TabOrientation::Vertical)
            .indicator(Some(DEFAULT_INDICATOR))
            .render(area, &mut buf);
        let label_col = col_str(&buf, 4);
        let indicator_col = col_str(&buf, 4);

        assert!(label_col.contains("L"));
        assert!(label_col.contains("o"));
        assert!(label_col.contains("g"));
        assert!(indicator_col.contains('▸'));
        assert_eq!(buf[(4, 3)].symbol(), "▸");
        assert_eq!(buf[(4, 4)].symbol(), "L");
    }

    #[test]
    fn vertical_active_tab_top_border_uses_top_right_corner() {
        let label = vertical_label("Tab");
        let label = label.as_str();
        let width = vertical_rail_width(&[label]);
        let height = vertical_tab_height(label);
        let buf = render_vertical(&[label], 0, width, height);
        let top_line = line_str(&buf, 0);

        assert!(top_line.starts_with('╭'));
        assert!(top_line.ends_with('╯'));
    }

    #[test]
    fn vertical_active_tab_opens_right() {
        let label = vertical_label("Tab");
        let label = label.as_str();
        let width = vertical_rail_width(&[label]);
        let height = vertical_tab_height(label);
        let buf = render_vertical(&[label], 0, width, height);
        let right_col = col_str(&buf, width - 1);
        let glyphs: Vec<char> = right_col.chars().collect();

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
        let width = vertical_rail_width(&[first, second]);
        let height = vertical_tab_height(first) + vertical_tab_height(second);
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
        let width = vertical_rail_width(&[tall, also]);
        let height = vertical_tab_height(tall);
        let buf = render_vertical(&[tall, also], 0, width, height);
        let col = col_str(&buf, 4);

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
