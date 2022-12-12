use std::fmt::{self, Display};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LocationData {
    pub name: String,
    pub ascii_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub country_code: String,
    pub timezone: String,
    pub admin1: Option<String>,
    pub admin2: Option<String>,
    pub feature_code: String,
    pub feature_class: Option<String>,
    pub modification_date: NaiveDate,
    pub elevation: Option<usize>,
    pub population: Option<usize>,
}

impl Display for LocationData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            vec![
                self.name.clone(),
                self.admin1.clone().unwrap_or_default(),
                self.admin2.clone().unwrap_or_default(),
                self.country_code.clone(),
            ]
            .into_iter()
            .filter(|s| !s.trim().is_empty())
            .collect::<Vec<_>>()
            .join(", "),
        )
    }
}
