//! Copyright © 2026 ComfyHome™
//! All rights reserved.
//!
//! Licensed under the ComfyGit SA-PS:DA License
//!
//! For details, see the LICENSE file in the repository root.

use std::time::{Duration, Instant};

use ratatui_core::layout::Rect;

use crate::config::{OverflowPolicy, TabDirection, TabWheelDirection};
use crate::layout::compute_viewport;
use crate::nav::TabNav;
use crate::reorder::{TabReorder, can_drag_index, can_drop_at};

/// In-progress mouse drag for tab reordering.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TabReorderDrag {
    /// Display index of the tab being dragged.
    pub source: usize,
    /// Current hover index (valid drop target).
    pub hover: usize,
    /// `true` after the first drag event; press alone does not draw drag highlight.
    pub armed: bool,
}

/// Border-only flash when a tab becomes selected (two on/off pulses).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SelectionFlash {
    tab: usize,
    started: Instant,
}

/// Wall-clock length of each on or off segment (four segments = two blinks).
pub const SELECTION_FLASH_SEGMENT: Duration = Duration::from_millis(150);
/// Total wall-clock duration of the selection flash animation (four segments).
pub const SELECTION_FLASH_TOTAL: Duration = Duration::from_millis(600);

/// Mutable tab selection and scroll state for [`StatefulWidget`](ratatui_core::widgets::StatefulWidget) rendering.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TabNavState {
    /// Index of the highlighted tab.
    pub selected: usize,
    /// Index of the first visible tab when [`OverflowPolicy::Scroll`] is active.
    pub scroll_offset: usize,
    /// Active drag when reordering tabs with the mouse.
    pub reorder_drag: Option<TabReorderDrag>,
    /// Border highlight animation after selection changes.
    selection_flash: Option<SelectionFlash>,
    /// When `false`, selection changes do not start a flash (see [`TabNav::selection_flash`](crate::TabNav::selection_flash)).
    pub selection_flash_enabled: bool,
}

impl TabNavState {
    /// Creates state with `selected` and `scroll_offset` at zero.
    pub const fn new(selected: usize) -> Self {
        Self {
            selected,
            scroll_offset: 0,
            reorder_drag: None,
            selection_flash: None,
            selection_flash_enabled: true,
        }
    }

    /// Starts the border-only selection flash on `tab_index` (two pulses).
    pub fn flash_selection(&mut self, tab_index: usize) {
        if !self.selection_flash_enabled {
            return;
        }
        self.selection_flash = Some(SelectionFlash {
            tab: tab_index,
            started: Instant::now(),
        });
    }

    /// Clears an in-progress selection flash.
    pub fn cancel_selection_flash(&mut self) {
        self.selection_flash = None;
    }

    /// Whether a selection-flash animation is still running (apps should keep redrawing).
    pub fn selection_flash_active(&self) -> bool {
        let Some(flash) = self.selection_flash else {
            return false;
        };
        flash.started.elapsed() < SELECTION_FLASH_TOTAL
    }

    /// Whether `tab_index` should draw the selection-flash border style this frame.
    pub(crate) fn selection_flash_border_on(&self, tab_index: usize) -> bool {
        let Some(flash) = self.selection_flash else {
            return false;
        };
        if flash.tab != tab_index {
            return false;
        }
        let elapsed = flash.started.elapsed();
        if elapsed >= SELECTION_FLASH_TOTAL {
            return false;
        }
        let segment_ms = SELECTION_FLASH_SEGMENT.as_millis().max(1);
        let segment = (elapsed.as_millis() / segment_ms) as u32;
        segment.is_multiple_of(2)
    }

    /// Drops expired selection-flash state (called from stateful render).
    pub(crate) fn tick_selection_flash(&mut self) {
        if self.selection_flash_active() {
            return;
        }
        self.selection_flash = None;
    }

    fn notify_selection_changed(&mut self, previous: usize) {
        if self.selected != previous {
            self.flash_selection(self.selected);
        }
    }

