
pub mod field;
use serde::{Serialize,Deserialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClipError {
    #[error("invalid password : {0}")]
    InvalidPassword(String),
    #[error("invalid title : {0}")]
    InvalidTitle(String),
    #[error("empty content")]
    EmptyContent,
    #[error("date parse error : {0}")]
    DateParse(#[from] chrono::ParseError),
    #[error("id parse error : {0}")]
    Id(#[from] uuid::Error),
    #[error("hits parse error : {0}")]
    Hits(#[from] std::num::TryFromIntError)
}



#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Clip {
    pub clip_id : field::ClipId,
    pub shortcode : field::ShortCode,
    pub content : field::Content,
    pub title : field::Title,
    pub posted : field::Posted,
    pub expires : field::Expiers,
    pub password : field::Password,
    pub hits : field::Hits,
}