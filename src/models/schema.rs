table! {
    asset (id) {
        id -> Bigint,
        owner_id -> Bigint,
        club_id -> Varchar,
        name -> Text,
        alternative_description -> Nullable<Text>,
        file_path -> Text,
        file_size -> Integer,
        image_width -> Nullable<Integer>,
        image_height -> Nullable<Integer>,
    }
}

table! {
    club (id) {
        id -> Varchar,
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

table! {
    user (id) {
        id -> Bigint,
        name -> Text,
        screen_name -> Text,
        icon -> Nullable<Varchar>,
        permission -> Integer,
        access_token -> Nullable<Varchar>,
        access_token_secret -> Nullable<Varchar>,
    }
}

table! {
    user_club_relation (user_id, club_id) {
        user_id -> Bigint,
        club_id -> Varchar,
        level -> Integer,
    }
}

joinable!(asset -> club (club_id));
joinable!(asset -> user (owner_id));
joinable!(user_club_relation -> club (club_id));
joinable!(user_club_relation -> user (user_id));

allow_tables_to_appear_in_same_query!(
    asset,
    club,
    user,
    user_club_relation,
);
