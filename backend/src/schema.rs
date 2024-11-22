// @generated automatically by Diesel CLI.

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
