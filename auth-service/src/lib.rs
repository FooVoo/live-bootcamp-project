extern crate pretty_env_logger;
pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

use app_state::AppState;
use axum::http::Method;
use axum::routing::post;
use axum::serve::Serve;
use axum::Router;
use std::error::Error;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let asset_dir =
            ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            "http://gate.foovoo.dev:8000".parse()?,
            "http://139.59.135.189:8000".parse()?,
        ];

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .fallback_service(asset_dir)
            .route("/signup", post(routes::signup_handler))
            .route("/login", post(routes::login_handler))
            .route("/verify-2fa", post(routes::verify_2fa_handler))
            .route("/logout", post(routes::logout_handler))
            .route("/verify-token", post(routes::verify_token_handler))
            .with_state(app_state)
            .layer(cors);

        let listener = TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();

        let server = axum::serve(listener, router);

        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}
