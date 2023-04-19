use std::{convert::Infallible, io::Error};

use poem::{
    handler, listener::TcpListener, middleware::Cors, post, web::Json, EndpointExt, Route, Server,
};
use serde::Deserialize;

use crate::config::state::AppConfig;

#[derive(Debug, Deserialize)]
struct HelloRequest {
    name: String,
}

#[handler]
fn index(req: Json<HelloRequest>) -> Json<serde_json::Value> {
    Json(serde_json::json! ({
        "code": 0,
        "message": req.name,
    }))
}

pub async fn run_poem_app(
    config: &AppConfig,
    server: Server<TcpListener<String>, Infallible>,
) -> Result<(), Error> {
    let cors = Cors::new();

    let app = Route::new().at("/", post(index)).with(cors);

    println!("Starting server at http://{}:{}", config.host, config.port);
    server.run(app).await
}
