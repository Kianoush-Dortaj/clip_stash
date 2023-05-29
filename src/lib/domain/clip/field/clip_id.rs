use crate::lib::data::DbId;
use derive_more::{Constructor, Display, From};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone, Debug, Display, Constructor, Deserialize, Serialize, PartialEq)]
pub struct ClipId(DbId);

impl ClipId {
    pub fn into_inner(self) -> DbId {
        self.0
    }
}

impl Default for ClipId {
    fn default() -> Self {
        Self(DbId::nil())
    }
}

impl From<DbId> for ClipId {
    fn from(id: DbId) -> Self {
        Self(id)
    }
}
