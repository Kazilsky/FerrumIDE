use std::time::{Duration, Instant};
use tokio::time;
use tokio::sync::mpsc;
use crate::manager::Hertz;

pub struct TimerEvent {
    timer_name: String,
    command: String
}

pub struct Timer {
    name: String,
    hz: Option<Hertz>,
    command: Option<String>,
    started: bool,
}

impl Timer {
    pub fn new(name: String, command: Option<String>) -> Self {
        Self {
            name, 
            hz: Some(Hertz::new(1)),
            command: command,
            started: false
        }
    }

    pub fn start_timer(&mut self, tx: mpsc::UnboundedSender<TimerEvent>) {
        if let Some(mut hz) = self.hz.take() {
            self.started = true;

            let timer_name = self.name.clone();
            let command = self.command.clone();

            tokio::spawn(async move {
                let mut interval = time::interval(Duration::from_millis(1));

                loop {
                    interval.tick().await;

                    if hz.update() {
                        if let Some(ref cmd) = command {
                            let event = TimerEvent {
                                timer_name: timer_name.clone(),
                                command: cmd.clone(),
                            };

                            // Отправляем в Manager. Если Manager закрылся — выходим из цикла
                            if tx.send(event).is_err() {
                                break;
                            }
                        }
                    }
                }
            });
        }
    }
}