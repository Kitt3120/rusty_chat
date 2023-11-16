use crate::common::cli::{Command, CommandHandler};
pub struct CommandHandlerBuilder {
    commands: Vec<Command>,
}

impl CommandHandlerBuilder {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn add_command(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }

    pub fn build(self) -> CommandHandler {
        CommandHandler::new(self.commands)
    }
}

impl Default for CommandHandlerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

//TODO: Integration tests