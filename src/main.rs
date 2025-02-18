#![allow(dead_code, unused_imports, unused_variables, unused, unused_mut)]

// region:    --- Modules

mod error;
mod model;

mod web;

pub use self::error::{Error, Result};

use chrono::prelude::*;

use crate::model::ModelManager;
use crate::web::{account_endpoint, customer_endpoint, role_endpoint};
use axum::{middleware, Router};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	// -- FOR DEV ONLY

	// Initialize ModelManager.
	let mm = ModelManager::new().await?;

	// -- Define Routes
	let routes_all = Router::new()
		// `GET /` goes to `root`
		.nest("/customer", customer_endpoint::routes(mm.clone()))
		.nest("/account", account_endpoint::routes(mm.clone()))
		.nest("/role", role_endpoint::routes(mm.clone()))
		;

	// region:    --- Start Server
	let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
	info!("{:<12} - {addr}\n", "LISTENING");
	axum::Server::bind(&addr)
		.serve(Router::new().nest("/api", routes_all).into_make_service())
		.await
		.unwrap();
	// endregion: --- Start Server

	Ok(())
}
