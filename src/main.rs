#![allow(dead_code, unused_imports, unused_variables, unused, unused_mut)]

// region:    --- Modules

mod error;
mod model;

mod web;

use chrono::prelude::*;
pub use self::error::{Error, Result};

use axum::{middleware, Router};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;
use crate::web::customer_endpoint;

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	// -- FOR DEV ONLY

	// Initialize ModelManager.

	// -- Define Routes

	let routes_all = Router::new()
		// `GET /` goes to `root`
		.nest("/customer", customer_endpoint::routes())
	;

	// region:    --- Start Server
	let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
	info!("{:<12} - {addr}\n", "LISTENING");
	axum::Server::bind(&addr)
		.serve(routes_all.into_make_service())
		.await
		.unwrap();
	// endregion: --- Start Server

	Ok(())
}
