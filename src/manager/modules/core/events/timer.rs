use std::time::{Duration, Instant};
use std::todo;
use tokio::time;
use tokio::sync::mpsc;
use crate::manager::Hertz;

pub struct TimerEvent {
    pub timer_name: String,
    pub action_type: Option<TimerAction>,
    pub command: String,
}

pub enum TimerAction {
    Command,
    Return 
}

pub struct Timer {
    name: String,
    hz: Option<Hertz>,
    action: Option<TimerAction>,
    action_param: Option<String>,
    started: bool,
}

impl Timer {
    pub fn new(name: String, action: Option<TimerAction>, action_param: Option<String>, hz: Option<u32>) -> Self {
        let mut format_hz = 0; 

        if let Some(hz) = hz {
            format_hz = hz
        } else {
            format_hz = 1
        }
        
        Self {
            name, 
            hz: Some(Hertz::new(format_hz)),
            action,
            action_param: action_param,
            started: false
        }
    }

    pub fn action_type(&self) -> &str {
        if let Some(action_type) = &self.action {
            match action_type {
                TimerAction::Command => return "command",
                TimerAction::Return => return "return_timer"
            }
        }

        return "Whattheshit";
    }

    pub fn start_timer(&mut self, tx: mpsc::UnboundedSender<TimerEvent>) {
        if let Some(mut hz) = self.hz.take() {
            self.started = true;

            let timer_name = self.name.clone();
            let command = self.action_param.clone();
            let action_type = self.action_type().to_owned();

            // Ofc I don't have so much knowledge for MPSC, so I use a google search AI for help me :P
            tokio::spawn(async move {
                let mut interval = time::interval(Duration::from_millis(1));

                loop {
                    interval.tick().await;
                    if hz.update() {
                        if action_type == "command" {
                            if let Some(ref cmd) = command {
                                let event = TimerEvent {
                                    timer_name: timer_name.clone(),
                                    action_type: Some(TimerAction::Command),
                                    command: cmd.clone(),
                                };

                                if tx.send(event).is_err() {
                                    break;
                                }
                            }
                        } else if action_type == "return_timer" {
                            if let Some(ref cmd) = command {
                                let event = TimerEvent {
                                    timer_name: timer_name.clone(),
                                    action_type: Some(TimerAction::Return),
                                    command: cmd.clone(),
                                };

                                if tx.send(event).is_err() {
                                    break;
                                }
                            }
                        }
                    }
                }
            });
        }
    }

    pub fn created_simple_timer(name: String, action: Option<TimerAction>, action_param: Option<String>, hz: Option<u32>, tx: mpsc::UnboundedSender<TimerEvent>) -> Self { 
        // By default, start_timer creates a timer with a frequency of 1 Hz. 
        // A note for my future, incompetent self: 
        // Hz indicates how many actions occur per second! 
        // 1 Hz means one `println` per second; 2 Hz means two `println`s per second.

        // In future I add Hz set for future features :)
        
        let mut timer = Self::new(name, action, action_param, hz);
        timer.start_timer(tx);
        timer
    }
}
