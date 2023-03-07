//! User profile database impl

use uuid::Uuid;

use crate::{error::ServerResult, server::state::DatabaseConnection};

use super::{forms::PersonalInfoUpdateData, models::PersonalInfo};

impl PersonalInfo {
    /// Find and return user `PersonalInfo` matching the `id` from the database.
    #[tracing::instrument(skip(db), name = "Find PersonalInfo")]
    pub async fn find(id: Uuid, db: DatabaseConnection) -> ServerResult<Option<Self>> {
        match sqlx::query!(
            r#"
                SELECT user_.id AS user_id,
                    user_.first_name AS user_first_name, 
                    user_.last_name AS user_last_name, 
                    user_.gender AS user_gender,
                    user_.date_of_birth AS user_date_of_birth, 
                    user_.date_joined AS user_date_joined,
                    address.email AS user_email, 
                    phone.phone AS "user_phone?"
                    -- government_id.national_id AS "user_government_id?"
                FROM accounts.users user_
                LEFT JOIN accounts.emails address
                    ON user_.id = address.user_id
                LEFT JOIN accounts.phones phone
                    ON user_.id = phone.user_id
                -- LEFT JOIN accounts.government_ids government_id
                --    ON user_.id = government_id.user_id

                WHERE user_.id = $1;
            "#,
            id
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(rec) => {
                let personal_info = Self::from_row(
                    rec.user_id,
                    rec.user_first_name,
                    rec.user_last_name,
                    rec.user_gender,
                    rec.user_date_of_birth,
                    // rec.user_government_id,
                    rec.user_email,
                    rec.user_phone,
                    rec.user_date_joined.date(),
                );

                Ok(Some(personal_info))
            }
            Err(err) => {
                if matches!(err, sqlx::Error::RowNotFound) {
                    Ok(None)
                } else {
                    tracing::error!(
                        "Database error, User personal info could not be fetched: {}",
                        err
                    );
                    Err(err.into())
                }
            }
        }
    }

    /// Update user `PersonalInfo` matching the `id` in the database.
    ///
    /// Caller must validate the `id` exists
    #[tracing::instrument(skip(db, values), name = "Update PersonalInfo")]
    pub async fn update(
        id: Uuid,
        values: PersonalInfoUpdateData,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE accounts.users user_
                SET first_name = COALESCE($1, user_.first_name),
                    last_name = $2,
                    gender = $3,
                    date_of_birth = $4
                WHERE user_.id = $5
            "#,
            values.first_name,
            values.last_name,
            values.gender,
            values.date_of_birth,
            id
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("User personal infos updated successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                tracing::error!(
                    "Database error, User personal infos could not be updated: {}",
                    err
                );
                Err(err.into())
            }
        }
    }
}
