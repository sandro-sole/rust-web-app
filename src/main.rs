// #![allow(dead_code, unused_imports, unused_variables, unused, unused_mut)]

// region:    --- Modules

mod error;
mod model;

pub use self::error::{Error, Result};

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

	// -- Define Routes

	let routes_all = Router::new()
		// `GET /` goes to `root`
		.route("/", axum::routing::get(crate::web::root))
		.route("/users", axum::routing::post(crate::web::create_user))
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

mod web {
	use serde::{Deserialize, Serialize};
	use axum::{
		routing::{get, post},
		http::StatusCode,
		response::IntoResponse,
		Json, Router,
	};

	// basic handler that responds with a static string
	pub async fn root() -> &'static str {
		"Hello, World!"
	}


	pub async fn create_user(
		// this argument tells axum to parse the request body
		// as JSON into a `CreateUser` type
		Json(payload): Json<CreateUser>,
	) -> (StatusCode, Json<User>) {
		// insert your application logic here
		let user = User {
			id: 1337,
			username: payload.username,
		};

		// this will be converted into a JSON response
		// with a status code of `201 Created`
		(StatusCode::CREATED, Json(user))
	}

	// the input to our `create_user` handler
	#[derive(Deserialize)]
	pub struct CreateUser {
		username: String,
	}

	// the output to our `create_user` handler
	#[derive(Serialize)]
	pub struct User {
		id: u64,
		username: String,
	}
}
