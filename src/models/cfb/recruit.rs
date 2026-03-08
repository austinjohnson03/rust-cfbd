use crate::models::cfb::recruit_type::RecruitType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const MIN_YEAR: u32 = 1995;

#[derive(Debug, Deserialize, Serialize)]
pub struct Recruit {
    pub id: Uuid,
    pub recruit_type: RecruitType,
    pub year: u32,
    pub ranking: Option<u32>,
    pub name: String,
    pub school: Option<String>,
    pub committed_to: Option<String>,
    pub position: Option<String>, // Change to enum once I get all options
    pub height: Option<f64>,
    pub weight: Option<u32>,
    pub stars: u32,
    pub rating: f64,
}

impl Recruit {
    pub fn new(
        id: Uuid,
        recruit_type: RecruitType,
        year: u32,
        ranking: Option<u32>,
        name: String,
        school: Option<String>,
        committed_to: Option<String>,
        position: Option<String>,
        height: Option<f64>,
        weight: Option<u32>,
        stars: u32,
        rating: f64,
    ) -> Result<Self, String> {
        if year <= MIN_YEAR {
            return Err(format!(
                "Year must be greater than or equal to {}",
                MIN_YEAR
            ));
        }
        let ranking = ranking.filter(|r| *r >= 1);

        if name.is_empty() {
            return Err(String::from("Name cannot be empty."));
        }

        let school = school.filter(|s| !s.is_empty());
        let committed_to = committed_to.filter(|c| !c.is_empty());
        let position = position.filter(|p| !p.is_empty());
        let height = height.filter(|h| *h > 0.0);
        let weight = weight.filter(|w| *w > 0);
        if !(1..=5).contains(&stars) {
            return Err(String::from("Star rating must be between 1-5."));
        }

        if rating <= 0.0 {
            return Err(String::from("Rating must be greater than zero."));
        }

        Ok(Recruit {
            id,
            recruit_type,
            year,
            ranking,
            name,
            school,
            committed_to,
            position,
            height,
            weight,
            stars,
            rating,
        })
    }
}
