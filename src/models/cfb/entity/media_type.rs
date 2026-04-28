use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Tv,
    Radio,
    Web,
    Ppv,
    Mobile,
}

impl FromStr for MediaType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tv" => Ok(MediaType::Tv),
            "radio" => Ok(MediaType::Radio),
            "web" => Ok(MediaType::Web),
            "ppv" => Ok(MediaType::Ppv),
            "mobile" => Ok(MediaType::Mobile),
            _ => Err(format!("Invalid media type: {}", s)),
        }
    }
}
