use color_eyre::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;

mod manager;
use manager::Manager;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    terminal::enable_raw_mode()?;

    let mut out = io::stdout();  
    execute!(out, EnterAlternateScreen, cursor::Hide)?;

    let mut app = App::new();
    let res = app.run();

    execute!(out, cursor::Show, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    res
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

    fn run(&mut self) -> Result<()> {
        self.manager.prelude();

        loop {
            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.kind == KeyEventKind::Press {
                        let keep_running = self.manager.handle_key(key_event.code);
                        if !keep_running {
                            break;
                        }
                    }
                }
            }
            self.manager.process_command_timers();
        }
        Ok(())
    }
}
