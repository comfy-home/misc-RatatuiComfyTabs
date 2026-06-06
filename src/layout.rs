//! Copyright © 2026 ComfyHome™
//! All rights reserved.
//!
//! Licensed under the ComfyGit SA-PS:DA License
//!
//! For details, see the LICENSE file in the repository root.

use ratatui_core::layout::Rect;
use unicode_width::UnicodeWidthChar;

use crate::config::{
    HorizontalPosition, OverflowPolicy, TabBarAlign, TabBarEnd, TabMargin, TabOrientation,
    TabPadding, VerticalPosition,
};
use crate::nav::TabNav;
use crate::{DEFAULT_INDICATOR, TAB_BORDER};

pub(crate) fn effective_margin(nav: &TabNav<'_>) -> TabMargin {
    nav.margin.unwrap_or(match nav.orientation {
        TabOrientation::Horizontal => TabMargin::ZERO,
        TabOrientation::Vertical => TabMargin::vertical_default(),
    })
}

pub(crate) fn effective_padding(nav: &TabNav<'_>) -> TabPadding {
    nav.padding.unwrap_or(match nav.orientation {
        TabOrientation::Horizontal => TabPadding::horizontal_default(),
        TabOrientation::Vertical => TabPadding::vertical_default(),
    })
}

pub(crate) fn effective_tab_bar_end(nav: &TabNav<'_>) -> TabBarEnd {
    nav.tab_bar_end.unwrap_or(TabBarEnd::NoEnd)
}

fn label_line_count(label: &str) -> u16 {
    if label.is_empty() {
        0
    } else {
        label.lines().count() as u16
    }
}

pub(crate) fn label_char(ch: char, all_caps: bool) -> char {
    if all_caps {
        ch.to_uppercase().next().unwrap_or(ch)
    } else {
        ch
    }
}

