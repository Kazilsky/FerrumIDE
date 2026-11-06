use color_eyre::Result;
use ratatui::{DefaultTerminal};

// Import Own Files
mod modules;

use modules::buffer::Buffer as IDEBuffer;
use modules::view::View;
use modules::ui::UI;



fn main() -> Result<()> {
    color_eyre::install()?;
    let app = App::new();

    ratatui::run(
        |terminal| 
        app.run(terminal)
    )
}

/// App holds the state of the application
struct App {
    /// Initialize Editor
    buffer: IDEBuffer,
    /// Initialize UI
    ui: UI,
    /// 
    view: View
}

impl App {
    fn new() -> Self {
        Self {
            buffer: IDEBuffer::new(),
            ui: UI::new(),
            view: View::new()
        }
    }

    fn run(&self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(
                |frame| 
                self.ui.render(frame)
            )?;   
        }
    }
}
