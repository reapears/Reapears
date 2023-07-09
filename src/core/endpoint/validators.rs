//! Common validators utilities  impls

use super::{EndpointRejection, EndpointResult};
use tokio::task::{AbortHandle, JoinSet};

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
