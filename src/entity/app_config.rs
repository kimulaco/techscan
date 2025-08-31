use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct AppConfig {
    pub exclude: Option<Vec<String>>,
    pub reporter: Option<String>,
}
