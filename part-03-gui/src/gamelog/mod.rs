use crate::rltk::RGB;

mod logstore;
use logstore::*;
pub use logstore::{clear_log, clone_log, print_log, restore_log};
use serde::{Deserialize, Serialize};
mod builder;
pub use builder::*;
mod events;
pub use events::{clear_events, clone_events, get_event_count, load_events, record_event};

#[derive(Serialize, Deserialize, Clone)]
pub struct LogFragment {
    pub color: RGB,
    pub text: String,
}
