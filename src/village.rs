use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Village {
    #[serde(rename = "Ortschaftsname")]
    pub name: String,
    #[serde(rename = "PLZ")]
    pub zip_code: i16,
    #[serde(rename = "Zusatzziffer")]
    pub one_digit_spare: i8,
    #[serde(rename = "Gemeindename")]
    pub commune: String,
    #[serde(rename = "BFS-Nr")]
    pub bfs_number: String,
    #[serde(rename = "Kantonskürzel")]
    pub canton: String,
    #[serde(rename = "E")]
    pub longitude: String,
    #[serde(rename = "N")]
    pub latitude: String,
    #[serde(rename = "Sprache")]
    pub language: String,
}
