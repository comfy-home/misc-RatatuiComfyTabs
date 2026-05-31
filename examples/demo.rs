//! Interactive demo for ratatui-comfy-tabs.
//!
//! Keys: `h`/`l` or ←/→ (horizontal), `j`/`k` or ↑/↓ (vertical), `Tab`/`BackTab` cycle,
//! `M` toggle horizontal/vertical mode, `I` toggle active-tab indicator, `q` quit.

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::Alignment,
    prelude::{Buffer, Constraint, Layout, Rect, Stylize, Widget},
    style::{Color, Style},
    symbols::border,
    widgets::{Block, Borders, Paragraph},
};
use ratatui_comfy_tabs::{TabNav, TabOrientation, vertical_label};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    ratatui::run(|terminal| App::default().run(terminal))?;
    Ok(())
}

const TABS: &[&str] = &[
    "Overview",
    "Nodes",
    "Network",
    "Content",
    "Inference",
    "Config",
    "Logs",
];

const INDICATOR: &str = "▸";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum DemoMode {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Default)]
struct App {
    selected: usize,
    mode: DemoMode,
    show_indicator: bool,
    vertical_labels: Vec<String>,
}

impl App {
    fn run(mut self, terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
        self.vertical_labels = TABS.iter().map(|label| vertical_label(label)).collect();

        loop {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;

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

                    KeyCode::BackTab => {
                        self.selected = (self.selected + TABS.len() - 1) % TABS.len();
                    }
                    KeyCode::Tab => {
                        self.selected = (self.selected + 1) % TABS.len();
                    }

                    KeyCode::Left | KeyCode::Char('h') if self.mode == DemoMode::Horizontal => {
                        self.selected = self.selected.saturating_sub(1);
                    }
                    KeyCode::Right | KeyCode::Char('l') if self.mode == DemoMode::Horizontal => {
                        self.selected = (self.selected + 1).min(TABS.len() - 1);
                    }

                    KeyCode::Up | KeyCode::Char('k') if self.mode == DemoMode::Vertical => {
                        self.selected = self.selected.saturating_sub(1);
                    }
                    KeyCode::Down | KeyCode::Char('j') if self.mode == DemoMode::Vertical => {
                        self.selected = (self.selected + 1).min(TABS.len() - 1);
                    }

                    _ => {}
                }
            }
        }
    }

    fn styled_tab_nav<'a>(&self, tabs: &'a [&'a str]) -> TabNav<'a> {
        let bg = Color::Rgb(20, 20, 40);
        let highlight = Color::LightBlue;
        let dim = Color::DarkGray;
        let border = Color::Rgb(60, 60, 100);

        let mut nav = TabNav::new(tabs, self.selected);
        if self.mode == DemoMode::Vertical {
            nav = nav.orientation(TabOrientation::Vertical);
        }

        nav = nav
            .style(Style::new().fg(dim).bg(bg))
            .highlight_style(Style::new().fg(highlight).bg(bg))
            .border_style(Style::new().fg(border).bg(bg));

        if self.show_indicator {
            nav.indicator(Some(INDICATOR))
        } else {
            nav.indicator(None)
        }
    }

    fn vertical_rail_width(&self) -> u16 {
        self.vertical_labels
            .iter()
            .map(|label| label.lines().map(|line| line.len()).max().unwrap_or(0) as u16 + 8)
            .max()
            .unwrap_or(8)
    }

    fn shortcut_footer_text(&self) -> String {
        let mode_label = match self.mode {
            DemoMode::Horizontal => "horizontal",
            DemoMode::Vertical => "vertical",
        };
        let indicator_label = if self.show_indicator { "on" } else { "off" };
        let nav_keys = match self.mode {
            DemoMode::Horizontal => "h/l or ←/→",
            DemoMode::Vertical => "j/k or ↑/↓",
        };

        format!(
            "{nav_keys} tabs | Tab cycle | M mode ({mode_label}) | I indicator ({indicator_label}) | q quit"
        )
    }

    fn content_block<'a>(&self, title: &'a str, border: Color, bg: Color) -> Block<'a> {
        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::new().fg(border))
            .style(Style::new().fg(Color::White).bg(bg));

        if self.mode == DemoMode::Vertical {
            let mut border_set = border::ROUNDED;
            border_set.top_left = "─";
            block = block
                .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM)
                .border_set(border_set)
                .title(format!(" {} ", title))
                .title_alignment(Alignment::Left);
        } else {
            block = block.title(format!(" {} ", title));
        }

        block
    }

    fn render_content_pane(
        &self,
        area: Rect,
        buf: &mut Buffer,
        border: Color,
        bg: Color,
        body: &str,
    ) {
        let block = self.content_block(TABS[self.selected], border, bg);
        let inner = block.inner(area);
        block.render(area, buf);

        if self.mode == DemoMode::Vertical {
            let style = Style::new().fg(border);
            buf[(area.x, area.y)].set_symbol("─").set_style(style);
        }

        let [main, footer] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(inner);

        Paragraph::new(body).render(main, buf);
        self.shortcut_footer_text()
            .dim()
            .into_centered_line()
            .render(footer, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let bg = Color::Rgb(20, 20, 40);
        let border = Color::Rgb(60, 60, 100);

        Block::new().style(Style::new().bg(bg)).render(area, buf);

        let [header, body] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(area);

        "ratatui-comfy-tabs demo"
            .bold()
            .fg(Color::LightBlue)
            .into_centered_line()
            .render(header, buf);

        match self.mode {
            DemoMode::Horizontal => self.render_horizontal(body, buf, bg, border),
            DemoMode::Vertical => self.render_vertical(body, buf, bg, border),
        }
    }
}

impl App {
    fn render_horizontal(&self, area: Rect, buf: &mut Buffer, bg: Color, border: Color) {
        let [tabs, content] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);

        self.styled_tab_nav(TABS).render(tabs, buf);

        self.render_content_pane(
            content,
            buf,
            border,
            bg,
            &format!("Selected: {}", TABS[self.selected]),
        );
    }

    fn render_vertical(&self, area: Rect, buf: &mut Buffer, bg: Color, border: Color) {
        let rail_width = self.vertical_rail_width();
        let [tabs, content] =
            Layout::horizontal([Constraint::Length(rail_width), Constraint::Fill(1)]).areas(area);

        let label_refs: Vec<&str> = self.vertical_labels.iter().map(String::as_str).collect();
        self.styled_tab_nav(&label_refs).render(tabs, buf);

        self.render_content_pane(
            content,
            buf,
            border,
            bg,
            &format!("Selected: {}", TABS[self.selected]),
        );
    }
}
