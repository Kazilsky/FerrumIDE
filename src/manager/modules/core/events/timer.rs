use std::time::{Duration, Instant};
use super::command::CommandRegistry;
use crate::manager::modules::context::Context;

pub struct TimerTask {
    pub id: &'static str,
    pub command: &'static str, 
    pub trigger_at: Instant,   
    pub interval: Option<Duration>, 
}

pub struct TimerManager {
    tasks: Vec<TimerTask>,
}

impl TimerManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn set_timeout(&mut self, id: &'static str, delay: Duration, command: &'static str) {
        self.cancel(id);
        self.tasks.push(TimerTask {
            id,
            command,
            trigger_at: Instant::now() + delay,
            interval: None,
        });
    }

    pub fn set_interval(&mut self, id: &'static str, interval: Duration, command: &'static str) {
        self.cancel(id);
        self.tasks.push(TimerTask {
            id,
            command,
            trigger_at: Instant::now() + interval,
            interval: Some(interval),
        });
    }

    pub fn cancel(&mut self, id: &'static str) {
        self.tasks.retain(|t| t.id != id);
    }

    pub fn tick(&mut self, ctx: &mut Context, commands: &CommandRegistry) {
        let now = Instant::now();
        let mut commands_to_run = Vec::new();

        self.tasks.retain_mut(|task| {
            if now >= task.trigger_at {
                commands_to_run.push(task.command);

                if let Some(interval) = task.interval {
                    task.trigger_at = now + interval;
                    true 
                } else {
                    false 
                }
            } else {
                true 
            }
        });

        for cmd in commands_to_run {
            commands.execute(cmd, ctx);
        }
    }
}
