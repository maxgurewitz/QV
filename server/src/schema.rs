use super::sql_enum_types;

table! {
   use diesel::sql_types::*;
   use super::super::sql_enum_types::Progress;

    polls (id) {
        id -> Int4,
        email -> Varchar,
        title -> Varchar,
        poll_type -> Varchar,
        current_progress -> Progress,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    proposals (id) {
        id -> Int4,
        summary -> Text,
        full_description_link -> Nullable<Varchar>,
        poll_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    user_invites (id) {
        id -> Int4,
        email -> Varchar,
        poll_id -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    users (email) {
        email -> Varchar,
        email_verified -> Nullable<Bool>,
        name -> Nullable<Varchar>,
        locale -> Nullable<Varchar>,
        picture -> Nullable<Varchar>,
    }
}

table! {
    votes (id) {
        id -> Int4,
        user_invite_id -> Int4,
        proposal_id -> Int4,
        points -> Numeric,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(proposals -> polls (poll_id));
joinable!(user_invites -> polls (poll_id));
joinable!(votes -> proposals (proposal_id));
joinable!(votes -> user_invites (user_invite_id));

allow_tables_to_appear_in_same_query!(
    polls,
    proposals,
    user_invites,
    users,
    votes,
);
