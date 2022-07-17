pub mod database;
pub mod record;
pub mod server;
pub mod watcher;

pub use database::read_records;
pub use record::{Record, RecordSet};
pub use server::{create_router, ServerState, SharedServerState};
pub use watcher::watch;
