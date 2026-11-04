use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Position};
use ratatui::symbols::border;
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph, Borders};
use ratatui::{DefaultTerminal, Frame};

use ratatui::border;
use ratatui::layout::Direction;

use ratatui::text::{Line, Text};

// Import Own Files
mod modules;

use modules::buffer::Buffer as IDEBuffer;

enum InputMode {
    Normal,
    Editing,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::new().run(terminal))
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input_mode: InputMode,
    /// Initialize Editor
    buffer: IDEBuffer,
}

impl App {
    fn new() -> Self {
        Self {
            input_mode: InputMode::Normal,
            buffer: IDEBuffer::new(),
        }
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            self.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => self.buffer.line_break(),
                        KeyCode::Char(to_insert) => self.buffer.enter_char(to_insert),
                        KeyCode::Backspace => self.buffer.delete_char(),
                        KeyCode::Left => self.buffer.move_cursor_left(),
                        KeyCode::Right => self.buffer.move_cursor_right(),
                        KeyCode::Esc => self.input_mode = InputMode::Normal,
                        _ => {}
                    },
                }
            }
        }
    }

    fn status_line_name(&self, mode: &InputMode) -> String {
        match mode {
            InputMode::Editing => "test".to_string(),
            InputMode::Normal => "asd".to_string(),
        }
    }

    fn render(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(frame.area());

        // Каждую строку вектора оборачиваем в ListItem
        let items: Vec<ListItem> = self.buffer.strokes_array
            .iter()
            .map(|line| ListItem::new(line.as_str()))
            .collect();

        let list_widget = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(" Editor "));

        let b = Block::default()
            .title(self.status_line_name(&self.input_mode))
            .borders(Borders::ALL);

        // Рендерим строго по своим местам!
        frame.render_widget(list_widget, chunks[0]); // Код — в chunks[0]
        frame.render_widget(b, chunks[1]);          // Статус — в chunks[1]
    }
}
