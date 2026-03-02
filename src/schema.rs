// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        genre -> Varchar,
        #[max_length = 1024]
        image_link -> Nullable<Varchar>,
        utgivelsesdato -> Nullable<Date>,
    }
}
