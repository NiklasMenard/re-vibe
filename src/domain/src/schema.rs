// @generated automatically by Diesel CLI.

diesel::table! {
    product_categories (category_id) {
        category_id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    product_tags (product_id, tag_id) {
        product_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    products (product_id) {
        product_id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 255]
        description -> Varchar,
        price -> Numeric,
        quantity -> Int4,
        seller_id -> Uuid,
        category_id -> Int4,
        creation_date -> Timestamp,
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

diesel::joinable!(product_tags -> products (product_id));
diesel::joinable!(product_tags -> tags (tag_id));
diesel::joinable!(products -> product_categories (category_id));
diesel::joinable!(products -> users (seller_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    product_categories,
    product_tags,
    products,
    roles,
    tags,
    user_roles,
    users,
);
