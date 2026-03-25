use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SeasonType {
    Regular,
    Postseason,
    Both,
    Allstar,
    #[serde(rename = "spring_regular")]
    SpringRegular,
    #[serde(rename = "spring_postseason")]
    SpringPostseason,
}

impl fmt::Display for SeasonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            SeasonType::Regular => "Regular",
            SeasonType::Postseason => "Postseason",
            SeasonType::Both => "Both",
            SeasonType::Allstar => "Allstar",
            SeasonType::SpringRegular => "Spring Regular",
            SeasonType::SpringPostseason => "Spring Postseason",
        };

        write!(f, "{}", s)
    }
}
