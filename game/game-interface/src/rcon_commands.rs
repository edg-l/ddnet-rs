use std::collections::HashMap;

use base::network_string::NetworkString;
use command_parser::parser::CommandArg;
use hiarc::Hiarc;
use serde::{Deserialize, Serialize};

/// A single rcon command.
#[derive(Debug, Hiarc, Default, Clone, Serialize, Deserialize)]
pub struct RconCommand {
    pub args: Vec<CommandArg>,
    pub usage: NetworkString<65536>,
    pub description: NetworkString<65536>,
}

/// Commands supported by the server.
#[derive(Debug, Hiarc, Default, Clone, Serialize, Deserialize)]
pub struct RconCommands {
    /// list of commands and their required args
    pub cmds: HashMap<NetworkString<65536>, RconCommand>,
}

#[derive(Debug, Hiarc, Default, Clone, Copy, Serialize, Deserialize)]
pub enum AuthLevel {
    #[default]
    None,
    Moderator,
    Admin,
}

/// A remote console command that a mod might support.
/// Note that some rcon commands are already processed
/// by the server implementation directly, like
/// changing a map.
#[derive(Debug, Hiarc, Clone, Serialize, Deserialize)]
pub struct ExecRconCommand {
    /// the raw unprocessed command string.
    pub raw: NetworkString<{ 65536 * 2 + 1 }>,
    /// The auth level the client has for this command.
    pub auth_level: AuthLevel,
}
