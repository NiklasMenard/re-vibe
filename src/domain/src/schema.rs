// @generated automatically by Diesel CLI.

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
        author_id -> Uuid,
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
    roles (role_id) {
        role_id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    tags (tag_id) {
        tag_id -> Int4,
        #[max_length = 50]
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    user_roles (user_id, role_id) {
        user_id -> Uuid,
        role_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 100]
        password -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 100]
        salt -> Varchar,
        bio -> Nullable<Text>,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        profile_picture_url -> Nullable<Text>,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(posts -> categories (category_id));
diesel::joinable!(posts -> users (author_id));
diesel::joinable!(posttags -> posts (post_id));
diesel::joinable!(posttags -> tags (tag_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    comments,
    posts,
    posttags,
    roles,
    tags,
    user_roles,
    users,
);
