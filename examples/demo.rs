//! Copyright © 2026 ComfyHome™
//! All rights reserved.
//!
//! Licensed under the ComfyGit SA-PS:DA License
//!
//! For details, see the LICENSE file in the repository root.

//! Interactive demo for ratatui-comfy-tabs.
//!
//! Run: `cargo run --example demo`

use ratatui::{
    Frame,
    crossterm::{
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEvent,
            MouseEventKind,
        },
        execute,
    },
    layout::Alignment,
    prelude::{Buffer, Constraint, Layout, Rect, Stylize, Widget},
    style::{Color, Style},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};
use ratatui_comfy_tabs::{
    HorizontalPosition, OverflowPolicy, TabAxis, TabBarAlign, TabBarEnd, TabDirection, TabNav,
    TabNavState, TabOrientation, TabPadding, TabReorderPolicy, TabWheelDirection, VerticalPosition,
    tab_border, try_reorder, vertical_label,
};
use ratatui_core::widgets::StatefulWidget;
use std::io::stdout;
use std::time::{Duration, Instant};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    ratatui::run(|terminal| {
        execute!(stdout(), EnableMouseCapture)?;
        let result = App::default().run(terminal);
        let _ = execute!(stdout(), DisableMouseCapture);
        result
    })?;
    Ok(())
}

const TABS: &[&str] = &[
    "Overview", "Nodes", "Network", "Content", "UI", "Config", "Logs",
];

const INDICATOR: &str = "▸";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum DemoMode {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum BorderKind {
    #[default]
    Rnd,
    Sqr,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum PaddingPreset {
    #[default]
    Default,
    Alt2,
    Alt3,
}

struct App {
    tab_state: TabNavState,
    mode: DemoMode,
    horizontal_position: HorizontalPosition,
    vertical_position: VerticalPosition,
    tab_bar_align: TabBarAlign,
    border_kind: BorderKind,
    show_indicator: bool,
    padding_preset: PaddingPreset,
    tab_bar_end: TabBarEnd,
    all_caps: bool,
    overflow: OverflowPolicy,
    narrow_tabs: bool,
    mouse_wheel: bool,
    mouse_click: bool,
    mouse_reorder: bool,
    reorder_policy: TabReorderPolicy,
    tab_order: Vec<usize>,
    vertical_labels: Vec<String>,
    wheel_strip_area: Rect,
    tab_hit_area: Rect,
    last_mouse_column: u16,
    last_mouse_row: u16,
    last_command: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            tab_state: TabNavState::default(),
            mode: DemoMode::default(),
            horizontal_position: HorizontalPosition::default(),
            vertical_position: VerticalPosition::default(),
            tab_bar_align: TabBarAlign::default(),
            border_kind: BorderKind::default(),
            show_indicator: false,
            padding_preset: PaddingPreset::default(),
            tab_bar_end: TabBarEnd::default(),
            all_caps: false,
            overflow: OverflowPolicy::default(),
            narrow_tabs: false,
            mouse_wheel: true,
            mouse_click: true,
            mouse_reorder: true,
            reorder_policy: TabReorderPolicy::AllPinned,
            tab_order: (0..TABS.len()).collect(),
            vertical_labels: Vec::new(),
            wheel_strip_area: Rect::default(),
            tab_hit_area: Rect::default(),
            last_mouse_column: 0,
            last_mouse_row: 0,
            last_command: String::from("// Press a shortcut or click a tab to see the code change"),
        }
    }
}

