use serde::Deserialize;

use crate::domain::usecases::Page;

/// Paging parameters accepted by every list endpoint.
///
/// List queries declare these fields directly rather than flattening this struct:
/// the query extractor uses `serde_urlencoded`, which cannot deserialize
/// `flatten`. Keeping the bounds here means every endpoint clamps identically.
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct PageQuery {
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

impl PageQuery {
    /// Clamp the caller's request into a bounded page.
    pub(crate) fn into_page(self) -> Page {
        page_from(self.limit, self.offset)
    }
}

/// Build a bounded page from raw query values.
pub(crate) fn page_from(limit: u32, offset: u32) -> Page {
    Page {
        limit: if limit == 0 {
            default_limit()
        } else {
            limit.min(max_limit())
        },
        offset,
    }
}

pub(crate) const fn default_limit() -> u32 {
    10
}

pub(crate) const fn max_limit() -> u32 {
    100
}
