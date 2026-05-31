//! Copyright © 2026 ComfyHome™
//! All rights reserved.
//!
//! Licensed under the ComfyGit SA-PS:DA License
//!
//! For details, see the LICENSE file in the repository root.

/// Converts a single-line label into a vertical stack of characters (one per row).
///
/// ```
/// use ratatui_comfy_tabs::vertical_label;
///
/// assert_eq!(vertical_label("Hi"), "H\ni");
/// ```
pub fn vertical_label(text: &str) -> String {
    text.chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
