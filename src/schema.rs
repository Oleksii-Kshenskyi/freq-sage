// @generated automatically by Diesel CLI.

diesel::table! {
    frequencies (id) {
        id -> Nullable<Integer>,
        word -> Text,
        frequency -> BigInt,
        lang -> Text,
    }
}

diesel::table! {
    sentence_rankings (id) {
        id -> Nullable<Integer>,
        sentence -> Text,
        ranking -> BigInt,
        lang -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(frequencies, sentence_rankings,);
