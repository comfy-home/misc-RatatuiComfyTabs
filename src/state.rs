//! Copyright © 2026 ComfyHome™
//! All rights reserved.
//!
//! Licensed under the ComfyGit SA-PS:DA License
//!
//! For details, see the LICENSE file in the repository root.

use ratatui_core::layout::Rect;

use crate::config::{OverflowPolicy, TabDirection, TabWheelDirection};
use crate::layout::compute_viewport;
use crate::nav::TabNav;

/// Mutable tab selection and scroll state for [`StatefulWidget`](ratatui_core::widgets::StatefulWidget) rendering.
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

    /// Cycles tabs when the pointer is over `strip_area` and [`TabNav::mouse_wheel`] is enabled.
    ///
    /// Returns `true` when the wheel event was consumed.
    pub fn handle_mouse_wheel(
        &mut self,
        nav: &TabNav<'_>,
        strip_area: Rect,
        mouse_column: u16,
        mouse_row: u16,
        direction: TabWheelDirection,
    ) -> bool {
        if !nav.mouse_wheel || nav.tabs.is_empty() {
            return false;
        }

        if !nav.wheel_hover(strip_area, self.scroll_offset, mouse_column, mouse_row) {
            return false;
        }

        self.select_direction(direction.tab_direction(), nav.tabs.len());
        if nav.overflow == OverflowPolicy::Scroll {
            self.ensure_selected_visible(nav, strip_area);
        }
        true
    }

    /// Selects the tab under the pointer when [`TabNav::mouse_click`] is enabled.
    ///
    /// Pass the same `area` used to render the tab strip. Returns `true` when a tab was selected.
    pub fn handle_mouse_click(
        &mut self,
        nav: &TabNav<'_>,
        area: Rect,
        mouse_column: u16,
        mouse_row: u16,
    ) -> bool {
        if !nav.mouse_click || nav.tabs.is_empty() {
            return false;
        }

        let Some(index) = nav.tab_index_at(area, self.scroll_offset, mouse_column, mouse_row)
        else {
            return false;
        };

        self.select(index, nav.tabs.len());
        if nav.overflow == OverflowPolicy::Scroll {
            self.ensure_selected_visible(nav, area);
        }
        true
    }
}
