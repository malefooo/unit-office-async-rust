
use std::fmt;
use serde::export::Formatter;
use toml::de;
use std::io;

pub enum CommonErrorEnum{
    mongodb_error(mongodb::error::Error),
    common_error(String),
    toml_error(de::Error),
}


impl From<mongodb::error::Error> for CommonErrorEnum{
    fn from(error : mongodb::error::Error) -> Self {
        CommonErrorEnum::mongodb_error(error)
    }
}

impl From<de::Error> for CommonErrorEnum{
    fn from(error : de::Error) -> Self {
        CommonErrorEnum::toml_error(error)
    }
}
