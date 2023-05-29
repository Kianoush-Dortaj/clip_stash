use crate::lib::domain::clip::ClipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Title(Option<String>);
impl Title {
    fn new<T: Into<Option<String>>>(title: T) -> Self {
        let title: Option<String> = title.into();
        if let Some(title) = title {
            if !title.trim().is_empty() {
                Self(Some(title))
            } else {
                Self(None)
            }
        } else {
            Self(None)
        }
    }
}


impl Default for Title {
    fn default() -> Self {
        Self(None)
    }
}

impl FromStr for Title {
    type Err = ClipError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       Ok(Self::new(s.to_string()))
    }
}
