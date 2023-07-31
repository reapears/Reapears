//! Accounts routers impls

use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get, post},
    Router,
};

use crate::{
    accounts::{
        emails::handlers::{
            email_change_approve, email_exists, email_update, new_email_change_verify,
        },
        passwords::handlers::{password_change, password_forgot, password_reset, password_verify},
        personal_info::handlers::{user_personal_info, user_personal_info_update},
        user::handlers::{
            account_confirm, account_deactivate, account_lock, account_unlock, signup, user_list,
            user_make_staff, user_make_superuser, user_revoke_staff, user_revoke_superuser,
        },
        user_profile::handlers::{
            user_my_profile, user_photo_upload, user_profile, user_profile_update,
        },
    },
    auth::api_key::handlers::{
        api_key_delete, api_key_list, generate_api_key_for_app, generate_api_key_for_user,
    },
    auth::sessions::handlers::{login, logout},
    features::{
        direct_message::handlers::{direct_message_websocket, user_conversations},
        harvest_subscription::handlers::user_harvest_subscriptions,
    },
    server::state::ServerState,
};

/// Accounts routers
pub fn routers() -> Router<ServerState> {
    Router::new()
        // Accounts
        .route("/account/signup", post(signup))
        .route("/account/deactivate", delete(account_deactivate))
        .route("/account/login", post(login))
        .route("/account/logout", delete(logout))
        .route("/account/lock", post(account_lock))
        .route("/account/unlock", post(account_unlock))
        .route("/account/email-exists", post(email_exists))
        .route("/account/confirm", get(account_confirm))
        .route("/account/forgot-password", post(password_forgot))
        .route("/account/reset-password", post(password_reset))
        // Users
        .route("/account/users", get(user_list))
        .route("/account/users/:user_id/profile", get(user_profile))
        .route(
            "/account/users/profile",
            get(user_my_profile).put(user_profile_update),
        )
        .route(
            "/account/users/profile/photo",
            post(user_photo_upload).layer(DefaultBodyLimit::max(crate::IMAGE_MAX_SIZE)),
        )
        // DirectMessage
        .route("/account/users/chat", get(direct_message_websocket))
        .route(
            "/account/users/chat/direct_message",
            get(user_conversations),
        )
        // Settings
        .route(
            "/account/settings/personal-info",
            get(user_personal_info).put(user_personal_info_update),
        )
        .route("/account/settings/change-email", post(email_update))
        .route(
            "/account/settings/approve-email-change",
            post(email_change_approve),
        )
        .route(
            "/account/settings/confirm-new-email",
            post(new_email_change_verify),
        )
        .route("/account/settings/change-password", post(password_change))
        .route("/account/settings/verify-password", post(password_verify))
        // .route("/account/settings/phones", put(phone_update))
        .route("/account/settings/add-superuser", post(user_make_superuser))
        .route(
            "/account/settings/revoke-superuser",
            post(user_revoke_superuser),
        )
        .route("/account/settings/add-staff", post(user_make_staff))
        .route("/account/settings/revoke-staff", post(user_revoke_staff))
        // Subscriptions
        .route(
            "/account/harvests-subscriptions",
            get(user_harvest_subscriptions),
        )
        // ApiKey Auth
        .route("/account/auth/api-key", get(api_key_list))
        .route("/account/auth/api-key/:token_id", delete(api_key_delete))
        .route(
            "/account/auth/api-key/for_user",
            get(generate_api_key_for_user),
        )
        .route(
            "/account/auth/api-key/for_app",
            get(generate_api_key_for_app),
        )
}
