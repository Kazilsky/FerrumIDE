use ratatui::Frame;

use ratatui::layout::{Constraint, Direction, Layout, Position};

use ratatui::widgets::{Block, Borders, Paragraph};

use ratatui::text::Line;

pub struct UI {}

pub enum InputMode {
    Normal,
    Insert,
}

impl UI {
    pub fn new() -> Self {
        Self {}
    }

    fn status_line_name(&self, mode: &InputMode) -> String {
        match mode {
            InputMode::Insert => "Insert".to_string(),
            InputMode::Normal => "Normal".to_string(),
        }
    }

    pub fn render(
        &self,
        frame: &mut Frame,
        strokes_list: &Vec<String>,
        char_index: &u16,
        present_stroke: &u16,
    ) {
        // SPLIT SCREEN
        let chunks_vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(frame.area());
        let chunks_horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(frame.area());

        let mut lines = Vec::new();

        for s in strokes_list {
            lines.push(Line::raw(s.as_str()));
        }

        let line_widget =
            Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" Editor "));

        // Рендерим строго по своим местам!
        frame.render_widget(line_widget, chunks_vertical[0]); // Код — в chunks[0]
        frame.set_cursor_position(Position::new(
            chunks_vertical[0].x + char_index + 1,
            chunks_vertical[0].y + present_stroke + 1,
        ));
    }
}
