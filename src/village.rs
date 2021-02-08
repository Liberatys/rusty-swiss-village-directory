use serde::Deserialize;
use rutie::{ RString, AnyObject, Object, Class, VerifiedObject };
use rutie::types::{ Value, ValueType };

#[derive(Debug, Deserialize, Clone)]
pub struct Village {
    #[serde(rename = "Ortschaftsname")]
    name: String,
    #[serde(rename = "PLZ")]
    zip_code: i16,
    #[serde(rename = "Zusatzziffer")]
    one_digit_spare: i8,
    #[serde(rename = "Gemeindename")]
    commune: String,
    #[serde(rename = "BFS-Nr")]
    bfs_number: String,
    #[serde(rename = "Kantonskürzel")]
    canton: String,
    #[serde(rename = "E")]
    longitude: String,
    #[serde(rename = "N")]
    latitude: String,
    #[serde(rename = "Sprache")]
    language: String,
}

impl Village {
    pub fn name(&self) -> &str {
        &self.name
    }
}
