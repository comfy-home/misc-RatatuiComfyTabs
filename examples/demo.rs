//! Interactive demo for ratatui-comfy-tabs.
//!
//! Run: `cargo run --example demo`

use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::Alignment,
    prelude::{Buffer, Constraint, Layout, Rect, Stylize, Widget},
    style::{Color, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use ratatui_comfy_tabs::{
    OverflowPolicy, TabAxis, TabBarEnd, TabDirection, TabNav, TabNavState, TabOrientation,
    TabPadding, vertical_label,
};
use ratatui_core::widgets::StatefulWidget;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    ratatui::run(|terminal| App::default().run(terminal))?;
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
    Rounded,
    Square,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum PaddingPreset {
    #[default]
    Default,
    Alt2,
    Alt3,
}

#[derive(Default)]
struct App {
    tab_state: TabNavState,
    mode: DemoMode,
    border_kind: BorderKind,
    show_indicator: bool,
    padding_preset: PaddingPreset,
    tab_bar_end: TabBarEnd,
    all_caps: bool,
    overflow: OverflowPolicy,
    narrow_tabs: bool,
    vertical_labels: Vec<String>,
    last_tab_strip_area: Rect,
}

impl App {
    fn run(mut self, terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
        self.vertical_labels = TABS.iter().map(|label| vertical_label(label)).collect();

        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),

                    KeyCode::Char('m') | KeyCode::Char('M') => {
                        self.mode = match self.mode {
                            DemoMode::Horizontal => DemoMode::Vertical,
                            DemoMode::Vertical => DemoMode::Horizontal,
                        };
                    }

                    KeyCode::Char('i') | KeyCode::Char('I') => {
                        self.show_indicator = !self.show_indicator;
                    }

                    KeyCode::Char('b') | KeyCode::Char('B') => {
                        self.border_kind = match self.border_kind {
                            BorderKind::Rounded => BorderKind::Square,
                            BorderKind::Square => BorderKind::Rounded,
                        };
                    }

                    KeyCode::Char('1') => {
                        self.padding_preset = match self.padding_preset {
                            PaddingPreset::Default => PaddingPreset::Alt2,
                            PaddingPreset::Alt2 => PaddingPreset::Alt3,
                            PaddingPreset::Alt3 => PaddingPreset::Default,
                        };
                    }

                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        self.all_caps = !self.all_caps;
                    }

                    KeyCode::Char('2') => {
                        self.tab_bar_end = match self.tab_bar_end {
                            TabBarEnd::NoEnd => TabBarEnd::Angl,
                            TabBarEnd::Angl => TabBarEnd::Rnd,
                            TabBarEnd::Rnd => TabBarEnd::NoEnd,
                        };
                    }

                    KeyCode::Char('o') | KeyCode::Char('O') => {
                        self.overflow = match self.overflow {
                            OverflowPolicy::Truncate => OverflowPolicy::Scroll,
                            OverflowPolicy::Scroll => OverflowPolicy::Truncate,
                        };
                        self.tab_state.scroll_offset = 0;
                    }

                    KeyCode::Char('w') | KeyCode::Char('W') => {
                        self.narrow_tabs = !self.narrow_tabs;
                    }

                    KeyCode::BackTab => {
                        self.tab_state
                            .select_direction_wrapping(TabDirection::Previous, TABS.len());
                    }
                    KeyCode::Tab => {
                        self.tab_state
                            .select_direction_wrapping(TabDirection::Next, TABS.len());
                    }

                    KeyCode::Left | KeyCode::Char('h') if self.mode == DemoMode::Horizontal => {
                        self.tab_state
                            .select_direction(TabAxis::Decrease.direction(), TABS.len());
                    }
                    KeyCode::Right | KeyCode::Char('l') if self.mode == DemoMode::Horizontal => {
                        self.tab_state
                            .select_direction(TabAxis::Increase.direction(), TABS.len());
                    }

                    KeyCode::Up | KeyCode::Char('k') if self.mode == DemoMode::Vertical => {
                        self.tab_state
                            .select_direction(TabAxis::Decrease.direction(), TABS.len());
                    }
                    KeyCode::Down | KeyCode::Char('j') if self.mode == DemoMode::Vertical => {
                        self.tab_state
                            .select_direction(TabAxis::Increase.direction(), TABS.len());
                    }

                    KeyCode::Char('[') => {
                        self.tab_state.scroll_prev();
                    }
                    KeyCode::Char(']') => {
                        self.tab_state
                            .scroll_next(&self.styled_tab_nav(TABS), self.last_tab_strip_area);
                    }

                    _ => {}
                }
            }
        }
    }

    fn tab_border_set(&self) -> border::Set<'static> {
        match self.border_kind {
            BorderKind::Rounded => border::ROUNDED,
            BorderKind::Square => border::PLAIN,
        }
    }

    fn content_border_set(&self) -> border::Set<'static> {
        let mut set = self.tab_border_set();
        if self.mode == DemoMode::Vertical {
            set.top_left = "─";
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
            TabBarEnd::Angl => "angl",
            TabBarEnd::Rnd => "rnd",
        }
    }

    fn overflow_label(&self) -> &'static str {
        match self.overflow {
            OverflowPolicy::Truncate => "truncate",
            OverflowPolicy::Scroll => "scroll",
        }
    }

    fn styled_tab_nav<'a>(&self, tabs: &'a [&'a str]) -> TabNav<'a> {
        let bg = Color::Rgb(20, 20, 40);
        let highlight = Color::LightBlue;
        let dim = Color::DarkGray;
        let border_color = Color::Rgb(60, 60, 100);

        let mut nav = TabNav::new(tabs, self.tab_state.selected)
            .border_set(self.tab_border_set())
            .overflow(self.overflow);
        if self.mode == DemoMode::Vertical {
            nav = nav.orientation(TabOrientation::Vertical);
        }

        nav = nav.tab_bar_end(self.tab_bar_end).all_caps(self.all_caps);

        if let Some(pad) = self.padding_for_mode() {
            nav = nav.padding(pad);
        }

        nav = nav
            .style(Style::new().fg(dim).bg(bg))
            .highlight_style(Style::new().fg(highlight).bg(bg))
            .border_style(Style::new().fg(border_color).bg(bg));

        if self.show_indicator {
            nav.indicator(Some(INDICATOR))
        } else {
            nav.indicator(None)
        }
    }

    fn vertical_rail_width(&self) -> u16 {
        let label_refs: Vec<&str> = self.vertical_labels.iter().map(String::as_str).collect();
        self.styled_tab_nav(&label_refs).vertical_rail_width()
    }

    fn shortcut_footer_line(&self) -> Line<'static> {
        let key = |s: &'static str| Span::styled(s, Style::new().fg(Color::Yellow));
        let dim = |s: &'static str| Span::styled(s, Style::new().fg(Color::DarkGray));

        let border_label = match self.border_kind {
            BorderKind::Rounded => "rounded",
            BorderKind::Square => "square",
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

        let mut spans = Vec::new();

        match self.mode {
            DemoMode::Horizontal => {
                spans.extend([
                    key("h"),
                    dim("/"),
                    key("l"),
                    dim(" or "),
                    key("←"),
                    dim("/"),
                    key("→"),
                ]);
            }
            DemoMode::Vertical => {
                spans.extend([
                    key("j"),
                    dim("/"),
                    key("k"),
                    dim(" or "),
                    key("↑"),
                    dim("/"),
                    key("↓"),
                ]);
            }
        }

        spans.extend([
            dim(" tabs | "),
            key("Tab"),
            dim(" cycle | "),
            key("M"),
            dim(" mode ("),
            Span::styled(mode_label, Style::new().fg(Color::DarkGray)),
            dim(") | "),
            key("I"),
            dim(" indicator ("),
            Span::styled(indicator_label, Style::new().fg(Color::DarkGray)),
            dim(") | "),
            key("B"),
            dim(" border ("),
            Span::styled(border_label, Style::new().fg(Color::DarkGray)),
            dim(") | "),
            key("1"),
            dim(" pad ("),
            Span::styled(padding_label, Style::new().fg(Color::DarkGray)),
            dim(") | "),
            key("2"),
            dim(" end ("),
            Span::styled(end_label, Style::new().fg(Color::DarkGray)),
            dim(") | "),
            key("C"),
            dim(" caps ("),
            Span::styled(caps_label, Style::new().fg(Color::DarkGray)),
            dim(") | "),
            key("O"),
            dim(" overflow ("),
            Span::styled(overflow_label, Style::new().fg(Color::DarkGray)),
            dim(") | "),
            key("W"),
            dim(" width ("),
            Span::styled(width_label, Style::new().fg(Color::DarkGray)),
            dim(") | "),
            key("["),
            dim("/"),
            key("]"),
            dim(" scroll | "),
            key("q"),
            dim(" quit"),
        ]);

        Line::from(spans)
    }

    fn content_block<'a>(&self, title: &'a str, border_color: Color, bg: Color) -> Block<'a> {
        let mut block = Block::default()
            .border_set(self.content_border_set())
            .border_style(Style::new().fg(border_color))
            .style(Style::new().fg(Color::White).bg(bg));

        if self.mode == DemoMode::Vertical {
            block = block.borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM);
        } else {
            block = block.borders(Borders::ALL).title(format!(" {} ", title));
        }

        block
    }

    fn paint_vertical_content_top_border(&self, area: Rect, buf: &mut Buffer, border_color: Color) {
        let title = TABS[self.tab_state.selected];
        let top = format!("─ {title} ");
        let style = Style::new().fg(border_color);
        let border_set = self.content_border_set();

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

    fn render_content_pane(
        &self,
        area: Rect,
        buf: &mut Buffer,
        border_color: Color,
        bg: Color,
        body: &str,
    ) {
        let block = self.content_block(TABS[self.tab_state.selected], border_color, bg);
        let inner = block.inner(area);
        block.render(area, buf);

        if self.mode == DemoMode::Vertical {
            self.paint_vertical_content_top_border(area, buf, border_color);
        }

        let [main, footer] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(inner);

        Paragraph::new(body).render(main, buf);

        Paragraph::new(self.shortcut_footer_line())
            .alignment(Alignment::Center)
            .render(footer, buf);
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
        let strip_height = self.styled_tab_nav(TABS).horizontal_strip_height();
        let [tabs, content] =
            Layout::vertical([Constraint::Length(strip_height), Constraint::Fill(1)]).areas(area);

        let tab_area = if self.narrow_tabs {
            Rect {
                width: tabs.width.min(42),
                ..tabs
            }
        } else {
            tabs
        };

        self.last_tab_strip_area = tab_area;
        let nav = self.styled_tab_nav(TABS);
        self.tab_state
            .ensure_selected_visible(&nav, self.last_tab_strip_area);
        StatefulWidget::render(nav, tab_area, frame.buffer_mut(), &mut self.tab_state);

        self.render_content_pane(
            content,
            frame.buffer_mut(),
            border_color,
            bg,
            &format!(
                "Selected: {} (scroll {})",
                TABS[self.tab_state.selected], self.tab_state.scroll_offset
            ),
        );
    }

    fn render_vertical(&mut self, frame: &mut Frame, area: Rect, bg: Color, border_color: Color) {
        let rail_width = self.vertical_rail_width();
        let [tabs, content] =
            Layout::horizontal([Constraint::Length(rail_width), Constraint::Fill(1)]).areas(area);

        let tab_area = if self.narrow_tabs {
            Rect {
                height: tabs.height.min(14),
                ..tabs
            }
        } else {
            tabs
        };

        let label_refs: Vec<&str> = self.vertical_labels.iter().map(String::as_str).collect();
        self.last_tab_strip_area = tab_area;
        let nav = self.styled_tab_nav(&label_refs);
        self.tab_state
            .ensure_selected_visible(&nav, self.last_tab_strip_area);
        StatefulWidget::render(nav, tab_area, frame.buffer_mut(), &mut self.tab_state);

        self.render_content_pane(
            content,
            frame.buffer_mut(),
            border_color,
            bg,
            &format!(
                "Selected: {} (scroll {})",
                TABS[self.tab_state.selected], self.tab_state.scroll_offset
            ),
        );
    }
}
