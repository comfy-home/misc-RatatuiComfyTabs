use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::{Buffer, Constraint, Layout, Rect, Stylize, Widget},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};
use ratatui_comfy_tabs::{TabNav, TabOrientation, vertical_label};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    ratatui::run(|terminal| App::default().run(terminal))?;
    Ok(())
}

const TAB_SOURCES: &[&str] = &["Home", "Logs", "Settings"];

const CONTENT: &[&str] = &[
    "Dashboard and status overview.",
    "Application logs appear in this panel.",
    "Configure preferences here.",
];

fn vertical_tabs() -> Vec<String> {
    TAB_SOURCES.iter().map(|label| vertical_label(label)).collect()
}

#[derive(Default)]
struct App {
    selected: usize,
}

impl App {
    fn run(mut self, terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),

                    KeyCode::BackTab => {
                        self.selected =
                            (self.selected + TAB_SOURCES.len() - 1) % TAB_SOURCES.len();
                    }
                    KeyCode::Tab => {
                        self.selected = (self.selected + 1) % TAB_SOURCES.len();
                    }

                    KeyCode::Up | KeyCode::Char('k') => {
                        self.selected = self.selected.saturating_sub(1);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.selected = (self.selected + 1).min(TAB_SOURCES.len() - 1);
                    }

                    _ => {}
                }
            }
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let bg = Color::Rgb(20, 20, 40);
        let highlight = Color::LightCyan;
        let dim = Color::DarkGray;
        let border = Color::Rgb(60, 60, 100);

        Block::new().style(Style::new().bg(bg)).render(area, buf);

        let labels = vertical_tabs();
        let label_refs: Vec<&str> = labels.iter().map(String::as_str).collect();
        let rail_width = label_refs
            .iter()
            .map(|label| label.lines().map(|line| line.len()).max().unwrap_or(0) as u16 + 8)
            .max()
            .unwrap_or(8);

        let [header, body] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(area);

        "vertical tabs — j/k or ↑/↓ to navigate, q to quit"
            .dim()
            .into_centered_line()
            .render(header, buf);

        let [tabs, content] =
            Layout::horizontal([Constraint::Length(rail_width), Constraint::Fill(1)]).areas(body);

        TabNav::new(&label_refs, self.selected)
            .orientation(TabOrientation::Vertical)
            .style(Style::new().fg(dim).bg(bg))
            .highlight_style(Style::new().fg(highlight).bg(bg))
            .border_style(Style::new().fg(border).bg(bg))
            .render(tabs, buf);

        Paragraph::new(CONTENT[self.selected])
            .block(
                Block::bordered()
                    .title(format!(" {} ", TAB_SOURCES[self.selected]))
                    .border_style(Style::new().fg(border))
                    .style(Style::new().fg(Color::White).bg(bg)),
            )
            .render(content, buf);
    }
}
