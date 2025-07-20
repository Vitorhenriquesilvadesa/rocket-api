use serde::{Deserialize, Serialize};

pub mod user_reqs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pageable {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
