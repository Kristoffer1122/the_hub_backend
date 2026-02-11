// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        genre -> Varchar,
        utgivelsesdato -> Nullable<Date>,
    }
}
