use axum::{Json, Router};
use axum::http::StatusCode;
use tracing::info;
use crate::model::customer::Customer;

pub fn routes() -> Router {
  let router = Router::new()
// `GET /` goes to `root`
    .route("/customers", axum::routing::post(create_customer))
    ;
  router
}

pub async fn create_customer(
  // this argument tells axum to parse the request body
  // as JSON into a `CreateUser` type
  Json(payload): Json<Customer>,
) -> (StatusCode, Json<Customer>) {
  // insert your application logic here
  let customer = Customer {
    ..payload
  };

  info!("{:<12} - {:?}\n", "CREATE", &customer);
  // this will be converted into a JSON response
  // with a status code of `201 Created`
  (StatusCode::CREATED, Json(customer))
}