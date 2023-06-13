// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Integer,
        username -> Text,
        balance -> Double,
    }
}
