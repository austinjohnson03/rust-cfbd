use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DivisionClassification {
    Fbs,
    Fcs,
    #[serde(rename = "ii")]
    Div2,
    #[serde(rename = "iii")]
    Div3,
}

impl fmt::Display for DivisionClassification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = matches!(
            self,
            DivisionClassification::Fbs | DivisionClassification::Fcs
        );
        write!(f, "{}", s)
    }
}

impl DivisionClassification {
    pub fn is_division_1(&self) -> bool {
        matches!(
            self,
            DivisionClassification::Fbs | DivisionClassification::Fcs
        )
    }

    pub fn is_fbs(&self) -> bool {
        matches!(self, DivisionClassification::Fbs)
    }

    pub fn all() -> Vec<DivisionClassification> {
        vec![
            DivisionClassification::Fbs,
            DivisionClassification::Fcs,
            DivisionClassification::Div2,
            DivisionClassification::Div3,
        ]
    }

    pub fn label(&self) -> &str {
        match self {
            DivisionClassification::Fbs => "FBS",
            DivisionClassification::Fcs => "FCS",
            DivisionClassification::Div2 => "II",
            DivisionClassification::Div3 => "III",
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            DivisionClassification::Fbs => "fbs",
            DivisionClassification::Fcs => "fcs",
            DivisionClassification::Div2 => "ii",
            DivisionClassification::Div3 => "iii",
        }
    }
}

impl FromStr for DivisionClassification {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fbs" => Ok(DivisionClassification::Fbs),
            "fcs" => Ok(DivisionClassification::Fcs),
            "ii" => Ok(DivisionClassification::Div2),
            "iii" => Ok(DivisionClassification::Div3),
            _ => Err(format!("Invalid division classification: {}", s)),
        }
    }
}
