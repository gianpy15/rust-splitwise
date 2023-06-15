// @generated automatically by Diesel CLI.

diesel::table! {
    group_to_user (id) {
        id -> Integer,
        group_id -> Integer,
        user_id -> Integer,
    }
}

diesel::table! {
    split_group (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        username -> Text,
        balance -> Double,
    }
}

diesel::joinable!(group_to_user -> split_group (group_id));
diesel::joinable!(group_to_user -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(group_to_user, split_group, user,);
