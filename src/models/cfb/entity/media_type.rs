use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Tv,
    Radio,
    Web,
    Ppv,
    Mobile,
}
