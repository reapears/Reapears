//! Farm rating model impls
#![allow(
    dead_code,
    clippy::missing_const_for_fn,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::core::{accounts::user::models::UserIndex, types::ModelIdentifier};

/// A `Vec` of farm ratings
pub type FarmRatingList = Vec<FarmRating>;

/// The model representing a row in the `locations` database table.
///
/// Returned by `farm_rating_detail/list` handler.
#[derive(Debug, Clone, Serialize)]
pub struct FarmRating {
    pub id: Uuid,
    pub grade: u8,
    pub comment: Option<String>,
    pub farm: ModelIdentifier,
    pub author: UserIndex,
    /// The date on which the rating was last updated at and
    /// if is not set is the rating creation date
    pub update_at: OffsetDateTime,
}

impl FarmRating {
    /// Creates a new `FarmRating` from the database row
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn from_row(
        id: Uuid,
        grade: i32,
        comment: Option<String>,
        update_at: OffsetDateTime,
        farm_id: Uuid,
        farm_name: String,
        user_id: Uuid,
        user_first_name: String,
        user_last_name: Option<String>,
        user_photo: Option<String>,
    ) -> Self {
        Self {
            id,
            grade: grade as u8,
            comment,
            farm: ModelIdentifier::from_row(farm_id, farm_name),
            author: UserIndex::from_row(user_id, user_first_name, user_last_name, user_photo),
            update_at,
        }
    }
}

// /// Filter for `FarmRatings`
// #[derive(Clone, Debug)]
// pub enum FarmRatingFilter {
//     AuthorIdEq(Vec<Uuid>),
//     AuthorIdNe(Vec<Uuid>),
//     FarmIdEq(Vec<Uuid>),
//     FarmIdNe(Vec<Uuid>),
//     GradeLt(i32),
//     GradeGt(i32),
//     GradeEq(Vec<i32>),
//     GradeNe(Vec<i32>),
//     CreatedAtLt(Date),
//     CreatedAtGt(Date),
//     CreatedAtEq(Vec<Date>),
//     CreatedAtNe(Vec<Date>),
// }
