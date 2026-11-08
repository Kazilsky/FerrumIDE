use color_eyre::Result;
use crossterm::event;
use crossterm::event::Event;

use ratatui::DefaultTerminal;

// Import manager orchestrator
mod manager;

use manager::Manager;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = App::new();

    ratatui::run(|terminal| app.run(terminal))
}

/// App holds the state of the application
struct App {
    /// Initialize Manager
    /// Manager holds a much param for App, like cursor, buffer
    /// [`Buffer`, 'Cursor']
    manager: Manager,
}

impl App {
    fn new() -> Self {
        Self {
            manager: Manager::new(),
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.manager.prelude();
        
        loop {
            terminal.draw(|frame| self.manager.render_ui(frame))?;

            if let Event::Key(key_event) = event::read()? {
                let keep_running = self.manager.handle_key(key_event.code);

                // If handle_key return false — exit from for
                if !keep_running {
                    break; // Break app
                }
            }
        }

        ratatui::restore();
        Ok(())
    }
}
