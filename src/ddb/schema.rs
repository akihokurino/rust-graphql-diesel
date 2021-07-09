table! {
    photos (id) {
        id -> Varchar,
        user_id -> Varchar,
        url -> Varchar,
        is_public -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

joinable!(photos -> users (user_id));

allow_tables_to_appear_in_same_query!(
    photos,
    users,
);
