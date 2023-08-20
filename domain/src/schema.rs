// @generated automatically by Diesel CLI.

diesel::table! {
    authors (author_id) {
        author_id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        bio -> Nullable<Text>,
        profile_picture_url -> Nullable<Text>,
    }
}

diesel::table! {
    categories (category_id) {
        category_id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    comments (comment_id) {
        comment_id -> Int4,
        post_id -> Int4,
        #[max_length = 100]
        author_name -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        content -> Text,
        comment_date -> Timestamp,
    }
}

diesel::table! {
    posts (post_id) {
        post_id -> Int4,
        #[max_length = 200]
        title -> Varchar,
        content -> Text,
        publication_date -> Timestamp,
        author_id -> Int4,
        category_id -> Int4,
    }
}

diesel::table! {
    posttags (post_id, tag_id) {
        post_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    tags (tag_id) {
        tag_id -> Int4,
        #[max_length = 50]
        name -> Nullable<Varchar>,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(posts -> authors (author_id));
diesel::joinable!(posts -> categories (category_id));
diesel::joinable!(posttags -> posts (post_id));
diesel::joinable!(posttags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    categories,
    comments,
    posts,
    posttags,
    tags,
);
