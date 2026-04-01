use serde::{Deserialize, Serialize};
use std::fmt;

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
        let s = match self {
            DivisionClassification::Fbs => "FBS",
            DivisionClassification::Fcs => "FCS",
            DivisionClassification::Div2 => "II",
            DivisionClassification::Div3 => "III",
        };

        write!(f, "{}", s)
    }
}

impl DivisionClassification {
    pub fn is_division_1(&self) -> bool {
        match self {
            DivisionClassification::Fbs | DivisionClassification::Fcs => true,
            _ => false,
        }
    }
}
