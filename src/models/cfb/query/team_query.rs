use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    conference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<i32>,
}

pub struct TeamQueryBuilder {
    conference: Option<String>,
    year: Option<i32>,
}

impl Default for TeamQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TeamQueryBuilder {
    pub fn new() -> Self {
        TeamQueryBuilder {
            conference: None,
            year: None,
        }
    }

    pub fn conference(mut self, conference: impl Into<String>) -> Self {
        self.conference = Some(conference.into());
        self
    }

    pub fn year(mut self, year: i32) -> Self {
        self.year = Some(year);
        self
    }

    pub fn build(self) -> TeamQuery {
        TeamQuery {
            conference: self.conference,
            year: self.year,
        }
    }
}
