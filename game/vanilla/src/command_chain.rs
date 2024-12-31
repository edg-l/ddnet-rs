use std::collections::HashMap;

use base::network_string::NetworkString;
use command_parser::parser::CommandArg;
use game_interface::rcon_commands::RconCommand;

#[derive(Debug)]
pub struct Command<T> {
    pub rcon: RconCommand,
    pub cmd: T,
}

#[derive(Debug)]
pub struct CommandChain<T> {
    pub cmds: HashMap<NetworkString<65536>, Command<T>>,
    pub parser: HashMap<NetworkString<65536>, Vec<CommandArg>>,
}

impl<T> CommandChain<T> {
    pub fn new(cmds: HashMap<NetworkString<65536>, Command<T>>) -> Self {
        let parser = cmds
            .iter()
            .map(|(name, cmd)| (name.clone(), cmd.rcon.args.clone()))
            .collect();
        Self { cmds, parser }
    }
}
