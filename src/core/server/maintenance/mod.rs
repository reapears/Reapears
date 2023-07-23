//! Server maintenance routines impls

mod tasks;

pub use tasks::server_maintenance;

/*

* Delete users
- users are added to account_deletion_requested
-
* Archive harvests that have been up for more than 90 days
-

* Delete Files
- delete files that have been saved but the request did not complete successfully


*/
