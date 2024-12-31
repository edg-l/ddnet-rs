use std::{net::SocketAddr, sync::Arc};

use thiserror::Error;

use super::{connection::ConnectionStats, errors::Banned};

pub type NetworkStats = ConnectionStats;

#[derive(Error, Debug, Clone)]
pub enum NetworkEventConnectingClosed {
    #[error("kicked for {0}")]
    Kicked(String),
    #[error("banned for \"{0}\"")]
    Banned(Banned),
    #[error("shutdown was requested {0}")]
    Shutdown(String),
    #[error("application restarted")]
    Reset,
    #[error("{0}")]
    Other(String),
}

#[derive(Error, Debug, Clone)]
pub enum NetworkEventConnectingFailed {
    /// The given server name was malformed
    #[error("invalid server name: {0}")]
    InvalidServerName(String),
    /// The remote [`SocketAddr`] supplied was malformed
    ///
    /// Examples include attempting to connect to port 0, or using an inappropriate address family.
    #[error("invalid remote address: {0}")]
    InvalidRemoteAddress(SocketAddr),

    /// The peer closed or aborted the connection automatically
    #[error("closed by peer: {0}")]
    ConnectionClosed(NetworkEventConnectingClosed),
    /// Communication with the peer has lapsed for longer than the negotiated idle timeout
    ///
    /// If neither side is sending keep-alives, a connection will time out after a long enough idle
    /// period even if the peer is still reachable.
    #[error("timed out")]
    TimedOut,
    /// The local application closed the connection
    #[error("closed")]
    LocallyClosed,

    #[error("{0}")]
    Other(String),
}

#[derive(Error, Debug, Clone)]
pub enum NetworkEventDisconnect {
    /// The given server name was malformed
    #[error("invalid server name: {0}")]
    InvalidServerName(String),
    /// The remote [`SocketAddr`] supplied was malformed
    ///
    /// Examples include attempting to connect to port 0, or using an inappropriate address family.
    #[error("invalid remote address: {0}")]
    InvalidRemoteAddress(SocketAddr),

    /// The peer closed or aborted the connection automatically
    #[error("closed by peer: {0}")]
    ConnectionClosed(NetworkEventConnectingClosed),
    /// Communication with the peer has lapsed for longer than the negotiated idle timeout
    ///
    /// If neither side is sending keep-alives, a connection will time out after a long enough idle
    /// period even if the peer is still reachable.
    #[error("timed out")]
    TimedOut,
    /// The local application closed the connection
    #[error("closed")]
    LocallyClosed,

    #[error("{0}")]
    Other(String),

    #[error("gracefully disconnected")]
    Graceful,
}

#[derive(Debug, Clone)]
pub enum NetworkEvent {
    /// [`SocketAddr`] represents:
    /// - client: the ip of the server
    /// - server: the ip of the client
    Connected {
        addr: SocketAddr,
        cert: Arc<x509_cert::Certificate>,
        initial_network_stats: NetworkStats,
    },
    Disconnected(NetworkEventDisconnect),
    ConnectingFailed(NetworkEventConnectingFailed),
    NetworkStats(NetworkStats),
}
