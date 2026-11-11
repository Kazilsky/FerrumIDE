use crossterm::event::KeyCode;
use std::collections::HashMap;

use super::context::Context;
use super::command::{CommandRegistry, CommandFn};

pub struct Plugin {
    pub name: String,
    commands: Vec<(String, CommandFn)>,
    pub bindings: Vec<(KeyCode, String)>,
}

impl Plugin {
    pub fn new(name: String) -> Self {
        Self { 
            name: name, 
            commands: Vec::new(), 
            bindings: Vec::new() 
        }
    }

    pub fn command(mut self, name: String, func: CommandFn) -> Self {
        self.commands.push((name.to_string(), func));
        self
    }

    pub fn bind(mut self, key: KeyCode, command: String) -> Self {
        self.bindings.push((key, command));
        self
    }
}

pub struct PluginData {
    registry: CommandRegistry,
    bindings: HashMap<KeyCode, String>,
    plugins: Vec<Plugin>,
}

impl PluginData {
    pub fn new() -> Self {
        Self {
            registry: CommandRegistry::new(), // core-команды
            bindings: HashMap::new(),
            plugins: Vec::new(),
        }
    }

    pub fn load(&mut self, plugin: Plugin) {
        for (name, func) in &plugin.commands {
            self.registry.register(name, *func);
        }
        for (key, cmd) in &plugin.bindings {
            self.bindings.insert(*key, cmd.clone());
        }
        self.plugins.push(plugin); // последним, после всех обращений
    }
}

pub struct PluginManager<'a> {
    ctx: &'a mut Context,
    data: &'a PluginData,
}

impl PluginManager<'_> {
    pub fn exec(&mut self, name: &str) {
        if let Some(cmd) = self.data.registry.get(name) {
            cmd(self.ctx);
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) -> bool {
        match self.data.bindings.get(&key).cloned() {
            Some(name) => { self.exec(&name); true }
            None => false,
        }
    }
}