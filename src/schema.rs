table! {
    users (user_id) {
        user_id -> Int4,
        user_uuid -> Uuid,
        hash -> Nullable<Varchar>,
        email -> Varchar,
        created_at -> Timestamp,
        name -> Varchar,
    }
}
