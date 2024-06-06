use serde::Deserialize;

fn default_bind_address() -> String {
    "0.0.0.0:3000".to_owned()
}

fn default_gateways() -> Vec<String> {
    vec!["http://127.0.0.1:8080".to_owned()]
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_bind_address")]
    pub bind_address: String,

    #[serde(default = "default_gateways")]
    pub gateways: Vec<String>,
}
