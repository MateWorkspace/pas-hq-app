use std::sync::Mutex;

use pas_hq_app::domain::{
    contracts::utility::Logger,
    models::{
        AppContext,
        logger::{Level, Meta},
    },
};
use uuid::Uuid;

mod auth;
mod permission;
mod role;
mod user;

#[derive(Clone, Debug)]
pub(crate) struct Record {
    pub(crate) level: Level,
    pub(crate) tag: String,
    pub(crate) message: String,
    pub(crate) meta: Option<Meta>,
}

#[derive(Default)]
pub(crate) struct CapturingLogger {
    records: Mutex<Vec<Record>>,
}

impl CapturingLogger {
    pub(crate) fn records(&self) -> Vec<Record> {
        self.records.lock().unwrap().clone()
    }
}

impl Logger for CapturingLogger {
    fn log(
        &self,
        _context: &AppContext,
        level: Level,
        tag: &str,
        message: &str,
        meta: Option<&Meta>,
    ) {
        self.records.lock().unwrap().push(Record {
            level,
            tag: tag.to_owned(),
            message: message.to_owned(),
            meta: meta.cloned(),
        });
    }
}

pub(crate) fn context() -> AppContext {
    AppContext {
        actor: Some("anonymous".into()),
        trace_id: Some(Uuid::from_u128(2)),
        execution_id: Some(Uuid::from_u128(1)),
        claims: None,
    }
}
