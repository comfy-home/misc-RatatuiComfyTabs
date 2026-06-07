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
use crate::config::{
    HorizontalPosition, TabBarAlign, TabBarEnd, TabOrientation, TabPadding, VerticalPosition,
};
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

    apply_horizontal_tab_bar_end(
        ApplyHorizontalTabBarEndArgs {
            flow_start: content_left,
            flow_end: content_right,
            baseline_y,
            end_style: effective_tab_bar_end(nav),
            align: nav.tab_bar_align,
            viewport: &viewport,
            selected,
            opens_down,
            style: bs,
        },
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

    for entry in &positions.entries {
        let label = nav.tabs[entry.index];
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

    if !positions.entries.is_empty() {
        apply_vertical_tab_bar_end(
            ApplyVerticalTabBarEndArgs {
                content_top,
                content_bottom,
                baseline_x,
                end_style: effective_tab_bar_end(nav),
                align: nav.tab_bar_align,
                viewport: &positions,
                selected,
                opens_right,
                style: bs,
            },
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

fn mirror_cap_vertically(symbol: &'static str) -> &'static str {
    match symbol {
        "┌" => "└",
        "└" => "┌",
        "┐" => "┘",
        "┘" => "┐",
        "╭" => "╰",
        "╰" => "╭",
        "╮" => "╯",
        "╯" => "╮",
        "┬" => "┴",
        "┴" => "┬",
        other => other,
    }
}

fn mirror_cap_horizontally(symbol: &'static str) -> &'static str {
    match symbol {
        "├" => "┤",
        "┤" => "├",
        "┐" => "┌",
        "┌" => "┐",
        "└" => "┘",
        "┘" => "└",
        "╮" => "╭",
        "╭" => "╮",
        "╯" => "╰",
        "╰" => "╯",
        "┬" => "┴",
        "┴" => "┬",
        other => other,
    }
}

fn horizontal_tab_trailing_junction(last_visible_selected: bool) -> &'static str {
    if last_visible_selected {
        "│"
    } else {
        "┤"
    }
}

fn horizontal_margin_trailing_cap(end_style: TabBarEnd, opens_down: bool) -> Option<&'static str> {
    match (end_style, opens_down) {
        (TabBarEnd::Sqr, true) => Some("┐"),
        (TabBarEnd::Rnd, true) => Some("╮"),
        (TabBarEnd::Sqr, false) => Some("┘"),
        (TabBarEnd::Rnd, false) => Some("╯"),
        (TabBarEnd::NoEnd, _) => None,
    }
}

fn horizontal_start_tab_bar_end_caps(
    end_style: TabBarEnd,
    opens_down: bool,
    first_visible_selected: bool,
) -> Option<(&'static str, &'static str)> {
    if end_style == TabBarEnd::NoEnd {
        return None;
    }
    let trailing = match (end_style, opens_down) {
        (TabBarEnd::Sqr, true) => "┐",
        (TabBarEnd::Rnd, true) => "╮",
        (TabBarEnd::Sqr, false) => "┘",
        (TabBarEnd::Rnd, false) => "╯",
        (TabBarEnd::NoEnd, _) => return None,
    };
    let leading = if first_visible_selected { "│" } else { "├" };
    Some((leading, trailing))
}

struct ApplyHorizontalTabBarEndArgs<'a> {
    flow_start: u16,
    flow_end: u16,
    baseline_y: u16,
    end_style: TabBarEnd,
    align: TabBarAlign,
    viewport: &'a TabViewport,
    selected: usize,
    opens_down: bool,
    style: Style,
}

struct ApplyVerticalTabBarEndArgs<'a> {
    content_top: u16,
    content_bottom: u16,
    baseline_x: u16,
    end_style: TabBarEnd,
    align: TabBarAlign,
    viewport: &'a TabViewport,
    selected: usize,
    opens_right: bool,
    style: Style,
}

fn horizontal_tab_bar_end_caps(
    end_style: TabBarEnd,
    align: TabBarAlign,
    viewport: &TabViewport,
    selected: usize,
    opens_down: bool,
) -> Option<(&'static str, &'static str)> {
    let first_visible_selected = viewport
        .entries
        .first()
        .is_some_and(|entry| entry.index == selected);
    let last_visible_selected = viewport
        .entries
        .last()
        .is_some_and(|entry| entry.index == selected);
    let (mut leading, mut trailing) =
        horizontal_start_tab_bar_end_caps(end_style, opens_down, first_visible_selected)?;

    match align {
        TabBarAlign::Start => {}
        TabBarAlign::Center => {
            leading = mirror_cap_horizontally(trailing);
        }
        TabBarAlign::End => {
            leading = mirror_cap_horizontally(trailing);
            trailing = if last_visible_selected { "│" } else { "┤" };
        }
    }

    Some((leading, trailing))
}

fn apply_horizontal_tab_bar_end(args: ApplyHorizontalTabBarEndArgs, buf: &mut Buffer) {
    if args.flow_end <= args.flow_start {
        return;
    }
    let Some((group_start, group_end)) = args.viewport.group_bounds() else {
        return;
    };
    let exact_fit = group_start == args.flow_start && group_end == args.flow_end;
    let cap_align = if exact_fit {
        TabBarAlign::Start
    } else {
        args.align
    };
    let Some((leading, _aligned_trailing)) = horizontal_tab_bar_end_caps(
        args.end_style,
        cap_align,
        args.viewport,
        args.selected,
        args.opens_down,
    ) else {
        return;
    };

    let last_visible_selected = args
        .viewport
        .entries
        .last()
        .is_some_and(|entry| entry.index == args.selected);
    let trailing_in_slack = group_end < args.flow_end;
    let tab_junction = horizontal_tab_trailing_junction(last_visible_selected);

    buf[(args.flow_start, args.baseline_y)]
        .set_symbol(leading)
        .set_style(args.style);

    if trailing_in_slack {
        if let Some(margin_cap) = horizontal_margin_trailing_cap(args.end_style, args.opens_down) {
            buf[(args.flow_end - 1, args.baseline_y)]
                .set_symbol(margin_cap)
                .set_style(args.style);
        }
    } else {
        buf[(group_end - 1, args.baseline_y)]
            .set_symbol(tab_junction)
            .set_style(args.style);
    }
}

fn vertical_tab_leading_junction(first_visible_active: bool) -> &'static str {
    if first_visible_active {
        "─"
    } else {
        "┬"
    }
}

fn vertical_tab_trailing_junction(last_visible_active: bool) -> &'static str {
    if last_visible_active {
        "─"
    } else {
        "┴"
    }
}

