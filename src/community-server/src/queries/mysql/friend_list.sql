CREATE TABLE friend_list (
    account_id1 BIGINT NOT NULL,
    account_id2 BIGINT NOT NULL,
    UNIQUE KEY((account_id1, account_id2))
);
