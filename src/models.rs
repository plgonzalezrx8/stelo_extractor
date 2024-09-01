use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BloodGlucoseEntry {
    #[serde(rename = "sourceName")]
    pub source_name: String,
    #[serde(rename = "creationDate")]
    pub creation_date: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub time: String,
    pub value: String,
    pub unit: String,
}
