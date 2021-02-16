table! {
    club (id) {
        id -> Bigint,
        name -> Varchar,
        is_published -> Bool,
        short_description -> Nullable<Text>,
        long_description -> Nullable<Mediumtext>,
        join_description -> Nullable<Text>,
        place -> Nullable<Text>,
        schedule -> Nullable<Text>,
        video_url -> Nullable<Text>,
        contact_url -> Nullable<Text>,
    }
}
