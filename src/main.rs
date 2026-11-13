use color_eyre::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, BufWriter, Stdout};
use std::time::Duration;

mod manager;
use manager::Manager;

fn main() -> Result<()> {
    color_eyre::install()?;

    terminal::enable_raw_mode()?;
    let mut stdout = BufWriter::new(io::stdout());
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    let mut app = App::new();
    let res = app.run(&mut stdout);

    execute!(stdout, cursor::Show, LeaveAlternateScreen)?;
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

    fn run(&mut self, stdout: &mut BufWriter<Stdout>) -> Result<()> {
        self.manager.prelude();

        loop {
            let new_ticks = self.manager.hz.update();
            // Внутри loop в main.rs:

            self.manager.tick();    

            if new_ticks > 0 && self.manager.hz.total_ticks % 16 == 0 {
                self.manager.render_ui(stdout);
            }

            if event::poll(Duration::from_millis(1))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.kind == KeyEventKind::Press {
                        let keep_running = self.manager.handle_key(key_event.code);

                        if !keep_running {
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