fn spans_width(spans: &[Span<'_>]) -> usize {
    spans.iter().map(Span::width).sum()
}

fn wrap_footer_segments(segments: Vec<Vec<Span<'static>>>, width: usize) -> Vec<Line<'static>> {
    if segments.is_empty() {
        return vec![Line::from("")];
    }

    let separator = vec![Span::styled(" | ", Style::new().fg(Color::DarkGray))];
    let separator_width = spans_width(&separator);

    let mut lines = Vec::new();
    let mut current = Vec::new();
    let mut current_width = 0;

    for segment in segments {
        let segment_width = spans_width(&segment);
        let added_width = if current.is_empty() {
            segment_width
        } else {
            separator_width + segment_width
        };

        if !current.is_empty() && current_width + added_width > width {
            lines.push(Line::from(std::mem::take(&mut current)));
            current = segment;
            current_width = segment_width;
        } else {
            if !current.is_empty() {
                current.extend(separator.clone());
                current_width += separator_width;
            }
            current.extend(segment);
            current_width += segment_width;
        }
    }

    if !current.is_empty() {
        lines.push(Line::from(current));
    }

    lines
}

impl App {
    fn record_command(&mut self, text: impl Into<String>) {
        self.last_command = text.into();
    }

    fn run(mut self, terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
        self.vertical_labels = TABS.iter().map(|label| vertical_label(label)).collect();

        loop {
            terminal.draw(|frame| self.draw(frame))?;

            let poll_timeout = if self.tab_state.selection_flash_active() {
                Duration::from_millis(50)
            } else {
                Duration::from_secs(24 * 60 * 60)
            };
            if !event::poll(poll_timeout)? {
                continue;
            }

            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),

                    KeyCode::Char('m') | KeyCode::Char('M') => {
                        self.mode = match self.mode {
                            DemoMode::Horizontal => DemoMode::Vertical,
                            DemoMode::Vertical => DemoMode::Horizontal,
                        };
                        self.tab_state.clear_scroll();
                        self.tab_state.cancel_reorder_drag();
                        self.record_command(format!("self.mode = DemoMode::{:?};", self.mode));
                    }

                    KeyCode::Char('i') | KeyCode::Char('I') => {
                        self.show_indicator = !self.show_indicator;
                        self.record_command(format!(
                            "self.show_indicator = !self.show_indicator;  // {}",
                            self.show_indicator
                        ));
                    }

                    KeyCode::Char('b') | KeyCode::Char('B') => {
                        self.border_kind = match self.border_kind {
                            BorderKind::Rnd => BorderKind::Sqr,
                            BorderKind::Sqr => BorderKind::Rnd,
                        };
                        self.record_command(format!(
                            ".border_set(tab_border::{:?});",
                            self.border_kind
                        ));
                    }

                    KeyCode::Char('1') => {
                        self.padding_preset = match self.padding_preset {
                            PaddingPreset::Default => PaddingPreset::Alt2,
                            PaddingPreset::Alt2 => PaddingPreset::Alt3,
                            PaddingPreset::Alt3 => PaddingPreset::Default,
                        };
                        self.record_command(format!(
                            "self.padding_preset = PaddingPreset::{:?};",
                            self.padding_preset
                        ));
                    }

                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        self.all_caps = !self.all_caps;
                        self.record_command(format!(
                            "self.all_caps = !self.all_caps;  // {}",
                            self.all_caps
                        ));
                    }

                    KeyCode::Char('2') => {
                        self.tab_bar_end = match self.tab_bar_end {
                            TabBarEnd::NoEnd => TabBarEnd::Sqr,
                            TabBarEnd::Sqr => TabBarEnd::Rnd,
                            TabBarEnd::Rnd => TabBarEnd::NoEnd,
                        };
                        self.record_command(format!(
                            "self.tab_bar_end = TabBarEnd::{:?};",
                            self.tab_bar_end
                        ));
                    }

                    KeyCode::Char('o') | KeyCode::Char('O') => {
                        self.overflow = match self.overflow {
                            OverflowPolicy::Truncate => OverflowPolicy::Scroll,
                            OverflowPolicy::Scroll => OverflowPolicy::Truncate,
                        };
                        self.tab_state.scroll_offset = 0;
                        self.record_command(format!(
                            "self.overflow = OverflowPolicy::{:?};\nself.tab_state.scroll_offset = 0;",
                            self.overflow
                        ));
                    }

                    KeyCode::Char('w') | KeyCode::Char('W') => {
                        self.narrow_tabs = !self.narrow_tabs;
                        self.record_command(format!(
                            "self.narrow_tabs = !self.narrow_tabs;  // {}",
                            self.narrow_tabs
                        ));
                    }

                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        self.mouse_wheel = !self.mouse_wheel;
                        self.record_command(format!(
                            "self.mouse_wheel = !self.mouse_wheel;  // {}",
                            self.mouse_wheel
                        ));
                    }

                    KeyCode::Char('x') | KeyCode::Char('X') => {
                        self.mouse_click = !self.mouse_click;
                        self.record_command(format!(
                            "self.mouse_click = !self.mouse_click;  // {}",
                            self.mouse_click
                        ));
                    }

                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        self.tab_state.selection_flash_enabled =
                            !self.tab_state.selection_flash_enabled;
                        if !self.tab_state.selection_flash_enabled {
                            self.tab_state.cancel_selection_flash();
                        }
                        self.record_command(format!(
                            "tab_state.selection_flash_enabled = {}; .selection_flash({})",
                            self.tab_state.selection_flash_enabled,
                            self.tab_state.selection_flash_enabled
                        ));
                    }

                    KeyCode::Char('p') | KeyCode::Char('P') => {
                        match self.mode {
                            DemoMode::Horizontal => {
                                self.horizontal_position = match self.horizontal_position {
                                    HorizontalPosition::Top => HorizontalPosition::Bottom,
                                    HorizontalPosition::Bottom => HorizontalPosition::Top,
                                };
                                self.record_command(format!(
                                    ".horizontal_position(HorizontalPosition::{:?});",
                                    self.horizontal_position
                                ));
                            }
                            DemoMode::Vertical => {
                                self.vertical_position = match self.vertical_position {
                                    VerticalPosition::Left => VerticalPosition::Right,
                                    VerticalPosition::Right => VerticalPosition::Left,
                                };
                                self.record_command(format!(
                                    ".vertical_position(VerticalPosition::{:?});",
                                    self.vertical_position
                                ));
                            }
                        }
                        self.tab_state.clear_scroll();
                        self.tab_state.cancel_reorder_drag();
                    }

                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        self.tab_bar_align = match self.tab_bar_align {
                            TabBarAlign::Start => TabBarAlign::Center,
                            TabBarAlign::Center => TabBarAlign::End,
                            TabBarAlign::End => TabBarAlign::Start,
                        };
                        self.tab_state.clear_scroll();
                        self.record_command(format!(
                            ".tab_bar_align(TabBarAlign::{:?});",
                            self.tab_bar_align
                        ));
                    }

                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        self.reorder_policy = match self.reorder_policy {
                            TabReorderPolicy::AllPinned => TabReorderPolicy::NonePinned,
                            TabReorderPolicy::NonePinned => TabReorderPolicy::SomePinned,
                            TabReorderPolicy::SomePinned => TabReorderPolicy::AllPinned,
                        };
                        self.mouse_reorder = self.reorder_policy != TabReorderPolicy::AllPinned;
                        self.tab_state.cancel_reorder_drag();
                        self.record_command(format!(
                            ".reorder_policy(TabReorderPolicy::{:?}).mouse_reorder({});\n// SomePinned pins Overview and Network (tab 1 and 3)\n done here in DEMO app via `pins` ==> `pins[display] = tab_index == 0 || tab_index == 2;`",
                            self.reorder_policy, self.mouse_reorder
                        ));
                    }

                    KeyCode::BackTab => {
                        self.tab_state.select_direction_wrapping(
                            TabDirection::Previous,
                            self.tab_order.len(),
                        );
                        self.record_command(format!(
                            "tab_state.select_direction_wrapping(TabDirection::Previous, {});\n// selected = {} ({})",
                            self.tab_order.len(),
                            self.tab_state.selected,
                            self.selected_tab_name()
                        ));
                    }
                    KeyCode::Tab => {
                        self.tab_state
                            .select_direction_wrapping(TabDirection::Next, self.tab_order.len());
                        self.record_command(format!(
                            "tab_state.select_direction_wrapping(TabDirection::Next, {});\n// selected = {} ({})",
                            self.tab_order.len(),
                            self.tab_state.selected,
                            self.selected_tab_name()
                        ));
                    }

                    KeyCode::Left if self.mode == DemoMode::Horizontal => {
                        self.tab_state
                            .select_direction(TabAxis::Decrease.direction(), self.tab_order.len());
                        self.record_command(format!(
                            "tab_state.select_direction(TabDirection::Previous, {});\n// selected = {} ({})",
                            self.tab_order.len(),
                            self.tab_state.selected,
                            self.selected_tab_name()
                        ));
                    }
                    KeyCode::Right if self.mode == DemoMode::Horizontal => {
                        self.tab_state
                            .select_direction(TabAxis::Increase.direction(), self.tab_order.len());
                        self.record_command(format!(
                            "tab_state.select_direction(TabDirection::Next, {});\n// selected = {} ({})",
                            self.tab_order.len(),
                            self.tab_state.selected,
                            self.selected_tab_name()
                        ));
                    }

                    KeyCode::Up | KeyCode::Char('k') if self.mode == DemoMode::Vertical => {
                        self.tab_state
                            .select_direction(TabAxis::Decrease.direction(), self.tab_order.len());
                        self.record_command(format!(
                            "tab_state.select_direction(TabDirection::Previous, {});\n// selected = {} ({})",
                            self.tab_order.len(),
                            self.tab_state.selected,
                            self.selected_tab_name()
                        ));
                    }
                    KeyCode::Down | KeyCode::Char('j') if self.mode == DemoMode::Vertical => {
                        self.tab_state
                            .select_direction(TabAxis::Increase.direction(), self.tab_order.len());
                        self.record_command(format!(
                            "tab_state.select_direction(TabDirection::Next, {});\n// selected = {} ({})",
                            self.tab_order.len(),
                            self.tab_state.selected,
                            self.selected_tab_name()
                        ));
                    }

                    KeyCode::Char('[') => {
                        self.tab_state.scroll_prev();
                        self.record_command(format!(
                            "tab_state.scroll_prev();\n// scroll_offset = {}",
                            self.tab_state.scroll_offset
                        ));
                    }
                    KeyCode::Char(']') => {
                        let labels_owned = self.tab_labels_owned_for_nav();
                        let labels: Vec<&str> = labels_owned.iter().map(String::as_str).collect();
                        let mut pin_buf = Vec::new();
                        let nav = self.prepare_tab_nav(&labels, &mut pin_buf);
                        self.tab_state.scroll_next(&nav, self.wheel_strip_area);
                        self.record_command(format!(
                            "tab_state.scroll_next(&nav, wheel_strip_area);\n// scroll_offset = {}",
                            self.tab_state.scroll_offset
                        ));
                    }

                    _ => {}
                },
                Event::Mouse(mouse) => {
                    self.handle_mouse(mouse);
                }
                _ => {}
            }
        }
    }

    fn tab_orientation(&self) -> TabOrientation {
        match self.mode {
            DemoMode::Horizontal => TabOrientation::Horizontal,
            DemoMode::Vertical => TabOrientation::Vertical,
        }
    }

    fn wheel_axes_from_mouse(
        mouse: &MouseEvent,
    ) -> (Option<TabWheelDirection>, Option<TabWheelDirection>) {
        let vertical = match mouse.kind {
            MouseEventKind::ScrollUp => Some(TabWheelDirection::Up),
            MouseEventKind::ScrollDown => Some(TabWheelDirection::Down),
            _ => None,
        };
        let horizontal = match mouse.kind {
            MouseEventKind::ScrollLeft => Some(TabWheelDirection::Up),
            MouseEventKind::ScrollRight => Some(TabWheelDirection::Down),
            _ => None,
        };
        (vertical, horizontal)
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) {
        self.last_mouse_column = mouse.column;
        self.last_mouse_row = mouse.row;

        match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if !self.handle_mouse_reorder_press() {
                    self.handle_mouse_click();
                }
            }
            MouseEventKind::Drag(MouseButton::Left) => self.handle_mouse_reorder_drag(),
            MouseEventKind::Up(MouseButton::Left) => self.handle_mouse_reorder_release(),
            MouseEventKind::ScrollUp
            | MouseEventKind::ScrollDown
            | MouseEventKind::ScrollLeft
            | MouseEventKind::ScrollRight => self.handle_mouse_wheel(mouse),
            _ => {}
        }
    }

    fn ordered_labels_owned(&self) -> Vec<String> {
        self.tab_order
            .iter()
            .map(|&index| TABS[index].to_string())
            .collect()
    }

    /// Tab labels for [`TabNav`] — must match rendering (stacked labels in vertical mode).
    fn tab_labels_owned_for_nav(&self) -> Vec<String> {
        match self.mode {
            DemoMode::Horizontal => self
                .tab_order
                .iter()
                .map(|&index| TABS[index].to_string())
                .collect(),
            DemoMode::Vertical => self
                .tab_order
                .iter()
                .map(|&index| self.vertical_labels[index].clone())
                .collect(),
        }
    }

    fn compute_tab_pins(&self, len: usize) -> Vec<bool> {
        let mut pins = vec![false; len];
        if self.reorder_policy == TabReorderPolicy::SomePinned {
            for (display, &tab_index) in self.tab_order.iter().enumerate().take(len) {
                pins[display] = tab_index == 0 || tab_index == 2;
            }
        }
        pins
    }

    fn tab_pins_option<'a>(&self, len: usize, storage: &'a mut Vec<bool>) -> Option<&'a [bool]> {
        if self.reorder_policy != TabReorderPolicy::SomePinned {
            return None;
        }
        *storage = self.compute_tab_pins(len);
        Some(storage.as_slice())
    }

    fn apply_tab_order_reorder(&mut self, from: usize, to: usize) {
        let pins = self.compute_tab_pins(self.tab_order.len());
        let pin_slice = match self.reorder_policy {
            TabReorderPolicy::SomePinned => Some(pins.as_slice()),
            _ => None,
        };
        if try_reorder(
            &mut self.tab_order,
            from,
            to,
            self.reorder_policy,
            pin_slice,
        ) {
            self.tab_state.selected = ratatui_comfy_tabs::remap_selected_index_with_pins(
                self.tab_state.selected,
                from,
                to,
                pin_slice,
            );
        }
    }

    fn handle_mouse_reorder_press(&mut self) -> bool {
        if !self.mouse_reorder {
            return false;
        }
        let labels_owned = self.tab_labels_owned_for_nav();
        let labels: Vec<&str> = labels_owned.iter().map(String::as_str).collect();
        let mut pin_buf = Vec::new();
        let nav = self.prepare_tab_nav(&labels, &mut pin_buf);
        let started = self.tab_state.handle_mouse_reorder_press(
            &nav,
            self.tab_hit_area,
            self.last_mouse_column,
            self.last_mouse_row,
        );
        if started {
            self.record_command(format!(
                "tab_state.handle_mouse_reorder_press(&nav, tab_hit_area, {}, {});",
                self.last_mouse_column, self.last_mouse_row
            ));
        }
        started
    }

    fn handle_mouse_reorder_drag(&mut self) {
        let Some(drag) = self.tab_state.reorder_drag else {
            return;
        };
        let from = drag.source;
        let labels_owned = self.tab_labels_owned_for_nav();
        let labels: Vec<&str> = labels_owned.iter().map(String::as_str).collect();
        let mut pin_buf = Vec::new();
        let nav = self.prepare_tab_nav(&labels, &mut pin_buf);
        let prev_hover = drag.hover;
        self.tab_state.handle_mouse_reorder_drag(
            &nav,
            self.tab_hit_area,
            self.last_mouse_column,
            self.last_mouse_row,
        );
        let Some(drag) = self.tab_state.reorder_drag else {
            return;
        };
        if drag.hover != prev_hover && drag.hover != from {
            self.apply_tab_order_reorder(from, drag.hover);
            self.tab_state.reorder_drag = Some(ratatui_comfy_tabs::TabReorderDrag {
                source: drag.hover,
                hover: drag.hover,
                armed: true,
            });
            self.record_command(format!(
                "try_reorder(&mut tab_order, {}, {}, {:?}, …);",
                from, drag.hover, self.reorder_policy
            ));
        }
    }

    fn handle_mouse_reorder_release(&mut self) {
        if !self.tab_state.is_reorder_dragging() {
            return;
        }
        let labels_owned = self.tab_labels_owned_for_nav();
        let labels: Vec<&str> = labels_owned.iter().map(String::as_str).collect();
        let mut pin_buf = Vec::new();
        let nav = self.prepare_tab_nav(&labels, &mut pin_buf);
        if let Some(reorder) = self.tab_state.handle_mouse_reorder_release(&nav) {
            self.apply_tab_order_reorder(reorder.from, reorder.to);
            self.record_command(format!(
                "try_reorder(&mut tab_order, {}, {}, TabReorderPolicy::{:?}, …);",
                reorder.from, reorder.to, self.reorder_policy
            ));
        } else {
            self.handle_mouse_click();
        }
    }

    fn handle_mouse_click(&mut self) {
        let labels_owned = self.tab_labels_owned_for_nav();
        let labels: Vec<&str> = labels_owned.iter().map(String::as_str).collect();
        let mut pin_buf = Vec::new();
        let nav = self.prepare_tab_nav(&labels, &mut pin_buf);
        let consumed = self.tab_state.handle_mouse_click(
            &nav,
            self.tab_hit_area,
            self.last_mouse_column,
            self.last_mouse_row,
        );

        if consumed {
            self.record_command(format!(
                "tab_state.handle_mouse_click(&nav, tab_hit_area, {}, {});\n// selected = {} ({})",
                self.last_mouse_column,
                self.last_mouse_row,
                self.tab_state.selected,
                self.selected_tab_name()
            ));
        }
    }

    fn handle_mouse_wheel(&mut self, mouse: MouseEvent) {
        let (vertical, horizontal) = Self::wheel_axes_from_mouse(&mouse);
        let Some(direction) =
            TabWheelDirection::from_axes(vertical, horizontal, self.tab_orientation())
        else {
            return;
        };

        self.drain_matching_wheel(direction);

        let labels_owned = self.tab_labels_owned_for_nav();
        let labels: Vec<&str> = labels_owned.iter().map(String::as_str).collect();
        let mut pin_buf = Vec::new();
        let nav = self.prepare_tab_nav(&labels, &mut pin_buf);
        let consumed = self.tab_state.handle_mouse_wheel(
            &nav,
            self.wheel_strip_area,
            self.last_mouse_column,
            self.last_mouse_row,
            direction,
        );

        if consumed {
            self.record_command(format!(
                "tab_state.handle_mouse_wheel(&nav, wheel_strip_area, col, row, TabWheelDirection::{:?});\n// selected = {} ({})",
                direction,
                self.tab_state.selected,
                self.selected_tab_name()
            ));
        }
    }

    fn drain_matching_wheel(&mut self, direction: TabWheelDirection) {
        let deadline = Instant::now() + Duration::from_millis(30);
        while Instant::now() < deadline {
            if !event::poll(Duration::from_millis(0)).unwrap_or(false) {
                break;
            }
            let Ok(Event::Mouse(mouse)) = event::read() else {
                break;
            };
            self.last_mouse_column = mouse.column;
            self.last_mouse_row = mouse.row;
            let (vertical, horizontal) = Self::wheel_axes_from_mouse(&mouse);
            let same = TabWheelDirection::from_axes(vertical, horizontal, self.tab_orientation())
                == Some(direction);
            if !same {
                break;
            }
        }
    }

    fn tab_border_set(&self) -> border::Set<'static> {
        match self.border_kind {
            BorderKind::Rnd => tab_border::Rnd,
            BorderKind::Sqr => tab_border::Sqr,
        }
    }

    fn content_border_set(&self) -> border::Set<'static> {
        let mut set = self.tab_border_set();
        if self.mode == DemoMode::Vertical {
            match self.vertical_position {
                VerticalPosition::Left => set.top_left = "─",
                VerticalPosition::Right => set.top_right = "─",
            }
        }
        set
    }

    fn padding_for_mode(&self) -> Option<TabPadding> {
        match (self.mode, self.padding_preset) {
            (_, PaddingPreset::Default) => None,
            (DemoMode::Horizontal, PaddingPreset::Alt2) => Some(TabPadding::axes(1, 1)),
            (DemoMode::Horizontal, PaddingPreset::Alt3) => Some(TabPadding::axes(5, 5)),
            (DemoMode::Vertical, PaddingPreset::Alt2) => Some(TabPadding::uniform(3)),
            (DemoMode::Vertical, PaddingPreset::Alt3) => Some(TabPadding::new(1, 1, 2, 2)),
        }
    }

    fn padding_label(&self) -> &'static str {
        match (self.mode, self.padding_preset) {
            (_, PaddingPreset::Default) => "default",
            (DemoMode::Horizontal, PaddingPreset::Alt2) => "1/1",
            (DemoMode::Horizontal, PaddingPreset::Alt3) => "5/5",
            (DemoMode::Vertical, PaddingPreset::Alt2) => "3³",
            (DemoMode::Vertical, PaddingPreset::Alt3) => "1,2",
        }
    }

    fn tab_bar_end_label(&self) -> &'static str {
        match self.tab_bar_end {
            TabBarEnd::NoEnd => "none",
            TabBarEnd::Sqr => "sqr",
            TabBarEnd::Rnd => "rnd",
        }
    }

    fn overflow_label(&self) -> &'static str {
        match self.overflow {
            OverflowPolicy::Truncate => "truncate",
            OverflowPolicy::Scroll => "scroll",
        }
    }

    fn reorder_policy_label(&self) -> &'static str {
        match self.reorder_policy {
            TabReorderPolicy::AllPinned => "all-pinned",
            TabReorderPolicy::NonePinned => "none-pinned",
            TabReorderPolicy::SomePinned => "some-pinned",
        }
    }

    fn align_label(&self) -> &'static str {
        match self.tab_bar_align {
            TabBarAlign::Start => match self.mode {
                DemoMode::Horizontal => "start",
                DemoMode::Vertical => "top",
            },
            TabBarAlign::Center => "center",
            TabBarAlign::End => match self.mode {
                DemoMode::Horizontal => "end",
                DemoMode::Vertical => "bottom",
            },
        }
    }

    fn position_label(&self) -> &'static str {
        match self.mode {
            DemoMode::Horizontal => match self.horizontal_position {
                HorizontalPosition::Top => "top",
                HorizontalPosition::Bottom => "bottom",
            },
            DemoMode::Vertical => match self.vertical_position {
                VerticalPosition::Left => "left",
                VerticalPosition::Right => "right",
            },
        }
    }

    fn selected_tab_name(&self) -> &'static str {
        self.tab_order
            .get(self.tab_state.selected)
            .map(|&index| TABS[index])
            .unwrap_or(TABS[0])
    }

    fn prepare_tab_nav<'a>(&self, tabs: &'a [&'a str], pin_buf: &'a mut Vec<bool>) -> TabNav<'a> {
        let pin_opt = self.tab_pins_option(tabs.len(), pin_buf);
        self.build_tab_nav(tabs, pin_opt)
    }

    fn build_tab_nav<'a>(&self, tabs: &'a [&'a str], tab_pinned: Option<&'a [bool]>) -> TabNav<'a> {
        let bg = Color::Rgb(20, 20, 40);
        let highlight = Color::LightBlue;
        let dim = Color::DarkGray;
        let border_color = Color::Rgb(60, 60, 100);

        let mut nav = TabNav::new(tabs, self.tab_state.selected)
            .border_set(self.tab_border_set())
            .overflow(self.overflow)
            .mouse_wheel(self.mouse_wheel)
            .mouse_click(self.mouse_click)
            .reorder_policy(self.reorder_policy)
            .mouse_reorder(self.mouse_reorder);
        if self.mode == DemoMode::Vertical {
            nav = nav
                .orientation(TabOrientation::Vertical)
                .vertical_position(self.vertical_position);
        } else {
            nav = nav.horizontal_position(self.horizontal_position);
        }

        nav = nav
            .tab_bar_end(self.tab_bar_end)
            .tab_bar_align(self.tab_bar_align)
            .all_caps(self.all_caps);

        if let Some(pad) = self.padding_for_mode() {
            nav = nav.padding(pad);
        }

        if let Some(pinned) = tab_pinned {
            nav = nav.tab_pinned(pinned);
        }

        nav = nav
            .style(Style::new().fg(dim).bg(bg))
            .highlight_style(Style::new().fg(highlight).bg(bg))
            .border_style(Style::new().fg(border_color).bg(bg))
            .selection_flash(self.tab_state.selection_flash_enabled);

        if self.show_indicator {
            nav.indicator(Some(INDICATOR))
        } else {
            nav.indicator(None)
        }
    }

    fn vertical_rail_width(&self) -> u16 {
        let labels_owned = self.tab_labels_owned_for_nav();
        let labels: Vec<&str> = labels_owned.iter().map(String::as_str).collect();
        let mut pin_buf = Vec::new();
        self.prepare_tab_nav(&labels, &mut pin_buf)
            .vertical_rail_width()
    }

    fn shortcut_footer_segments(&self) -> Vec<Vec<Span<'static>>> {
        let key = |s: &'static str| Span::styled(s, Style::new().fg(Color::Yellow));
        let dim = |s: &'static str| Span::styled(s, Style::new().fg(Color::DarkGray));

        let border_label = match self.border_kind {
            BorderKind::Rnd => "rnd",
            BorderKind::Sqr => "sqr",
        };
        let mode_label = match self.mode {
            DemoMode::Horizontal => "horizontal",
            DemoMode::Vertical => "vertical",
        };
        let indicator_label = if self.show_indicator { "on" } else { "off" };
        let padding_label = self.padding_label();
        let end_label = self.tab_bar_end_label();
        let caps_label = if self.all_caps { "on" } else { "off" };
        let overflow_label = self.overflow_label();
        let width_label = if self.narrow_tabs { "narrow" } else { "wide" };
        let wheel_label = if self.mouse_wheel { "on" } else { "off" };
        let click_label = if self.mouse_click { "on" } else { "off" };
        let reorder_label = self.reorder_policy_label();
        let position_label = self.position_label();
        let align_label = self.align_label();

        let nav = match self.mode {
            DemoMode::Horizontal => vec![key("←"), dim("/"), key("→"), dim(" tabs")],
            DemoMode::Vertical => vec![
                key("j"),
                dim("/"),
                key("k"),
                dim(" or "),
                key("↑"),
                dim("/"),
                key("↓"),
                dim(" tabs"),
            ],
        };

        vec![
            nav,
            vec![key("Tab"), dim(" cycle")],
            vec![
                key("M"),
                dim(" mode ("),
                Span::styled(mode_label, Style::new().fg(Color::DarkGray)),
                dim(")"),
            ],
            vec![
                key("I"),
                dim(" indicator ("),
                Span::styled(indicator_label, Style::new().fg(Color::DarkGray)),
                dim(")"),
            ],
            vec![
                key("B"),
                dim(" border ("),
                Span::styled(border_label, Style::new().fg(Color::DarkGray)),
                dim(")"),
            ],
            vec![
                key("1"),
                dim(" pad ("),
                Span::styled(padding_label, Style::new().fg(Color::DarkGray)),
                dim(")"),
            ],
            vec![
                key("2"),
                dim(" end ("),
                Span::styled(end_label, Style::new().fg(Color::DarkGray)),
                dim(")"),
            ],
            vec![
                key("C"),
                dim(" caps ("),
                Span::styled(caps_label, Style::new().fg(Color::DarkGray)),
                dim(")"),
            ],
            vec![
                key("O"),
                dim(" overflow ("),
                Span::styled(overflow_label, Style::new().fg(Color::DarkGray)),
                dim(")"),
            ],
            vec![
                key("W"),
                dim(" width ("),
                Span::styled(width_label, Style::new().fg(Color::DarkGray)),
                dim(")"),
            ],
            vec![
                key("Y"),
                dim(" wheel ("),
                Span::styled(wheel_label, Style::new().fg(Color::DarkGray)),
                dim(")"),
            ],
            vec![
                key("X"),
                dim(" click ("),
                Span::styled(click_label, Style::new().fg(Color::DarkGray)),
                dim(")"),
            ],
            vec![
                key("P"),
                dim(" position ("),
                Span::styled(position_label, Style::new().fg(Color::DarkGray)),
                dim(")"),
            ],
            vec![
                key("A"),
                dim(" align ("),
                Span::styled(align_label, Style::new().fg(Color::DarkGray)),
                dim(")"),
            ],
            vec![
                key("R"),
                dim(" reorder ("),
                Span::styled(reorder_label, Style::new().fg(Color::DarkGray)),
                dim(")"),
            ],
            vec![
                key("F"),
                dim(" flash ("),
                Span::styled(
                    if self.tab_state.selection_flash_enabled {
                        "on"
                    } else {
                        "off"
                    },
                    Style::new().fg(Color::DarkGray),
                ),
                dim(")"),
            ],
            vec![key("["), dim("/"), key("]"), dim(" scroll")],
            vec![key("q"), dim(" quit")],
        ]
    }

    fn shortcut_footer_lines(&self, width: u16) -> Vec<Line<'static>> {
        wrap_footer_segments(self.shortcut_footer_segments(), width.max(1) as usize)
    }

    fn content_block<'a>(&self, _title: &'a str, border_color: Color, bg: Color) -> Block<'a> {
        let mut block = Block::default()
            .border_set(self.content_border_set())
            .border_style(Style::new().fg(border_color))
            .style(Style::new().fg(Color::White).bg(bg));

        match self.mode {
            DemoMode::Vertical => {
                let borders = match self.vertical_position {
                    VerticalPosition::Left => Borders::TOP | Borders::RIGHT | Borders::BOTTOM,
                    VerticalPosition::Right => Borders::TOP | Borders::LEFT | Borders::BOTTOM,
                };
                block = block.borders(borders);
            }
            DemoMode::Horizontal => {
                let borders = match self.horizontal_position {
                    HorizontalPosition::Top => Borders::LEFT | Borders::RIGHT | Borders::BOTTOM,
                    HorizontalPosition::Bottom => Borders::LEFT | Borders::TOP | Borders::RIGHT,
                };
                block = block.borders(borders);
            }
        }

        block
    }

    fn paint_vertical_content_top_border(&self, area: Rect, buf: &mut Buffer, border_color: Color) {
        let title = self.selected_tab_name();
        let top = format!("─ {title} ");
        let style = Style::new().fg(border_color);
        let border_set = self.content_border_set();

        match self.vertical_position {
            VerticalPosition::Left => {
                for (offset, ch) in top.chars().enumerate() {
                    let x = area.x + offset as u16;
                    if x >= area.right() {
                        break;
                    }
                    buf[(x, area.y)].set_char(ch).set_style(style);
                }

                for x in (area.x + top.chars().count() as u16)..area.right() {
                    buf[(x, area.y)]
                        .set_symbol(border_set.horizontal_top)
                        .set_style(style);
                }

                if area.right() > area.x {
                    buf[(area.right() - 1, area.y)]
                        .set_symbol(border_set.top_right)
                        .set_style(style);
                }
            }
            VerticalPosition::Right => {
                let title_len = top.chars().count() as u16;
                let start_x = area.right().saturating_sub(title_len);
                for (offset, ch) in top.chars().enumerate() {
                    let x = start_x + offset as u16;
                    if x >= area.right() {
                        break;
                    }
                    buf[(x, area.y)].set_char(ch).set_style(style);
                }

                for x in area.x..start_x {
                    buf[(x, area.y)]
                        .set_symbol(border_set.horizontal_top)
                        .set_style(style);
                }

                if area.right() > area.x {
                    buf[(area.x, area.y)]
                        .set_symbol(border_set.top_left)
                        .set_style(style);
                }
            }
        }
    }

    fn render_content_pane(
        &self,
        area: Rect,
        buf: &mut Buffer,
        border_color: Color,
        bg: Color,
        status: &str,
    ) {
        let block = self.content_block(self.selected_tab_name(), border_color, bg);
        let inner = block.inner(area);
        block.render(area, buf);

        if self.mode == DemoMode::Vertical {
            self.paint_vertical_content_top_border(area, buf, border_color);
        }

        let shortcut_lines = self.shortcut_footer_lines(inner.width);
        let footer_height = shortcut_lines.len().clamp(1, 8) as u16;
        let shortcuts = Paragraph::new(Text::from(shortcut_lines)).alignment(Alignment::Center);

        let body_inner = if self.mode == DemoMode::Horizontal {
            let [title_area, body_area] =
                Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(inner);
            Line::from(format!(" {} ", self.selected_tab_name()))
                .style(Style::new().fg(border_color))
                .render(title_area, buf);
            body_area
        } else {
            inner
        };

        let [main, footer] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(footer_height)])
                .areas(body_inner);

        let [top_spacer, command_area, status_area, bottom_spacer] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Min(3),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(main);

        let _ = (top_spacer, bottom_spacer);

        Paragraph::new(self.last_command.as_str())
            .alignment(Alignment::Center)
            .style(Style::new().fg(Color::LightCyan))
            .render(command_area, buf);

        Paragraph::new(status)
            .alignment(Alignment::Center)
            .style(Style::new().fg(Color::DarkGray))
            .render(status_area, buf);

        shortcuts.render(footer, buf);
    }
}

