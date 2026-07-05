// @generated automatically by Diesel CLI.

diesel::table! {
    can_message (id) {
        id -> Integer,
        profile_id -> Integer,
        can_id -> Integer,
        is_extended -> Integer,
        data -> Binary,
        mode -> Text,
        offset_ms -> Integer,
        period_ms -> Nullable<Integer>,
    }
}

diesel::table! {
    test_profile (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(can_message -> test_profile (profile_id));

diesel::allow_tables_to_appear_in_same_query!(can_message, test_profile,);
