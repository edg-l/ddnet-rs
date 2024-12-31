pub mod user_server;

use std::{collections::HashMap, net::SocketAddr};

use base::hash::Hash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub cert_hash: Hash,
    pub cur_load: u64,
    pub max_load: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub servers: HashMap<SocketAddr, ServerInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Register {
    pub password: String,
    pub info: ServerInfo,
    pub port: u16,
}
