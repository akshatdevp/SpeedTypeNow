// @generated automatically by Diesel CLI.

diesel::table! {
    long_text (id) {
        id -> Int4,
        difficulty -> Varchar,
        body -> Text,
        source -> Varchar,
    }
}
