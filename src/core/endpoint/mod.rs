//! Endpoint common utilities impls

mod fallback_404;
mod rejection;
pub mod validators;

pub use fallback_404::page_not_found;
pub use rejection::{EndpointRejection, EndpointResult};