    /// Whether a tab reorder drag is in progress.
    pub const fn is_reorder_dragging(&self) -> bool {
        self.reorder_drag.is_some()
    }

    /// Sets the selected tab, clamped to `tab_count`.
    pub fn select(&mut self, index: usize, tab_count: usize) {
        let previous = self.selected;
        if tab_count == 0 {
            self.selected = 0;
            self.scroll_offset = 0;
            self.notify_selection_changed(previous);
            return;
        }
        self.selected = index.min(tab_count - 1);
        self.notify_selection_changed(previous);
    }

    /// Moves selection along the strip's primary axis.
    pub fn select_direction(&mut self, direction: TabDirection, tab_count: usize) {
        if tab_count == 0 {
            return;
        }
        let previous = self.selected;
        self.selected = match direction {
            TabDirection::Previous => self.selected.saturating_sub(1),
            TabDirection::Next => (self.selected + 1).min(tab_count - 1),
        };
        self.notify_selection_changed(previous);
    }

    /// Wraps selection at the ends of the tab list.
    pub fn select_direction_wrapping(&mut self, direction: TabDirection, tab_count: usize) {
        if tab_count == 0 {
            return;
        }
        let previous = self.selected;
        self.selected = match direction {
            TabDirection::Previous => (self.selected + tab_count - 1) % tab_count,
            TabDirection::Next => (self.selected + 1) % tab_count,
        };
        self.notify_selection_changed(previous);
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

        if self.is_reorder_dragging() {
            return false;
        }

        self.select(index, nav.tabs.len());
        if nav.overflow == OverflowPolicy::Scroll {
            self.ensure_selected_visible(nav, area);
        }
        true
    }

    /// Starts a reorder drag on a draggable tab. Returns `true` when drag began.
    pub fn handle_mouse_reorder_press(
        &mut self,
        nav: &TabNav<'_>,
        area: Rect,
        mouse_column: u16,
        mouse_row: u16,
    ) -> bool {
        if !nav.reorder_enabled() {
            return false;
        }

        let Some(index) = nav.tab_index_at(area, self.scroll_offset, mouse_column, mouse_row)
        else {
            return false;
        };

        if !can_drag_index(index, nav.reorder_policy, nav.tab_pinned) {
            return false;
        }

        self.reorder_drag = Some(TabReorderDrag {
            source: index,
            hover: index,
            armed: false,
        });
        true
    }

    /// Updates the hover slot while dragging. Returns `true` when a drag is active.
    pub fn handle_mouse_reorder_drag(
        &mut self,
        nav: &TabNav<'_>,
        area: Rect,
        mouse_column: u16,
        mouse_row: u16,
    ) -> bool {
        let Some(drag) = self.reorder_drag.as_mut() else {
            return false;
        };
        drag.armed = true;

        let Some(hover) = nav.tab_index_at(area, self.scroll_offset, mouse_column, mouse_row)
        else {
            return true;
        };

        if can_drop_at(drag.source, hover, nav.reorder_policy, nav.tab_pinned) {
            drag.hover = hover;
        }
        true
    }

    /// Ends a reorder drag. Returns [`TabReorder`] when the tab moved to a new slot.
    pub fn handle_mouse_reorder_release(&mut self, nav: &TabNav<'_>) -> Option<TabReorder> {
        let drag = self.reorder_drag.take()?;
        if drag.source == drag.hover {
            return None;
        }
        if !can_drop_at(drag.source, drag.hover, nav.reorder_policy, nav.tab_pinned) {
            return None;
        }
        let previous = self.selected;
        self.selected =
            crate::reorder::remap_selected_index(self.selected, drag.source, drag.hover);
        self.notify_selection_changed(previous);
        Some(TabReorder {
            from: drag.source,
            to: drag.hover,
        })
    }

    /// Cancels an in-progress reorder drag without applying it.
    pub fn cancel_reorder_drag(&mut self) {
        self.reorder_drag = None;
    }

    /// Resets scroll mode offset (call when the strip layout or orientation changes).
    pub const fn clear_scroll(&mut self) {
        self.scroll_offset = 0;
    }
}
