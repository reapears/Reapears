//! Common validators utilities  impls

use super::{EndpointRejection, EndpointResult};
use tokio::task::{AbortHandle, JoinSet};
use uuid::Uuid;

/// Try parse uuid from string
///
/// # Errors
///
/// Return an error if failed to parse `ìd` to Uuid
pub fn parse_uuid(id: &str, err_msg: &'static str, trace_msg: &str) -> EndpointResult<Uuid> {
    Uuid::parse_str(id).map_err(|err| {
        tracing::error!("{trace_msg}, failed to parse Uuid: {}", err);
        EndpointRejection::BadRequest(err_msg.into())
    })
}

/// Forcibly try parse Uuid, panic it fails
///
/// # Panics
///
/// Panics if failed to parse `id`
#[must_use]
pub fn unwrap_uuid(id: &str) -> Uuid {
    Uuid::parse_str(id).unwrap()
}

/// Wait for all validation tasks to complete
///
/// # Errors
///
/// Return an error and abort all running tasks, if one the task failed to validate
pub async fn join_validation_tasks(
    mut tasks: JoinSet<EndpointResult<()>>,
    task_handlers: &[AbortHandle],
) -> EndpointResult<()> {
    while let Some(res) = tasks.join_next().await {
        match res {
            Ok(task_res) => match task_res {
                Ok(()) => {}
                Err(err) => {
                    // Cancel all unfinished tasks
                    for handler in task_handlers {
                        if !handler.is_finished() {
                            handler.abort();
                        }
                    }
                    tracing::error!("Validation error: {}", err);
                    return Err(EndpointRejection::BadRequest(err.to_string().into()));
                }
            },
            Err(err) => {
                tracing::error!("Server fault error: {}", err);
                return Err(EndpointRejection::internal_server_error());
            }
        }
    }
    Ok(())
}
