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
        release_date -> Nullable<Date>,
    }
}

diesel::table! {
    weekly_recaps (id) {
        id -> Integer,
        week_number -> Integer,
        year -> Integer,
        recap -> Text,
        generated_at -> Datetime,
    }
}

diesel::allow_tables_to_appear_in_same_query!(games, weekly_recaps,);
