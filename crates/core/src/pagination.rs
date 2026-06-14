//! Request/response pagination primitives.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct ResponsePagination {
    pub size: i64,
    pub page: i64,
}

impl Default for ResponsePagination {
    fn default() -> Self {
        Self { size: 10, page: 1 }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestPagination {
    pub total_count: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List<T> {
    pub pagination: RequestPagination,
    pub items: Vec<T>,
}
