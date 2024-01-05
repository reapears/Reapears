//! Server maintenance impls

use std::time::Duration;

use time::{OffsetDateTime, Time};

use crate::{accounts::AccountDelete, server::state::ServerState};

/// Server maintenance tasks runner
pub async fn server_maintenance(state: ServerState) {
    // Set up starting time
    // * Server maintenance will be kicked off at 4am every day
    const DAY: u64 = 60 * 60 * 24;
    let now = OffsetDateTime::now_utc().time();
    let run_at = Time::from_hms(4, 0, 0).unwrap();
    let start_at = tokio::time::Instant::now() + (now - run_at).unsigned_abs();

    let mut interval = tokio::time::interval_at(start_at, Duration::from_secs(DAY));
    // Runner
    loop {
        interval.tick().await;

        let db = state.database();
        // Delete user accounts the requested for account deletion
        AccountDelete::permanently_delete_accounts(db).await;
    }
}
