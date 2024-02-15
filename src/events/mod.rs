use std::collections::HashMap;

use crate::{command::Command, message::Message};

pub type EventCallbackFn<T> = fn(&T);

pub struct EventCallbacksHandler<T> {
    callbacks: Vec<EventCallbackFn<T>>,
}

impl<T> EventCallbacksHandler<T> {
    pub fn new() -> Self {
        Self { callbacks: vec![] }
    }

    pub fn add(&mut self, callback: EventCallbackFn<T>) {
        self.callbacks.push(callback)
    }

    pub fn execute(&self, args: &T) {
        for callback in &self.callbacks {
            callback(args);
        }
    }
}

pub struct EventHandler<'a> {
    message: EventCallbacksHandler<Message<'a>>,
    commands: HashMap<String, EventCallbacksHandler<Command<'a>>>,
}

impl<'a> EventHandler<'a> {
    pub fn new() -> Self {
        Self {
            message: EventCallbacksHandler::new(),
            commands: HashMap::new(),
        }
    }

    pub fn on_message(&mut self, cb: EventCallbackFn<Message<'a>>) {
        self.message.add(cb);
    }

    pub fn on_command(&mut self, command: String, cb: EventCallbackFn<Command<'a>>) {
        if !self.commands.contains_key(&command) {
            self.commands
                .insert(command.clone(), EventCallbacksHandler::new());
        }

        let handler = self.commands.get_mut(&command).unwrap();

        handler.add(cb)
    }

    pub fn has_command(&self, command: &String) -> bool {
        self.commands.contains_key(command)
    }

    pub fn execute_message(&self, message: Message<'a>) {
        self.message.execute(&message)
    }

    pub fn execute_command(&self, command: &Command<'a>) {
        let handler = self.commands.get(&command.name).unwrap();

        handler.execute(&command)
    }
}
