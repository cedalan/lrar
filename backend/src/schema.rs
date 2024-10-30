// @generated automatically by Diesel CLI.

diesel::table! {
    tenants (id) {
        id -> Integer,
        name -> Text,
        height -> Integer,
        profile_picture_uri -> Text,
        burns -> Integer,
    }
}
