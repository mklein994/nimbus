#[derive(Debug, Deserialize)]
pub struct Config {
    pub coordinates: Coordinates,
    pub token: String,
    /// darksky::Language
    pub language: Option<String>,
    /// darksky::Unit
    pub unit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}