impl App {
    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let bg = Color::Rgb(20, 20, 40);
        let border_color = Color::Rgb(60, 60, 100);

        Block::new()
            .style(Style::new().bg(bg))
            .render(area, frame.buffer_mut());

        let [header, body] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(area);

        "ratatui-comfy-tabs demo"
            .bold()
            .fg(Color::LightBlue)
            .into_centered_line()
            .render(header, frame.buffer_mut());

        match self.mode {
            DemoMode::Horizontal => self.render_horizontal(frame, body, bg, border_color),
            DemoMode::Vertical => self.render_vertical(frame, body, bg, border_color),
        }
    }

    fn render_horizontal(&mut self, frame: &mut Frame, area: Rect, bg: Color, border_color: Color) {
        let labels_owned = self.ordered_labels_owned();
        let labels: Vec<&str> = labels_owned.iter().map(String::as_str).collect();
        let mut pin_buf = Vec::new();
        let strip_height = self
            .prepare_tab_nav(&labels, &mut pin_buf)
            .horizontal_strip_height();
        let (tabs, content) = match self.horizontal_position {
            HorizontalPosition::Top => {
                let [tabs, content] =
                    Layout::vertical([Constraint::Length(strip_height), Constraint::Fill(1)])
                        .areas(area);
                (tabs, content)
            }
            HorizontalPosition::Bottom => {
                let [content, tabs] =
                    Layout::vertical([Constraint::Fill(1), Constraint::Length(strip_height)])
                        .areas(area);
                (tabs, content)
            }
        };

        let tab_area = if self.narrow_tabs {
            Rect {
                width: tabs.width.min(42),
                ..tabs
            }
        } else {
            tabs
        };

        self.tab_hit_area = tab_area;
        self.wheel_strip_area = tab_area;
        let mut pin_buf_render = Vec::new();
        let nav = self.prepare_tab_nav(&labels, &mut pin_buf_render);
        self.tab_state
            .ensure_selected_visible(&nav, self.tab_hit_area);
        StatefulWidget::render(nav, tab_area, frame.buffer_mut(), &mut self.tab_state);

        self.render_content_pane(
            content,
            frame.buffer_mut(),
            border_color,
            bg,
            &self.content_status_text(),
        );
    }

    fn content_status_text(&self) -> String {
        let drag = self
            .tab_state
            .reorder_drag
            .map(|d| format!(" · dragging tab {}", d.source))
            .unwrap_or_default();
        format!(
            "selected: {} · scroll_offset: {}{}",
            self.selected_tab_name(),
            self.tab_state.scroll_offset,
            drag
        )
    }

    fn render_vertical(&mut self, frame: &mut Frame, area: Rect, bg: Color, border_color: Color) {
        let rail_width = self.vertical_rail_width();
        let (tabs, content) = match self.vertical_position {
            VerticalPosition::Left => {
                let [tabs, content] =
                    Layout::horizontal([Constraint::Length(rail_width), Constraint::Fill(1)])
                        .areas(area);
                (tabs, content)
            }
            VerticalPosition::Right => {
                let [content, tabs] =
                    Layout::horizontal([Constraint::Fill(1), Constraint::Length(rail_width)])
                        .areas(area);
                (tabs, content)
            }
        };

        let tab_area = if self.narrow_tabs {
            Rect {
                height: tabs.height.min(14),
                ..tabs
            }
        } else {
            tabs
        };

        self.tab_hit_area = tab_area;
        self.wheel_strip_area = tab_area;
        let labels_owned = self.tab_labels_owned_for_nav();
        let labels: Vec<&str> = labels_owned.iter().map(String::as_str).collect();
        let mut pin_buf_render = Vec::new();
        let nav = self.prepare_tab_nav(&labels, &mut pin_buf_render);
        self.tab_state
            .ensure_selected_visible(&nav, self.tab_hit_area);
        StatefulWidget::render(nav, tab_area, frame.buffer_mut(), &mut self.tab_state);

        self.render_content_pane(
            content,
            frame.buffer_mut(),
            border_color,
            bg,
            &self.content_status_text(),
        );
    }
}
