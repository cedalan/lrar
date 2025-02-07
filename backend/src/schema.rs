// @generated automatically by Diesel CLI.

diesel::table! {
    burn (id) {
        id -> Int4,
        reason -> Text,
        receiver_id -> Int4,
        giver_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    notes (id) {
        id -> Int4,
        message -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    tenants (id) {
        id -> Int4,
        name -> Varchar,
        age -> Nullable<Int4>,
        image -> Nullable<Varchar>,
        burn_count -> Nullable<Int4>,
        dishwasher_count -> Nullable<Int4>,
        favorite_quote -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    burn,
    notes,
    tenants,
);