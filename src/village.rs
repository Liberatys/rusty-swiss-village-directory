use serde::Deserialize;
use rutie::{ RString, AnyObject, Object, Class, VerifiedObject };
use rutie::types::{ Value, ValueType };

#[derive(Deserialize, Clone)]
pub struct Village {
    #[serde(rename = "Ortschaftsname")]
    name: Box<str>,
    #[serde(rename = "PLZ")]
    zip_code: i16,
    #[serde(rename = "Zusatzziffer")]
    one_digit_spare: i8,
    #[serde(rename = "Gemeindename")]
    commune: Box<str>,
    #[serde(rename = "BFS-Nr")]
    bfs_number: Box<str>,
    #[serde(rename = "Kantonskürzel")]
    canton: Box<str>,
    #[serde(rename = "E")]
    longitude: Box<str>,
    #[serde(rename = "N")]
    latitude: Box<str>,
    #[serde(rename = "Sprache")]
    language: Box<str>,
}

impl Village {
    pub fn name(&self) -> &str {
        &self.name
    }
}
