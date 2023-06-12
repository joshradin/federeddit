// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Bigint,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 64]
        username -> Varchar,
        password_hash -> Text,
    }
}
