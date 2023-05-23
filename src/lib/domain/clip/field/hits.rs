use super::ClipError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Hits(u64);

impl Hits {

    pub fn new(data:u64) -> Self {
        Self(data)
    }

    pub fn into_inner(self) -> u64{
        self.0
    }
}