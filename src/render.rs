//! Copyright © 2026 ComfyHome™
//! All rights reserved.
//!
//! Licensed under the ComfyGit SA-PS:DA License
//!
//! For details, see the LICENSE file in the repository root.

use ratatui_core::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    symbols,
    widgets::{StatefulWidget, Widget},
};

use crate::config::{TabBarEnd, TabOrientation, TabPadding};
use crate::layout::{
    auto_vertical_tab_height, char_display_width, compute_viewport, effective_indicator,
    effective_margin, effective_padding, effective_tab_bar_end, horizontal_strip_height,
    label_char, label_origin, vertical_rail_width, TabViewport,
};
use crate::nav::TabNav;
use crate::state::TabNavState;
use crate::TAB_BORDER;

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
    let strip_height = horizontal_strip_height(nav);

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
    let rail_width = vertical_rail_width(nav).min(area.width);

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
