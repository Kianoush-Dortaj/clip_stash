use super::ClipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Title(Option<String>);

impl Title {
    fn new<T: Into<Option<String>>>(title: T) -> Self{
        let title: Option<String> = title.into();
        match title {
            Some(title) => {
                if !title.trim().is_empty() {
                    Ok(Self(Some(title)))
                } else {
                    Ok(Self(None))
                }
            }
            None => Ok(Self(None)),
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
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
