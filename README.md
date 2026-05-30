# ratatui-comfy-tabs

An advanced tab navigation widget for [Ratatui](https://ratatui.rs) with individually bordered boxes and rounded corners.

![demo](assets/demo.gif)

## Features

- Each tab renders as a bordered box with configurable corner style (rounded or square)
- Active tab opens into the content below via junction corners
- Continuous baseline spans the full widget width
- Optional indicator symbol next to the active tab label
- Builder API following Ratatui conventions
- Depends on `ratatui-core` only — no terminal backend required in library code

## Installation

```bash
cargo add ratatui-comfy-tabs
```

Or add it manually to your `Cargo.toml`:

```toml
[dependencies]
ratatui-comfy-tabs = "0.1"
ratatui = "0.30"
```

## Usage

```rust
use ratatui::style::{Color, Style};
use ratatui_comfy_tabs::TabNav;

let widget = TabNav::new(&["Files", "Search", "Settings"], 0)
    .highlight_style(Style::new().fg(Color::Cyan))
    .border_style(Style::new().fg(Color::DarkGray));
```

The widget requires exactly 3 rows of height (top border, label row, baseline).

## Builder Methods

| Method | Default | Description |
|--------|---------|-------------|
| `style()` | Unstyled | Inactive tab label style |
| `highlight_style()` | Unstyled | Active tab label style |
| `highlight_bold()` | `true` | Auto-apply bold to active tab |
| `border_style()` | Unstyled | Border and baseline style |
| `indicator()` | `Some("▸")` | Symbol left of active label; `None` to disable |
| `border_set()` | `ROUNDED` | Border character set (`ROUNDED`, `PLAIN`, etc.) |

## Examples

```bash
cargo run --example basic        # Static render, press any key to exit
cargo run --example interactive  # Arrow keys to navigate, q to quit
cargo run --example demo         # Styled multi-tab demo
```

## License

Version 0.1.0 and above is licensed under the Ratatui-Comfy-Tabs Project License — SA-PS:DA (v1.0). See [LICENSE.md](LICENSE.md).

## Contribution

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Attribution

ratatui-comfy-tabs v0.0.1 uses approx 350 LoC of `tui-tabs` by [jharsono](https://github.com/jharsono), therefore, v0.0.1 inherits its license. Lineage and upstream references are recorded in `Cargo.toml` under `[package.metadata]`.
