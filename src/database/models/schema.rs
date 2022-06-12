table! {
    password_reset_token (id) {
        id -> Bigint,
        expiryDate -> Nullable<Datetime>,
        token -> Nullable<Varchar>,
        user_id -> Unsigned<Integer>,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Unsigned<Integer>,
        date_created -> Nullable<Timestamp>,
        date_updated -> Nullable<Timestamp>,
        uid -> Char,
        display_name -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        email -> Varchar,
        email_verified -> Bool,
        telephone -> Nullable<Varchar>,
        last_login -> Nullable<Timestamp>,
        fcm -> Nullable<Varchar>,
        is_banned -> Bool,
        ip_address -> Nullable<Varchar>,
        meta_data -> Nullable<Varchar>,
        birthday -> Nullable<Timestamp>,
        password -> Varchar,
        role -> Integer,
        telephone_verified -> Bool,
        user_rating -> Decimal,
    }
}

table! {
    verification_token (id) {
        id -> Bigint,
        expiryDate -> Nullable<Datetime>,
        token -> Nullable<Varchar>,
        user_id -> Unsigned<Integer>,
    }
}

joinable!(password_reset_token -> users (user_id));
joinable!(verification_token -> users (user_id));

allow_tables_to_appear_in_same_query!(
    password_reset_token,
    posts,
    users,
    verification_token,
);
