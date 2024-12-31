use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

use anyhow::anyhow;
use async_trait::async_trait;
use community::user_server::UserToCommunityServer;
use ddnet_account_client_http_fs::cert_downloader::CertsDownloader;
use ddnet_account_client_reqwest::client::ClientReqwestTokioFs;
use ddnet_account_game_server::shared::Shared;
use ddnet_accounts_types::account_id::AccountId;
use game_database::traits::{DbKind, DbKindExtra};
use game_database_backend::GameDbBackend;
use network::network::{
    connection::NetworkConnectionId, event::NetworkEvent,
    event_generator::NetworkEventToGameEventGenerator,
};
use sql::database::{Database, DatabaseDetails};
use tokio::sync::mpsc::Sender;
use x509_cert::der::Encode;

use crate::queries::{add_friend::AddFriend, setup::setup};

#[derive(Debug, Clone)]
pub enum Event {
    Kick(NetworkConnectionId),
}

#[derive(Debug, Clone, Copy)]
struct User {
    account_id: AccountId,
}

pub struct CommunityServer {
    connections: RwLock<HashMap<NetworkConnectionId, User>>,
    cert_downloader: Arc<CertsDownloader>,
    sender: Sender<Event>,

    db: Arc<Database>,
    shared: Arc<Shared>,

    add_friend: AddFriend,
}

impl CommunityServer {
    pub async fn db_setup(details: DatabaseDetails) -> anyhow::Result<Arc<Database>> {
        Ok(Arc::new(
            Database::new(
                [(DbKind::MySql(DbKindExtra::Main), details)]
                    .into_iter()
                    .collect::<HashMap<_, _>>(),
            )
            .await?,
        ))
    }

    pub async fn new(sender: Sender<Event>, details: DatabaseDetails) -> anyhow::Result<Self> {
        let db = Self::db_setup(details).await?;
        let db_backend = Arc::new(GameDbBackend::new(db.clone())?);

        let pool = db
            .pools
            .get(&DbKind::MySql(DbKindExtra::Main))
            .ok_or_else(|| anyhow!("database connection was not intiailized for mysql."))?;
        ddnet_account_game_server::setup::setup(pool).await?;

        setup(db_backend.clone()).await?;
        let add_friend = AddFriend::new(db_backend).await?;

        let shared = ddnet_account_game_server::prepare::prepare(pool).await?;

        let client =
            ClientReqwestTokioFs::new(vec!["https://pg.ddnet.org:5555/".try_into()?], ".".as_ref())
                .await?;
        Ok(Self {
            connections: Default::default(),
            cert_downloader: CertsDownloader::new(client.client).await?,
            sender,

            db,
            shared,

            add_friend,
        })
    }
}

#[async_trait]
impl NetworkEventToGameEventGenerator for CommunityServer {
    async fn generate_from_binary(
        &self,
        _timestamp: Duration,
        con_id: &NetworkConnectionId,
        bytes: &[u8],
    ) {
        let con = {
            let con = self.connections.read().unwrap();
            let Some(con) = con.get(con_id).copied() else {
                return;
            };
            con
        };
        if let Ok(msg) = serde_json::from_slice::<UserToCommunityServer>(bytes) {
            match msg {
                UserToCommunityServer::JoinServer(_) => {
                    // update user status, ping all friends
                    todo!();
                }
                UserToCommunityServer::AddFriend(msg) => {
                    if con.account_id != msg.add_account_id {
                        let min_id = con.account_id.min(msg.add_account_id);
                        let max_id = con.account_id.max(msg.add_account_id);

                        if let Err(err) = self.add_friend.execute(min_id, max_id).await {
                            log::info!("Failed to add friend: {err}");
                        }
                    }
                }
            }
        }
    }

    async fn generate_from_network_event(
        &self,
        _timestamp: Duration,
        con_id: &NetworkConnectionId,
        network_event: &NetworkEvent,
    ) -> bool {
        match network_event {
            NetworkEvent::Connected { cert, .. } => {
                let account_server_public_keys = self.cert_downloader.public_keys();
                let user_id = ddnet_accounts_shared::game_server::user_id::user_id_from_cert(
                    &account_server_public_keys,
                    cert.to_der().unwrap(),
                );

                match user_id.account_id {
                    Some(account_id) => {
                        let _ = ddnet_account_game_server::auto_login::auto_login(
                            self.shared.clone(),
                            self.db
                                .pools
                                .get(&DbKind::MySql(DbKindExtra::Main))
                                .unwrap(),
                            &user_id,
                        )
                        .await;

                        self.connections
                            .write()
                            .unwrap()
                            .insert(*con_id, User { account_id });
                    }
                    None => {
                        // Kick the user
                        self.sender.send(Event::Kick(*con_id)).await.unwrap();
                    }
                }
            }
            NetworkEvent::Disconnected { .. } => {
                self.connections.write().unwrap().remove(con_id);
            }
            NetworkEvent::NetworkStats(_) => {
                // ignore for now
            }
            NetworkEvent::ConnectingFailed(_) => {
                // ignore
            }
        }
        false
    }
}
