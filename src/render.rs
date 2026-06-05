//! Copyright © 2026 ComfyHome™
//! All rights reserved.
//!
//! Licensed under the ComfyGit SA-PS:DA License
//!
//! For details, see the LICENSE file in the repository root.

use ratatui_core::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    symbols,
    widgets::{StatefulWidget, Widget},
};

use crate::TAB_BORDER;
use crate::config::{HorizontalPosition, TabBarEnd, TabOrientation, TabPadding, VerticalPosition};
use crate::layout::{
    TabViewport, auto_vertical_tab_height, char_display_width, compute_viewport,
    effective_indicator, effective_margin, effective_padding, effective_tab_bar_end,
    horizontal_strip_height, horizontal_strip_origin_y, label_char, label_origin,
    vertical_rail_origin_x, vertical_rail_width,
};
use crate::nav::TabNav;
use crate::state::TabNavState;

impl Widget for TabNav<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let selected = self.selected;
        let scroll_offset = self.scroll_offset;
        render_tab_nav(&self, area, buf, selected, scroll_offset, None, None);
    }
}

impl StatefulWidget for TabNav<'_> {
    type State = TabNavState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.tick_selection_flash();
        render_tab_nav(
            &self,
            area,
            buf,
            state.selected,
            state.scroll_offset,
            state.reorder_drag,
            Some(state),
        );
    }
}

fn render_tab_nav(
    nav: &TabNav<'_>,
    area: Rect,
    buf: &mut Buffer,
    selected: usize,
    scroll_offset: usize,
    reorder_drag: Option<crate::state::TabReorderDrag>,
    nav_state: Option<&TabNavState>,
) {
    if nav.tabs.is_empty() {
        return;
    }

    match nav.orientation {
        TabOrientation::Horizontal => render_horizontal(
            nav,
            area,
            buf,
            selected,
            scroll_offset,
            reorder_drag,
            nav_state,
        ),
        TabOrientation::Vertical => render_vertical(
            nav,
            area,
            buf,
            selected,
            scroll_offset,
            reorder_drag,
            nav_state,
        ),
    }
}

