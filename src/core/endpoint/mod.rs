//! Endpoint common utilities impls

mod fallback_404;
mod model_id;
mod rejection;
mod validated_json;
pub mod validators;

pub use fallback_404::page_not_found;
pub use model_id::ModelId;
pub use rejection::{EndpointRejection, EndpointResult};
pub use validated_json::{ValidateForm, ValidatedJson};
