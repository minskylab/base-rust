use config::{Config, Environment};
use dotenv::dotenv;
use serde_derive::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]

pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

pub fn load_config_from_dotenv() -> AppConfig {
    dotenv().ok();

    let config = Config::builder()
        .set_default("host", "127.0.0.1")
        .unwrap()
        .set_default("port", 8080)
        .unwrap()
        .add_source(
            Environment::with_prefix("APP")
                .try_parsing(true)
                .separator("_")
                .list_separator(" "),
        )
        .build()
        .unwrap();

    let app: AppConfig = config.try_deserialize().unwrap();

    app
}
