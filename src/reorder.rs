//! Copyright © 2026 ComfyHome™
//! All rights reserved.
//!
//! Licensed under the ComfyGit SA-PS:DA License
//!
//! For details, see the LICENSE file in the repository root.

use crate::config::TabReorderPolicy;

/// Completed drag-and-drop between two display indices.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TabReorder {
    /// Index in the current label slice before reorder.
    pub from: usize,
    /// Insertion index in the slice after removal (same convention as [`try_reorder`]).
    pub to: usize,
}

/// Whether the tab at `index` may be dragged.
pub fn can_drag_index(index: usize, policy: TabReorderPolicy, tab_pinned: Option<&[bool]>) -> bool {
    match policy {
        TabReorderPolicy::AllPinned => false,
        TabReorderPolicy::NonePinned => true,
        TabReorderPolicy::SomePinned => !is_pinned(index, tab_pinned),
    }
}

/// Whether dropping onto `to` is allowed when dragging from `from`.
pub fn can_drop_at(
    from: usize,
    to: usize,
    policy: TabReorderPolicy,
    tab_pinned: Option<&[bool]>,
) -> bool {
    if from == to {
        return true;
    }
    can_drag_index(from, policy, tab_pinned) && can_drag_index(to, policy, tab_pinned)
}

/// Reorders `items` in place. Returns `false` when the move is not allowed.
pub fn try_reorder<T>(
    items: &mut Vec<T>,
    from: usize,
    to: usize,
    policy: TabReorderPolicy,
    tab_pinned: Option<&[bool]>,
) -> bool {
    if from >= items.len() || to >= items.len() {
        return false;
    }
    if !can_drop_at(from, to, policy, tab_pinned) {
        return false;
    }
    if from == to {
        return true;
    }
    if policy == TabReorderPolicy::SomePinned {
        let Some(pins) = tab_pinned else {
            return false;
        };
        let movable = movable_indices(items.len(), Some(pins));
        let Some(from_pos) = movable.iter().position(|&index| index == from) else {
            return false;
        };
        let Some(to_pos) = movable.iter().position(|&index| index == to) else {
            return false;
        };
        if from_pos < to_pos {
            for pos in from_pos..to_pos {
                items.swap(movable[pos], movable[pos + 1]);
            }
        } else {
            for pos in (to_pos + 1..=from_pos).rev() {
                items.swap(movable[pos], movable[pos - 1]);
            }
        }
        return true;
    }

    let item = items.remove(from);
    items.insert(to, item);
    true
}

/// Maps a selection index after [`try_reorder`].
pub fn remap_selected_index(selected: usize, from: usize, to: usize) -> usize {
    remap_selected_index_with_pins(selected, from, to, None)
}

/// Maps a selection index after [`try_reorder`] with optional pinned slots.
pub fn remap_selected_index_with_pins(
    selected: usize,
    from: usize,
    to: usize,
    tab_pinned: Option<&[bool]>,
) -> usize {
    if is_pinned(selected, tab_pinned) {
        return selected;
    }
    if let Some(pins) = tab_pinned {
        let len = pins
            .len()
            .max(selected.saturating_add(1))
            .max(from + 1)
            .max(to + 1);
        let movable = movable_indices(len, Some(pins));
        let Some(from_pos) = movable.iter().position(|&index| index == from) else {
            return selected;
        };
        let Some(to_pos) = movable.iter().position(|&index| index == to) else {
            return selected;
        };
        let Some(sel_pos) = movable.iter().position(|&index| index == selected) else {
            return selected;
        };
        if selected == from {
            return to;
        }
        if from_pos < to_pos && sel_pos > from_pos && sel_pos <= to_pos {
            return movable[sel_pos - 1];
        }
        if from_pos > to_pos && sel_pos >= to_pos && sel_pos < from_pos {
            return movable[sel_pos + 1];
        }
        return selected;
    }

    if selected == from {
        return to;
    }
    if from < selected && to >= selected {
        selected.saturating_sub(1)
    } else if from > selected && to <= selected {
        selected.saturating_add(1)
    } else {
        selected
    }
}

fn is_pinned(index: usize, tab_pinned: Option<&[bool]>) -> bool {
    tab_pinned
        .and_then(|pins| pins.get(index))
        .copied()
        .unwrap_or(false)
}

fn movable_indices(len: usize, tab_pinned: Option<&[bool]>) -> Vec<usize> {
    (0..len)
        .filter(|&index| !is_pinned(index, tab_pinned))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_pinned_rejects_reorder() {
        let mut items = vec!['a', 'b', 'c'];
        assert!(!try_reorder(
            &mut items,
            0,
            2,
            TabReorderPolicy::AllPinned,
            None
        ));
    }

    #[test]
    fn none_pinned_reorders() {
        let mut items = vec!['a', 'b', 'c', 'd'];
        assert!(try_reorder(
            &mut items,
            1,
            3,
            TabReorderPolicy::NonePinned,
            None
        ));
        assert_eq!(items, ['a', 'c', 'd', 'b']);
    }

    #[test]
    fn some_pinned_keeps_pinned_slot() {
        let mut items = vec!['a', 'b', 'c', 'd'];
        let pinned = [true, false, true, false];
        assert!(!try_reorder(
            &mut items,
            0,
            2,
            TabReorderPolicy::SomePinned,
            Some(&pinned)
        ));
        assert!(!try_reorder(
            &mut items,
            1,
            2,
            TabReorderPolicy::SomePinned,
            Some(&pinned)
        ));
        assert!(try_reorder(
            &mut items,
            1,
            3,
            TabReorderPolicy::SomePinned,
            Some(&pinned)
        ));
        assert_eq!(items, ['a', 'd', 'c', 'b']);
    }

    #[test]
    fn some_pinned_preserves_anchor_when_moving_across_pinned() {
        let mut items = vec!['a', 'b', 'c', 'd', 'e'];
        let pinned = [true, false, true, false, false];
        assert!(try_reorder(
            &mut items,
            1,
            4,
            TabReorderPolicy::SomePinned,
            Some(&pinned)
        ));
        assert_eq!(items, ['a', 'd', 'c', 'e', 'b']);
        assert_eq!(items[2], 'c');
    }

    #[test]
    fn remap_selected_follows_move() {
        assert_eq!(remap_selected_index(1, 1, 3), 3);
        assert_eq!(remap_selected_index(2, 1, 3), 1);
    }
}
