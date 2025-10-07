// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Uuid,
        post_id -> Uuid,
        author_id -> Uuid,
        parent_comment_id -> Nullable<Uuid>,
        content -> Text,
        is_approved -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    open_id_credentials (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 50]
        provider -> Varchar,
        #[max_length = 255]
        provider_user_id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    post_links (id) {
        id -> Uuid,
        source_post_id -> Uuid,
        target_post_id -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    post_tags (post_id, tag_id) {
        post_id -> Uuid,
        tag_id -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        author_id -> Uuid,
        #[max_length = 500]
        title -> Varchar,
        #[max_length = 500]
        slug -> Varchar,
        content -> Text,
        published -> Bool,
        #[max_length = 20]
        visibility -> Varchar,
        published_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tags (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        slug -> Varchar,
        description -> Nullable<Text>,
        parent_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_group_members (id) {
        id -> Uuid,
        group_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_groups (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_roles (user_id, role_id) {
        user_id -> Uuid,
        role_id -> Uuid,
        assigned_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        display_name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (author_id));
diesel::joinable!(open_id_credentials -> users (user_id));
diesel::joinable!(post_tags -> posts (post_id));
diesel::joinable!(post_tags -> tags (tag_id));
diesel::joinable!(posts -> users (author_id));
diesel::joinable!(user_group_members -> user_groups (group_id));
diesel::joinable!(user_group_members -> users (user_id));
diesel::joinable!(user_groups -> users (user_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    open_id_credentials,
    post_links,
    post_tags,
    posts,
    roles,
    tags,
    user_group_members,
    user_groups,
    user_roles,
    users,
);
