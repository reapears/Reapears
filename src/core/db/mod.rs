//! Database Interface

// use axum::async_trait;
use uuid::Uuid;

// use crate::{
//     error::ServerResult,
//     {server::state::DatabaseConnection, types::Pagination},
// };

// /// Model's find for admins
// ///
// /// It may include fields that are not available to end users
// #[async_trait]
// pub trait AdminDatabase {
//     type Record;
//     /// Find and return `Self::Record` matching the `id` from model's database table.
//     async fn find(
//         id: Uuid,
//         pg: Option<Pagination>,
//         db: DatabaseConnection,
//     ) -> ServerResult<Option<Self::Record>>;
// }

// /// Filter records from the database
// #[async_trait]
// pub trait RecordFilter: Database {
//     type Filters;
//     /// Find records matching the filter from the models's database table.
//     async fn filter(
//         filter: Self::Filters,
//         pg: Pagination,
//         db: DatabaseConnection,
//     ) -> ServerResult<<Self as Database>::RecordList>;
// }

// /// Return model identifiers
// ///
// /// An identifier is usually a name and an id of a model
// #[async_trait]
// pub trait ModelIndex{
//     type Index;
//     async fn index(id: Uuid, pg: Pagination, db: DatabaseConnection) -> ServerResult<Index>
// }

/// Generates a new model id(Uuid)
#[must_use]
pub fn model_id() -> Uuid {
    Uuid::new_v4()
}
