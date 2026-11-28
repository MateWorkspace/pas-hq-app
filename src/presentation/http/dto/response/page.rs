use serde::Serialize;

use crate::domain::usecases::Page;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct PageResponse {
    pub limit: u32,
    pub offset: u32,
}

impl From<Page> for PageResponse {
    fn from(page: Page) -> Self {
        Self {
            limit: page.limit,
            offset: page.offset,
        }
    }
}
