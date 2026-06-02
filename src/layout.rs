//! Copyright © 2026 ComfyHome™
//! All rights reserved.
//!
//! Licensed under the ComfyGit SA-PS:DA License
//!
//! For details, see the LICENSE file in the repository root.

use ratatui_core::layout::Rect;
use unicode_width::UnicodeWidthChar;

use crate::config::{OverflowPolicy, TabBarEnd, TabMargin, TabOrientation, TabPadding};
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
                y: area.y,
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
                x: area.x,
                y: entry.offset,
                width: rail_width,
                height: entry.size,
            })
        }
    }
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
