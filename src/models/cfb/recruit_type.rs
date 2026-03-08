use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum RecruitType {
    HighSchool,
    JUCO,
    PrepSchool,
}
