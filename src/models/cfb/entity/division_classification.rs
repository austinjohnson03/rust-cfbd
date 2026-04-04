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
}
