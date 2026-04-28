use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

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

impl FromStr for SeasonType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "regular" => Ok(SeasonType::Regular),
            "postseason" => Ok(SeasonType::Postseason),
            "both" => Ok(SeasonType::Both),
            "allstar" => Ok(SeasonType::Allstar),
            "spring regular" => Ok(SeasonType::SpringRegular),
            "spring postseason" => Ok(SeasonType::SpringPostseason),
            _ => Err(format!("Invalid season type: {}", s)),
        }
    }
}
