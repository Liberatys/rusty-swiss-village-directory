use serde::Deserialize;
use rutie::{ RString, AnyObject, Object, Class, VerifiedObject };
use rutie::types::{ Value, ValueType };

#[derive(Debug, Deserialize, Clone)]
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

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn bfs_number(&self) -> &str {
        &self.bfs_number
    }

    pub fn latitude(&self) -> &str {
        &self.latitude
    }

    pub fn longitude(&self) -> &str {
        &self.longitude
    }

    pub fn commune(&self) -> &str {
        &self.commune
    }

    pub fn canton(&self) -> &str {
        &self.canton
    }

    pub fn zip_code(&self) -> i16 {
        self.zip_code
    }
}
