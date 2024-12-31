use std::sync::Arc;

use game_database::{
    statement::{Statement, StatementBuilder},
    traits::{DbInterface, DbKind, DbKindExtra},
};

#[derive(Clone)]
pub struct SetupFriendList {
    stmts: Vec<Arc<Statement<(), ()>>>,
}

impl SetupFriendList {
    pub async fn new(db: Arc<dyn DbInterface>) -> anyhow::Result<Self> {
        let mut stmts = Vec::new();

        let builder = StatementBuilder::<_, (), ()>::new(
            DbKind::MySql(DbKindExtra::Main),
            include_str!("mysql/friend_list.sql"),
            |_| vec![],
        );
        let stmt = Arc::new(Statement::new(db.clone(), builder).await?);
        stmts.push(stmt.clone());

        Ok(Self { stmts })
    }
}

pub async fn setup(db: Arc<dyn DbInterface>) -> anyhow::Result<()> {
    let setup_friend_list = SetupFriendList::new(db.clone()).await?;

    db.setup(
        "friend-list",
        vec![(
            1,
            setup_friend_list
                .stmts
                .iter()
                .map(|s| s.unique_id)
                .collect(),
        )]
        .into_iter()
        .collect(),
    )
    .await
}
