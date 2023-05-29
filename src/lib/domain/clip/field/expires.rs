use std::str::FromStr;
use crate::lib::domain::time::Time;
use crate::lib::domain::clip::ClipError;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Expiers(Option<Time>);

impl Expiers {
    pub fn new<T: Into<Option<Time>>>(expiers:T) -> Self {
        Self(expiers.into())
    }

    pub fn into_inner(self) -> Option<Time> {
        self.0
    }
}

impl Default for Expiers {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Expiers {
    type Err = ClipError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.is_empty() {
            Ok(Self(None))
        } else {
            match  Time::from_str(raw) {
                Ok(time) => Ok(Self::new(time)),
                Err(e) => Err(e.into())
            }
        }
    }
}