use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecruitTeamQuery {
    pub year: Option<i32>,
    pub team: Option<String>,
}

pub struct RecruitTeamQueryBuilder {
    year: Option<i32>,
    team: Option<String>,
}

impl RecruitTeamQueryBuilder {
    pub fn new() -> Self {
        RecruitTeamQueryBuilder {
            year: None,
            team: None,
        }
    }

    pub fn year(mut self, year: i32) -> Self {
        self.year = Some(year);
        self
    }

    pub fn team(mut self, team: impl Into<String>) -> Self {
        self.team = Some(team.into());
        self
    }

    pub fn build(self) -> RecruitTeamQuery {
        RecruitTeamQuery {
            year: self.year,
            team: self.team,
        }
    }
}

impl Default for RecruitTeamQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
