use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    pub name: String,
    pub abbreviation: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Country {
    pub name: String,
    pub code: String,
    pub numeric_code: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub id: Uuid,
    pub city: String,
    pub state: Option<State>,
    pub country: Country,
    pub timezone: Option<String>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub zip_code: Option<String>,
    pub fips_code: Option<String>,
}

impl State {
    pub fn new(name: String, abbreviation: String) -> Result<Self, String> {
        if name.is_empty() {
            return Err(String::from("State name cannot be empty."));
        }

        if abbreviation.is_empty() {
            return Err(String::from("State abbreviation cannot be empty."));
        }
        Ok(State { name, abbreviation })
    }
}

impl Country {
    pub fn new(name: String, code: String, numeric_code: i32) -> Result<Self, String> {
        if name.is_empty() {
            return Err(String::from("Country name cannot be empty."));
        }

        if code.len() != 3 {
            return Err(String::from("Country code must be 3 characters in length."));
        }

        if numeric_code < 1 || numeric_code > 999 {
            return Err(String::from(
                "Numeric country code must be valid ISO 3166-1 code.",
            ));
        }

        Ok(Country {
            name,
            code: code.to_uppercase(),
            numeric_code,
        })
    }
}

impl Location {
    pub fn new(
        id: Uuid,
        city: String,
        state: Option<State>,
        country: Country,
        timezone: Option<String>,
        latitude: Option<f32>,
        longitude: Option<f32>,
        zip_code: Option<String>,
        fips_code: Option<String>,
    ) -> Self {
    }
}
