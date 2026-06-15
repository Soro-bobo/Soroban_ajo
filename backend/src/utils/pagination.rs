use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const DEFAULT_PAGE_SIZE: i64 = 20;
pub const MAX_PAGE_SIZE: i64 = 100;

#[derive(Debug, Serialize)]
pub struct PageMeta {
    pub total: i64,
    pub limit: i64,
    pub next_cursor: Option<Uuid>,
    pub has_more: bool,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub meta: PageMeta,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(mut items: Vec<T>, limit: i64, total: i64, last_id: Option<Uuid>) -> Self {
        let has_more = items.len() as i64 > limit;
        if has_more {
            items.truncate(limit as usize);
        }

        let next_cursor = if has_more { last_id } else { None };

        Self {
            data: items,
            meta: PageMeta {
                total,
                limit,
                next_cursor,
                has_more,
            },
        }
    }
}

pub fn clamp_limit(requested: Option<i64>) -> i64 {
    requested
        .unwrap_or(DEFAULT_PAGE_SIZE)
        .clamp(1, MAX_PAGE_SIZE)
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CursorParams {
    pub cursor: Option<Uuid>,
    pub limit: Option<i64>,
}
