use std::convert::Infallible;

use poem::{listener::TcpListener, Server};

use crate::config::state::AppConfig;

pub fn new_poem_server(config: &AppConfig) -> Server<TcpListener<String>, Infallible> {
    let address = format!("{}:{}", config.host, config.port);

    Server::new(TcpListener::bind(address))
}