fn vertical_margin_trailing_cap(end_style: TabBarEnd, opens_right: bool) -> Option<&'static str> {
    let cap = match (end_style, opens_right) {
        (TabBarEnd::Sqr, true) => "└",
        (TabBarEnd::Rnd, true) => "╰",
        (TabBarEnd::Sqr, false) => "┘",
        (TabBarEnd::Rnd, false) => "╯",
        (TabBarEnd::NoEnd, _) => return None,
    };
    Some(cap)
}

fn mirror_cap_for_vertical_rail(symbol: &'static str, opens_right: bool) -> &'static str {
    if opens_right {
        symbol
    } else {
        mirror_cap_horizontally(symbol)
    }
}

fn vertical_tab_bar_junctions(
    first_visible_active: bool,
    last_visible_active: bool,
) -> (&'static str, &'static str) {
    (
        vertical_tab_leading_junction(first_visible_active),
        vertical_tab_trailing_junction(last_visible_active),
    )
}

fn vertical_tab_bar_margin_caps(
    end_style: TabBarEnd,
    opens_right: bool,
) -> (Option<&'static str>, Option<&'static str>) {
    let trailing = vertical_margin_trailing_cap(end_style, true);
    let leading = trailing.map(mirror_cap_vertically);
    (
        leading.map(|cap| mirror_cap_for_vertical_rail(cap, opens_right)),
        trailing.map(|cap| mirror_cap_for_vertical_rail(cap, opens_right)),
    )
}

fn apply_vertical_tab_bar_end(args: ApplyVerticalTabBarEndArgs, buf: &mut Buffer) {
    if args.content_bottom == 0 {
        return;
    }
    let Some((group_start, group_end)) = args.viewport.group_bounds() else {
        return;
    };
    if args.end_style == TabBarEnd::NoEnd {
        return;
    }

    let first_visible_active = args
        .viewport
        .entries
        .first()
        .is_some_and(|entry| entry.index == args.selected);
    let last_visible_active = args
        .viewport
        .entries
        .last()
        .is_some_and(|entry| entry.index == args.selected);

    let exact_fit = group_start == args.content_top && group_end == args.content_bottom;
    let cap_align = if exact_fit {
        TabBarAlign::Start
    } else {
        args.align
    };
    let trailing_in_slack = group_end < args.content_bottom;
    let leading_in_slack = group_start > args.content_top;
    let (leading_junction, trailing_junction) = vertical_tab_bar_junctions(
        first_visible_active,
        last_visible_active,
    );
    let (margin_leading, margin_trailing) =
        vertical_tab_bar_margin_caps(args.end_style, args.opens_right);

    if exact_fit {
        if let Some(first) = args.viewport.entries.first() {
            buf[(args.baseline_x, first.offset)]
                .set_symbol(leading_junction)
                .set_style(args.style);
        }
        if let Some(last) = args.viewport.entries.last() {
            buf[(args.baseline_x, last.offset + last.size - 1)]
                .set_symbol(trailing_junction)
                .set_style(args.style);
        }
        return;
    }

    match cap_align {
        TabBarAlign::Start => {
            if let Some(first) = args.viewport.entries.first() {
                buf[(args.baseline_x, first.offset)]
                    .set_symbol(leading_junction)
                    .set_style(args.style);
            }
            if trailing_in_slack {
                if let Some(cap) = margin_trailing {
                    buf[(args.baseline_x, args.content_bottom - 1)]
                        .set_symbol(cap)
                        .set_style(args.style);
                }
            } else if let Some(last) = args.viewport.entries.last() {
                buf[(args.baseline_x, last.offset + last.size - 1)]
                    .set_symbol(trailing_junction)
                    .set_style(args.style);
            }
        }
        TabBarAlign::Center => {
            if leading_in_slack {
                if let Some(cap) = margin_leading {
                    buf[(args.baseline_x, args.content_top)]
                        .set_symbol(cap)
                        .set_style(args.style);
                }
            }
            if trailing_in_slack {
                if let Some(cap) = margin_trailing {
                    buf[(args.baseline_x, args.content_bottom - 1)]
                        .set_symbol(cap)
                        .set_style(args.style);
                }
            }
        }
        TabBarAlign::End => {
            if leading_in_slack {
                if let Some(cap) = margin_leading {
                    buf[(args.baseline_x, args.content_top)]
                        .set_symbol(cap)
                        .set_style(args.style);
                }
            }
            if let Some(last) = args.viewport.entries.last() {
                buf[(args.baseline_x, last.offset + last.size - 1)]
                    .set_symbol(trailing_junction)
                    .set_style(args.style);
            }
        }
    }
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
