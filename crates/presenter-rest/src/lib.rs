pub mod cat_facts;
pub mod dog_facts;
mod shared;

pub use shared::{app_state::RestAppState, routes::RestControllers};
