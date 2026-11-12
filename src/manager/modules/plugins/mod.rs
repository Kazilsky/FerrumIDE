use crossterm::event::KeyCode;
use crate::manager::{CommandRegistry, modules::plugins};
use super::context::Context;

pub mod autopair;
pub use autopair::autopair;


// I don't know how use Box, I use it for create a SOME plugin, it's not necesarry but I CAN put plugin in this Box
// I don't know it's good practice or not :d
// For me, yes
pub type AnyPlugin = Box<dyn Plugin>;


// I don't know how create trait, so I just google most like
pub trait Plugin {
    // For my idea on_load it's not primary attribute
    // I can create plugin only with on_key or else with commands, it's also good
    fn on_load(&mut self, _commands: &mut CommandRegistry) {}
    fn on_key(&mut self, _key: KeyCode, _ctx: &mut Context, _commands: &CommandRegistry) -> bool {
        false
    }
}

pub struct PluginManager {
    plugins: Vec<AnyPlugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn load(&mut self, mut plugin: AnyPlugin, commands: &mut CommandRegistry) {
        plugin.on_load(commands);
        self.plugins.push(plugin);
    }

    pub fn handle_key(&mut self, key: KeyCode, ctx: &mut Context, commands: &CommandRegistry) -> bool {
        for plugin in &mut self.plugins {
            if plugin.on_key(key, ctx, commands) {
                return true;
            }
        }
        false
    }
}