fn render_horizontal(
    nav: &TabNav<'_>,
    area: Rect,
    buf: &mut Buffer,
    selected: usize,
    scroll_offset: usize,
    reorder_drag: Option<crate::state::TabReorderDrag>,
    nav_state: Option<&TabNavState>,
) {
    let margin = effective_margin(nav);
    let pad = effective_padding(nav);
    let strip_height = horizontal_strip_height(nav);

    if area.height < strip_height || area.width <= margin.start + margin.end {
        return;
    }

    let border = &nav.border_set;
    let bs = nav.border_style;

    let content_left = area.x + margin.start;
    let content_right = area.right() - margin.end;
    let top_y = horizontal_strip_origin_y(nav, area);
    let bot_y = top_y + strip_height - 1;
    let label_y = top_y + TAB_BORDER + pad.top;
    let opens_down = nav.horizontal_position == HorizontalPosition::Top;
    let baseline_y = if opens_down { bot_y } else { top_y };

    draw_horizontal_baseline(content_left, content_right, baseline_y, border, bs, buf);

    let viewport = compute_viewport(nav, area, scroll_offset);

    for entry in &viewport.entries {
        let label = nav.tabs[entry.index];
        let active = entry.index == selected;
        let dragging = is_reorder_drag_source(entry.index, reorder_drag);
        let selection_flash =
            nav_state.is_some_and(|state| state.selection_flash_border_on(entry.index));
        let left_x = entry.offset;
        let right_x = entry.offset + entry.size - 1;
        let text_style = tab_text_style(nav, active, dragging);
        let tab_border_style = tab_border_style(nav, bs, dragging, selection_flash);

        if opens_down {
            draw_top_border(left_x, right_x, top_y, border, tab_border_style, buf);
        }
        draw_horizontal_side_borders(left_x, right_x, top_y, bot_y, border, tab_border_style, buf);
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
            if opens_down {
                draw_active_bottom(left_x, right_x, bot_y, border, tab_border_style, buf);
            } else {
                draw_active_top(left_x, right_x, top_y, border, tab_border_style, buf);
                draw_bottom_border(left_x, right_x, bot_y, border, tab_border_style, buf);
            }
        } else if opens_down {
            draw_inactive_horizontal_bottom(left_x, right_x, bot_y, tab_border_style, buf);
        } else {
            draw_inactive_horizontal_top(left_x, right_x, top_y, tab_border_style, buf);
            draw_bottom_border(left_x, right_x, bot_y, border, tab_border_style, buf);
        }
    }

    draw_horizontal_overflow_affordances(&viewport, baseline_y, bs, buf);

    let first_visible_selected = viewport
        .entries
        .first()
        .is_some_and(|entry| entry.index == selected);

    apply_horizontal_tab_bar_end(
        content_left,
        content_right,
        baseline_y,
        effective_tab_bar_end(nav),
        first_visible_selected,
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
    reorder_drag: Option<crate::state::TabReorderDrag>,
    nav_state: Option<&TabNavState>,
) {
    let margin = effective_margin(nav);
    let pad = effective_padding(nav);
    let rail_width = vertical_rail_width(nav).min(area.width);

    if rail_width < TAB_BORDER * 2 + pad.left + pad.right
        || area.height <= margin.start + margin.end
    {
        return;
    }

    let border = &nav.border_set;
    let bs = nav.border_style;
    let left_x = vertical_rail_origin_x(nav, area);
    let right_x = left_x + rail_width - 1;
    let content_top = area.y + margin.start;
    let content_bottom = area.bottom() - margin.end;
    let opens_right = nav.vertical_position == VerticalPosition::Left;
    let baseline_x = if opens_right { right_x } else { left_x };

    draw_vertical_baseline(baseline_x, content_top, content_bottom, border, bs, buf);

    let positions = compute_viewport(nav, area, scroll_offset);
    let mut first_rendered: Option<(usize, u16)> = None;

    for entry in &positions.entries {
        let label = nav.tabs[entry.index];
        if first_rendered.is_none() {
            first_rendered = Some((entry.index, entry.offset));
        }
        let active = entry.index == selected;
        let dragging = is_reorder_drag_source(entry.index, reorder_drag);
        let selection_flash =
            nav_state.is_some_and(|state| state.selection_flash_border_on(entry.index));
        let top_y = entry.offset;
        let bot_y = entry.offset + entry.size - 1;
        let text_style = tab_text_style(nav, active, dragging);
        let tab_border_style = tab_border_style(nav, bs, dragging, selection_flash);

        draw_top_border(left_x, right_x, top_y, border, tab_border_style, buf);
        if opens_right {
            draw_vertical_side_borders(
                left_x,
                right_x,
                top_y,
                bot_y,
                border,
                tab_border_style,
                buf,
            );
        } else {
            draw_vertical_side_borders_right(
                left_x,
                right_x,
                top_y,
                bot_y,
                border,
                tab_border_style,
                buf,
            );
        }
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
        draw_bottom_border(left_x, right_x, bot_y, border, tab_border_style, buf);

        if active {
            if let Some(sym) = effective_indicator(nav) {
                let (label_x, label_y) = label_origin(left_x, top_y, pad);
                if label_y > top_y + TAB_BORDER {
                    buf[(label_x, label_y - 1)]
                        .set_symbol(sym)
                        .set_style(text_style);
                }
            }
            if opens_right {
                draw_active_right(left_x, right_x, top_y, bot_y, border, tab_border_style, buf);
            } else {
                draw_active_left(left_x, right_x, top_y, bot_y, border, tab_border_style, buf);
            }
        } else if opens_right {
            draw_inactive_vertical_right(left_x, right_x, top_y, bot_y, tab_border_style, buf);
        } else {
            draw_inactive_vertical_left(left_x, right_x, top_y, bot_y, tab_border_style, buf);
        }
    }

    draw_vertical_overflow_affordances(&positions, baseline_x, bs, buf);

    if let Some((first_index, first_top)) = first_rendered {
        apply_vertical_tab_bar_end(
            first_index == selected,
            first_top,
            baseline_x,
            content_bottom,
            effective_tab_bar_end(nav),
            opens_right,
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

fn is_reorder_drag_source(
    tab_index: usize,
    reorder_drag: Option<crate::state::TabReorderDrag>,
) -> bool {
    reorder_drag.is_some_and(|drag| drag.armed && drag.source == tab_index)
}

/// Default drag highlight: ANSI indexed foreground **46**.
pub(crate) fn default_reorder_drag_style() -> Style {
    Style::new().fg(Color::Indexed(46))
}

/// Default selection-flash border: ANSI indexed foreground **46**.
pub(crate) fn default_selection_flash_style() -> Style {
    Style::new().fg(Color::Indexed(46))
}

fn effective_reorder_drag_style(nav: &TabNav<'_>) -> Style {
    nav.reorder_drag_style
        .unwrap_or_else(default_reorder_drag_style)
}

fn effective_selection_flash_style(nav: &TabNav<'_>) -> Style {
    nav.selection_flash_style
        .unwrap_or_else(default_selection_flash_style)
}

fn tab_border_style(nav: &TabNav<'_>, base: Style, dragging: bool, selection_flash: bool) -> Style {
    if dragging {
        let drag = effective_reorder_drag_style(nav);
        if let Some(fg) = drag.fg {
            return base.fg(fg);
        }
        return base;
    }
    if nav.selection_flash_enabled && selection_flash {
        let flash = effective_selection_flash_style(nav);
        if let Some(fg) = flash.fg {
            return base.fg(fg);
        }
    }
    base
}

fn tab_text_style(nav: &TabNav<'_>, active: bool, dragging: bool) -> Style {
    if dragging {
        let mut style = effective_reorder_drag_style(nav);
        if nav.highlight_bold || active {
            style = style.add_modifier(Modifier::BOLD);
        }
        return style;
    }
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
    first_visible_selected: bool,
    style: Style,
    buf: &mut Buffer,
) {
    if end_style == TabBarEnd::NoEnd || end <= start {
        return;
    }

    let (left_inactive, right_cap) = match end_style {
        TabBarEnd::NoEnd => return,
        TabBarEnd::Sqr => ("├", "┐"),
        TabBarEnd::Rnd => ("├", "╮"),
    };
    let left_cap = if first_visible_selected {
        "│"
    } else {
        left_inactive
    };

    buf[(start, y)].set_symbol(left_cap).set_style(style);
    buf[(end - 1, y)].set_symbol(right_cap).set_style(style);
}

fn apply_vertical_tab_bar_end(
    first_active: bool,
    first_top: u16,
    baseline_x: u16,
    content_bottom: u16,
    end_style: TabBarEnd,
    opens_right: bool,
    style: Style,
    buf: &mut Buffer,
) {
    if end_style == TabBarEnd::NoEnd || content_bottom == 0 {
        return;
    }

    let top_junction = if first_active {
        "─"
    } else if opens_right {
        "┬"
    } else {
        "┴"
    };
    buf[(baseline_x, first_top)]
        .set_symbol(top_junction)
        .set_style(style);

    let bottom_cap = match (end_style, opens_right) {
        (TabBarEnd::Sqr, true) => "└",
        (TabBarEnd::Rnd, true) => "╰",
        (TabBarEnd::Sqr, false) => "┘",
        (TabBarEnd::Rnd, false) => "╯",
        (TabBarEnd::NoEnd, _) => return,
    };
    buf[(baseline_x, content_bottom - 1)]
        .set_symbol(bottom_cap)
        .set_style(style);
}

fn draw_vertical_baseline(
    baseline_x: u16,
    start_y: u16,
    end_y: u16,
    border: &symbols::border::Set,
    style: Style,
    buf: &mut Buffer,
) {
    for y in start_y..end_y {
        buf[(baseline_x, y)]
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

fn draw_vertical_side_borders_right(
    _left: u16,
    right: u16,
    top: u16,
    bottom: u16,
    border: &symbols::border::Set,
    style: Style,
    buf: &mut Buffer,
) {
    for y in (top + 1)..bottom {
        buf[(right, y)]
            .set_symbol(border.vertical_right)
            .set_style(style);
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

fn draw_active_top(
    left: u16,
    right: u16,
    y: u16,
    border: &symbols::border::Set,
    style: Style,
    buf: &mut Buffer,
) {
    buf[(left, y)].set_symbol(border.top_right).set_style(style);

    for x in (left + 1)..right {
        buf[(x, y)].set_symbol(" ").set_style(style);
    }

    buf[(right, y)].set_symbol(border.top_left).set_style(style);
}

fn draw_inactive_horizontal_bottom(left: u16, right: u16, y: u16, style: Style, buf: &mut Buffer) {
    buf[(left, y)].set_symbol("┴").set_style(style);
    buf[(right, y)].set_symbol("┴").set_style(style);
}

fn draw_inactive_horizontal_top(left: u16, right: u16, y: u16, style: Style, buf: &mut Buffer) {
    buf[(left, y)].set_symbol("┬").set_style(style);
    buf[(right, y)].set_symbol("┬").set_style(style);
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

fn draw_active_left(
    left: u16,
    _right: u16,
    top: u16,
    bottom: u16,
    border: &symbols::border::Set,
    style: Style,
    buf: &mut Buffer,
) {
    buf[(left, top)]
        .set_symbol(border.bottom_left)
        .set_style(style);

    for y in (top + 1)..bottom {
        buf[(left, y)].set_symbol(" ").set_style(style);
    }

    buf[(left, bottom)]
        .set_symbol(border.top_left)
        .set_style(style);
}

fn draw_inactive_vertical_left(
    left: u16,
    _right: u16,
    top: u16,
    bottom: u16,
    style: Style,
    buf: &mut Buffer,
) {
    buf[(left, top)].set_symbol("├").set_style(style);
    buf[(left, bottom)].set_symbol("├").set_style(style);
}