pub(crate) fn char_display_width(ch: char, all_caps: bool) -> u16 {
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

pub(crate) fn auto_horizontal_tab_width(label: &str, pad: TabPadding, all_caps: bool) -> u16 {
    TAB_BORDER * 2 + pad.left + label_display_width(label, all_caps) + pad.right
}

pub(crate) fn auto_vertical_tab_height(label: &str, pad: TabPadding) -> u16 {
    TAB_BORDER * 2 + pad.top + label_line_count(label) + pad.bottom
}

pub(crate) fn horizontal_strip_height(nav: &TabNav<'_>) -> u16 {
    let pad = effective_padding(nav);
    TAB_BORDER * 2 + pad.top + 1 + pad.bottom
}

pub(crate) fn horizontal_strip_origin_y(nav: &TabNav<'_>, area: Rect) -> u16 {
    let strip_height = horizontal_strip_height(nav);
    match nav.horizontal_position {
        HorizontalPosition::Top => area.y,
        HorizontalPosition::Bottom => area.bottom().saturating_sub(strip_height),
    }
}

pub(crate) fn vertical_rail_origin_x(nav: &TabNav<'_>, area: Rect) -> u16 {
    let rail_width = vertical_rail_width(nav).min(area.width);
    match nav.vertical_position {
        VerticalPosition::Left => area.x,
        VerticalPosition::Right => area.right().saturating_sub(rail_width),
    }
}

pub(crate) fn vertical_rail_width(nav: &TabNav<'_>) -> u16 {
    let pad = effective_padding(nav);
    nav.tabs
        .iter()
        .map(|label| auto_horizontal_tab_width(label, pad, nav.all_caps))
        .max()
        .unwrap_or_else(|| auto_horizontal_tab_width("", pad, nav.all_caps))
}

fn primary_tab_size(nav: &TabNav<'_>, index: usize, label: &str, pad: TabPadding) -> u16 {
    nav.tab_sizes
        .and_then(|sizes| sizes.get(index).copied())
        .unwrap_or_else(|| match nav.orientation {
            TabOrientation::Horizontal => auto_horizontal_tab_width(label, pad, nav.all_caps),
            TabOrientation::Vertical => auto_vertical_tab_height(label, pad),
        })
}

pub(crate) struct TabEntry {
    pub(crate) index: usize,
    pub(crate) offset: u16,
    pub(crate) size: u16,
}

pub(crate) struct TabViewport {
    pub(crate) entries: Vec<TabEntry>,
    pub(crate) clipped_before: bool,
    pub(crate) clipped_after: bool,
    pub(crate) before_affordance_at: Option<u16>,
    pub(crate) after_affordance_at: Option<u16>,
}

impl TabViewport {
    /// Primary-axis span of visible tabs: `(group_start, group_end)` with `group_end` exclusive.
    pub(crate) fn group_bounds(&self) -> Option<(u16, u16)> {
        let first = self.entries.first()?;
        let last = self.entries.last()?;
        Some((
            first.offset,
            last.offset.saturating_add(last.size),
        ))
    }
}

fn flow_bounds(nav: &TabNav<'_>, area: Rect) -> Option<(u16, u16)> {
    if nav.tabs.is_empty() {
        return None;
    }

    let margin = effective_margin(nav);
    match nav.orientation {
        TabOrientation::Horizontal => {
            let strip_height = horizontal_strip_height(nav);
            if area.height < strip_height || area.width <= margin.start + margin.end {
                return None;
            }
            Some((area.x + margin.start, area.right() - margin.end))
        }
        TabOrientation::Vertical => {
            let rail_width = vertical_rail_width(nav).min(area.width);
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

/// Layout [`Rect`] for one visible tab (same geometry as [`TabNav::tab_rects`](crate::TabNav::tab_rects)).
pub(crate) fn tab_entry_rect(nav: &TabNav<'_>, area: Rect, entry: &TabEntry) -> Option<Rect> {
    let margin = effective_margin(nav);
    let pad = effective_padding(nav);
    match nav.orientation {
        TabOrientation::Horizontal => {
            let strip_height = horizontal_strip_height(nav);
            if area.height < strip_height || area.width <= margin.start + margin.end {
                return None;
            }
            Some(Rect {
                x: entry.offset,
                y: horizontal_strip_origin_y(nav, area),
                width: entry.size,
                height: strip_height,
            })
        }
        TabOrientation::Vertical => {
            let rail_width = vertical_rail_width(nav).min(area.width);
            if rail_width < TAB_BORDER * 2 + pad.left + pad.right
                || area.height <= margin.start + margin.end
            {
                return None;
            }
            Some(Rect {
                x: vertical_rail_origin_x(nav, area),
                y: entry.offset,
                width: rail_width,
                height: entry.size,
            })
        }
    }
}

fn tab_fits_before_end(pos: u16, size: u16, flow_end: u16) -> bool {
    pos.saturating_add(size) <= flow_end
}

fn reserve_scroll_affordance(
    pos: u16,
    size: u16,
    flow_end: u16,
    has_more: bool,
    nav: &TabNav<'_>,
) -> bool {
    if nav.overflow != OverflowPolicy::Scroll || !has_more || !nav.overflow_affordance {
        return tab_fits_before_end(pos, size, flow_end);
    }
    pos.saturating_add(size).saturating_add(1) <= flow_end
}

fn align_shift(
    align: TabBarAlign,
    align_start: u16,
    align_end: u16,
    group_start: u16,
    group_end: u16,
) -> u16 {
    let align_span = align_end.saturating_sub(align_start);
    let group_span = group_end.saturating_sub(group_start);
    if group_span >= align_span {
        return 0;
    }
    match align {
        TabBarAlign::Start => 0,
        TabBarAlign::Center => {
            let slack = align_span - group_span;
            align_start
                .saturating_add(slack / 2)
                .saturating_sub(group_start)
        }
        TabBarAlign::End => align_end.saturating_sub(group_end),
    }
}

fn build_forward_entries(
    nav: &TabNav<'_>,
    pad: TabPadding,
    first_index: usize,
    content_start: u16,
    flow_end: u16,
) -> Vec<TabEntry> {
    let total = nav.tabs.len();
    let mut entries = Vec::with_capacity(total);
    let mut pos = content_start;

    for index in first_index..total {
        let size = primary_tab_size(nav, index, nav.tabs[index], pad);
        let has_more = index + 1 < total;

        if !tab_fits_before_end(pos, size, flow_end) {
            break;
        }

        if !reserve_scroll_affordance(pos, size, flow_end, has_more, nav) {
            break;
        }

        entries.push(TabEntry {
            index,
            offset: pos,
            size,
        });
        pos = pos.saturating_add(size);
    }

    entries
}

fn build_backward_entries(
    nav: &TabNav<'_>,
    pad: TabPadding,
    flow_start: u16,
    flow_end: u16,
) -> Vec<TabEntry> {
    let total = nav.tabs.len();
    let mut entries = Vec::with_capacity(total);
    let mut pos = flow_end;

    for index in (0..total).rev() {
        let size = primary_tab_size(nav, index, nav.tabs[index], pad);
        let Some(next_start) = pos.checked_sub(size) else {
            break;
        };
        if next_start < flow_start {
            break;
        }
        entries.push(TabEntry {
            index,
            offset: next_start,
            size,
        });
        pos = next_start;
    }

    entries.reverse();
    entries
}

pub(crate) fn compute_viewport(nav: &TabNav<'_>, area: Rect, scroll_offset: usize) -> TabViewport {
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

    let mut entries = build_forward_entries(nav, pad, first_index, content_start, flow_end);
    let mut clipped_after = entries.last().is_some_and(|entry| entry.index + 1 < total);
    let mut end_packed = false;

    if nav.tab_bar_align == TabBarAlign::End
        && nav.overflow == OverflowPolicy::Truncate
        && clipped_after
    {
        entries = build_backward_entries(nav, pad, flow_start, flow_end);
        clipped_before = entries.first().is_some_and(|entry| entry.index > 0);
        clipped_after = false;
        end_packed = true;
    }

    if !end_packed && let (Some(first), Some(last)) = (entries.first(), entries.last()) {
        let group_start = first.offset;
        let group_end = last.offset.saturating_add(last.size);
        let align_start = content_start;
        let align_end =
            flow_end.saturating_sub(usize::from(clipped_after && nav.overflow_affordance) as u16);
        let shift = align_shift(
            nav.tab_bar_align,
            align_start,
            align_end,
            group_start,
            group_end,
        );
        if shift > 0 {
            for entry in &mut entries {
                entry.offset = entry.offset.saturating_add(shift);
            }
        }
    }

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

pub(crate) fn effective_indicator<'a>(nav: &TabNav<'a>) -> Option<&'a str> {
    if nav.indicator_explicit {
        nav.indicator
    } else if nav.orientation == TabOrientation::Vertical {
        None
    } else {
        Some(DEFAULT_INDICATOR)
    }
}

pub(crate) fn label_origin(left: u16, top: u16, pad: TabPadding) -> (u16, u16) {
    (left + TAB_BORDER + pad.left, top + TAB_BORDER + pad.top)
}
