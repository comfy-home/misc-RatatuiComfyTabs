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
pub fn can_drag_index(
    index: usize,
    policy: TabReorderPolicy,
    tab_pinned: Option<&[bool]>,
) -> bool {
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
    let item = items.remove(from);
    items.insert(to, item);
    true
}

/// Maps a selection index after [`try_reorder`].
pub fn remap_selected_index(selected: usize, from: usize, to: usize) -> usize {
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
        assert_eq!(items, ['a', 'c', 'd', 'b']);
    }

    #[test]
    fn remap_selected_follows_move() {
        assert_eq!(remap_selected_index(1, 1, 3), 3);
        assert_eq!(remap_selected_index(2, 1, 3), 1);
    }
}
