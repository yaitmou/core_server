use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    #[serde(default)]
    pub records: Vec<T>,
    #[serde(default)]
    pub has_next: bool,
    #[serde(default)]
    pub current_page: i32,
    #[serde(default)]
    pub total: u64,
}

impl<T> PaginatedResponse<T> {
    pub fn with_records<S>(&self, records: Vec<S>) -> PaginatedResponse<S> {
        PaginatedResponse {
            records: records,
            has_next: self.has_next,
            current_page: self.current_page,
            total: self.total,
        }
    }
}

// Define a struct to capture common query parameters
#[derive(Debug, Deserialize)]
pub struct PaginatedParams {
    // Pagination parameters
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,

    // All other filter parameters
    #[serde(flatten, default)]
    pub query: HashMap<String, String>,
}
impl PaginatedParams {
    pub fn new() -> Self {
        Self {
            page: 0,
            limit: 10,
            query: HashMap::<String, String>::new(),
        }
    }

    pub fn with_filter(filter: &HashMap<String, String>) -> Self {
        let mut s = Self::new();
        s.query = filter.clone();
        s
    }

    pub fn all_with_filter(filter: HashMap<String, String>) -> Self {
        Self {
            page: 0,
            limit: 0,
            query: filter,
        }
    }
}

// Default pagination values
fn default_page() -> i32 {
    0
}
fn default_limit() -> i32 {
    10
}
