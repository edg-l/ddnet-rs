use std::{fmt::Debug, net::SocketAddr, sync::Arc};

use async_trait::async_trait;

use super::{connection::NetworkConnectionId, errors::Banned};

/// Plugin system interface for packets:
/// - modify a raw buffer before being sent
/// - modify a raw buffer before being read
/// Respects the order in which plugins are passed, the first plugin will always modify a write
/// buffer as first, and modify a read packet as last
#[async_trait]
pub trait NetworkPluginPacket: Debug + Sync + Send + 'static {
    async fn prepare_write(
        &self,
        id: &NetworkConnectionId,
        buffer: &mut Vec<u8>,
    ) -> anyhow::Result<()>;
    async fn prepare_read(
        &self,
        id: &NetworkConnectionId,
        buffer: &mut Vec<u8>,
    ) -> anyhow::Result<()>;
}

pub enum ConnectionEvent {
    /// The connection is allowed to connect
    Allow,
    Banned(Banned),
    /// E.g. if there is a connection limit
    Kicked(String),
}

/// Plugin system interface for connection related events:
/// - can listen for on_incoming events (e.g. to drop connections by IP, or modify the socket addr to emulate a proxy)
/// - can listen for on_connect events (e.g. to obtain important information about the connection)
/// Respects the order in which plugins are passed, the first plugin will always listen for on_connect events first
/// and on_disconnect events last
#[async_trait]
pub trait NetworkPluginConnection: Debug + Sync + Send + 'static {
    /// Returns `Ok(true)` if the connection should be allowed.
    ///
    /// Else it will be rejected silently.
    #[must_use]
    async fn on_incoming(&self, remote_addr: &SocketAddr) -> bool;
    #[must_use]
    async fn on_connect(
        &self,
        id: &NetworkConnectionId,
        remote_addr: &SocketAddr,
        cert: &x509_cert::Certificate,
    ) -> ConnectionEvent;
    async fn on_disconnect(
        &self,
        id: &NetworkConnectionId,
        remote_addr: &SocketAddr,
        cert: &x509_cert::Certificate,
    );
}

/// All plugins supported by the network implementation.
#[derive(Debug, Clone, Default)]
pub struct NetworkPlugins {
    pub packet_plugins: Arc<Vec<Arc<dyn NetworkPluginPacket>>>,
    pub connection_plugins: Arc<Vec<Arc<dyn NetworkPluginConnection>>>,
}
