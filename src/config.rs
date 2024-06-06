use serde::Deserialize;


#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    pub bind_address: String,
    pub gateways: Vec<String>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1:3345".to_owned(),
            gateways: vec!["http://127.0.0.1:8080".to_owned()],
        }
    }
}
