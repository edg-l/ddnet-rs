//! Communication between user & community server

use std::net::SocketAddr;

use ddnet_accounts_types::account_id::AccountId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinServer {
    pub addr: SocketAddr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddFriend {
    pub add_account_id: AccountId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserToCommunityServer {
    JoinServer(JoinServer),
    AddFriend(AddFriend),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunityServerToUser {
    ConnectedInfo,
}
