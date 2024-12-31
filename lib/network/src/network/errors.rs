use std::fmt::Display;

use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
pub enum ConnectionErrorCode {
    Kicked = 0x400,
    Banned,
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BanType {
    #[serde(rename = "c")]
    Custom(String),
    #[serde(rename = "v")]
    Vpn,
    /// Banned by the admin
    #[serde(rename = "a")]
    Admin,
}

impl Display for BanType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Vpn => "using a vpn",
                Self::Admin => "by an admin",
                Self::Custom(msg) => &msg,
            }
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Banned {
    #[serde(rename = "m")]
    pub msg: BanType,
    #[serde(rename = "u")]
    pub until: Option<chrono::DateTime<chrono::Utc>>,
}

impl Display for Banned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)?;
        if let Some(until) = self.until {
            write!(f, " until {until}",)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KickType {
    Kick(String),
    Ban(Banned),
}
