use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct IdPath {
    pub id: i64,
}
