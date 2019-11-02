table! {
    api_keys (id) {
        id -> Integer,
        public_id -> Varchar,
        permission -> Varchar,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        password -> Varchar,
        scheme -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    api_keys,
    users,
);
