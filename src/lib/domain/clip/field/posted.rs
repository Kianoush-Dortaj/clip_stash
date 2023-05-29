use crate::lib::domain::clip::ClipError;
use crate::lib::domain::time::Time;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Constructor, Deserialize, Serialize)]
pub struct Posted(Time);

impl Posted {
    pub fn into_inner(self) -> Time {
        self.0
    }
}
