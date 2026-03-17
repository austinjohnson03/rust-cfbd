use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    conference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<i32>,
}

impl TeamQuery {
    pub fn new() -> Self {
        TeamQuery {
            conference: None,
            year: None,
        }
    }

    pub fn conference(mut self, conference: String) -> Self {
        self.conference = Some(conference);
        self
    }

    pub fn year(mut self, year: i32) -> Self {
        self.year = Some(year);
        self
    }
}
