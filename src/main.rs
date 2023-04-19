use base_rust::{
    config::state::load_config_from_dotenv,
    service::{app::run_poem_app, server::new_poem_server},
};

#[tokio::main]
async fn main() {
    let config = load_config_from_dotenv();

    let server = new_poem_server(&config);

    run_poem_app(&config, server).await.unwrap()
}
