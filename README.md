# ratatui-comfy-tabs

An advanced tab navigation widget for [Ratatui](https://ratatui.rs) with individually bordered boxes and rounded corners.

![demo](assets/demo.gif)

## Features

- Horizontal tabs above content or vertical tabs in a left rail beside content
- Each tab renders as a bordered box with configurable corner style (rounded or square)
- Active tab opens into the adjacent content panel via junction corners
- Continuous baseline along the tab strip edge
- Optional indicator symbol on the active tab (`▸` by default for horizontal tabs)
- [`vertical_label`](https://docs.rs/ratatui-comfy-tabs/latest/ratatui_comfy_tabs/fn.vertical_label.html) helper for stacked single-character rows
- Configurable [`TabMargin`](https://docs.rs/ratatui-comfy-tabs/latest/ratatui_comfy_tabs/struct.TabMargin.html) and [`TabPadding`](https://docs.rs/ratatui-comfy-tabs/latest/ratatui_comfy_tabs/struct.TabPadding.html) with orientation-specific defaults
- Depends on `ratatui-core` only — no terminal backend required in library code

## Installation

```bash
cargo add ratatui-comfy-tabs
```

Or add it manually to your `Cargo.toml`:

```toml
[dependencies]
ratatui-comfy-tabs = "0.2"
ratatui = "0.30"
```

## Usage

### Horizontal tabs

```rust
use ratatui::style::{Color, Style};
use ratatui_comfy_tabs::TabNav;

let widget = TabNav::new(&["Files", "Search", "Settings"], 0)
    .highlight_style(Style::new().fg(Color::Cyan))
    .border_style(Style::new().fg(Color::DarkGray));
```

Requires exactly **3 rows** of height (top border, label row, baseline).

### Vertical tabs

```rust
use ratatui::style::{Color, Style};
use ratatui_comfy_tabs::{TabNav, TabOrientation, vertical_label};

let labels: Vec<String> = ["Files", "Search", "Settings"]
    .into_iter()
    .map(vertical_label)
    .collect();
let refs: Vec<&str> = labels.iter().map(String::as_str).collect();

let widget = TabNav::new(&refs, 0)
    .orientation(TabOrientation::Vertical)
    .highlight_style(Style::new().fg(Color::Cyan))
    .border_style(Style::new().fg(Color::DarkGray));
```

Requires at least **3 columns** of width. The indicator is **off by default** for vertical tabs; pass `.indicator(Some("▸"))` to enable.

Labels may contain `\n` for multi-line stacked text, or use [`vertical_label`](https://docs.rs/ratatui-comfy-tabs/latest/ratatui_comfy_tabs/fn.vertical_label.html) to rotate a string.

## Builder Methods

| Method | Default | Description |
|--------|---------|-------------|
| `orientation()` | `Horizontal` | `Horizontal` or `Vertical` tab strip |
| `margin()` | orientation-specific | Strip inset — see [Margin](#margin) |
| `padding()` | orientation-specific | Interior tab spacing — see [Padding](#padding) |
| `style()` | Unstyled | Inactive tab label style |
| `highlight_style()` | Unstyled | Active tab label style |
| `highlight_bold()` | `true` | Auto-apply bold to active tab |
| `border_style()` | Unstyled | Border and baseline style |
| `indicator()` | `Some("▸")` horizontal / `None` vertical | Active-tab marker; pass `None` to disable |
| `border_set()` | `ROUNDED` | Border character set (`ROUNDED`, `PLAIN`, etc.) |
| `horizontal_strip_height()` | — | Minimum render height for horizontal layout |
| `vertical_rail_width()` | — | Rail width for vertical layout (widest tab) |

### Margin

CSS-like inset for the tab strip along the flow axis:

| Orientation | Axes | Default | Example |
|-------------|------|---------|---------|
| Horizontal | left, right (columns) | `0 0` | `.margin(TabMargin::horizontal(2, 0))` |
| Vertical | top, bottom (rows) | `1 1` | `.margin(TabMargin::vertical(0, 2))` |

Use [`TabMargin::ZERO`] to disable vertical default inset.

### Padding

CSS-like `padding: top bottom left right` inside each tab box (top/bottom = rows, left/right = columns):

| Orientation | Default | Meaning |
|-------------|---------|---------|
| Horizontal | `0 0 3 3` | Three columns each side of the label; label on the middle row |
| Vertical | `1 1 1 1` | One row/column of space between border and label |

```rust
use ratatui_comfy_tabs::{TabNav, TabPadding, TabMargin};

TabNav::new(&["Files", "Search"], 0)
    .margin(TabMargin::horizontal(1, 1))
    .padding(TabPadding::new(0, 0, 2, 2));
```

## Demo

```bash
cargo run --example demo
```

| Key | Action |
|-----|--------|
| `h` / `l` or ← / → | Previous / next tab (horizontal mode) |
| `j` / `k` or ↑ / ↓ | Previous / next tab (vertical mode) |
| `Tab` / `BackTab` | Cycle tabs |
| `M` | Toggle horizontal / vertical mode |
| `I` | Toggle active-tab indicator |
| `B` | Toggle rounded / square borders |
| `q` / `Esc` | Quit |

Run `cargo run --example demo` for the interactive showcase.

## License

Version 0.1.0 and above is licensed under the Ratatui-Comfy-Tabs Project License — SA-PS:DA (v1.0). See [LICENSE.md](LICENSE.md).

## Contribution

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Attribution

ratatui-comfy-tabs v0.0.1 uses approx 350 LoC of `tui-tabs` by [jharsono](https://github.com/jharsono), therefore, v0.0.1 inherits its license. Lineage and upstream references are recorded in `Cargo.toml` under `[package.metadata]`.
