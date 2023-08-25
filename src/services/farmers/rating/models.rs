//! Farm rating model impls

use serde::Serialize;
use time::OffsetDateTime;

use crate::core::{
    accounts::user::models::UserIndex,
    types::{ModelID, ModelIdentifier},
};

/// A `Vec` of farm ratings
pub type FarmRatingList = Vec<FarmRating>;

/// The model representing a row in the `farm_ratings` database table.
#[derive(Debug, Clone, Serialize)]
pub struct FarmRating {
    pub id: ModelID,
    pub grade: u8,
    pub comment: Option<String>,
    pub farm: ModelIdentifier,
    pub author: UserIndex,
    /// The date on which the rating was last updated at and
    /// if is not set is the rating creation date
    pub update_at: OffsetDateTime,
}

// // `FarmRating` reply
// #[derive(Debug, Clone, Serialize)]
// pub struct RatingReply {
//     pub id: ModelID,
//     pub comment: String,
//     pub author: UserIndex,
//     pub reply_to: ModelID,
//     pub created_at: OffsetDateTime,
// }
// WITH RECURSIVE ctename AS (
//     SELECT empno, ename
//     FROM emp
//     WHERE empno = 7566
//  UNION ALL
//     SELECT emp.empno, emp.ename
//     FROM emp
//        JOIN ctename ON emp.mgr = ctename.empno
// )
// SELECT * FROM ctename;

impl FarmRating {
    /// Creates a new `FarmRating` from the database row
    #[allow(
        clippy::too_many_arguments,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    #[must_use]
    pub fn from_row(
        id: ModelID,
        grade: i32,
        comment: Option<String>,
        update_at: OffsetDateTime,
        farm_id: ModelID,
        farm_name: String,
        user_id: ModelID,
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
