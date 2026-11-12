use std::time::Duration;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;

mod manager;
use manager::Manager;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = App::new();

    ratatui::run(|terminal| app.run(terminal))
}

struct App {
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

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.kind == KeyEventKind::Press {
                        let keep_running = self.manager.handle_key(key_event.code);

                        if !keep_running {
                            break;
                        }
                    }
                }
            }

            self.manager.tick();
        }

        ratatui::restore();
        Ok(())
    }
}
