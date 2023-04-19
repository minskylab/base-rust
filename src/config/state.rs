use config::{Config, Environment};
use dotenv::dotenv;
use serde_derive::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]

pub struct AppConfig {
    host: String,
    port: u16,
}

pub fn load_config_from_dotenv() -> AppConfig {
    dotenv().ok();

    let config = Config::builder()
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
