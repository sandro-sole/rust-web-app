use axum::{Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use tracing::info;
use crate::model::customer::{Account, AccountBmc, Customer, CustomerBmc};
use crate::model::ModelManager;

pub fn routes(mm: ModelManager) -> Router {
  let router = Router::new()
// `GET /` goes to `root`
    //.route("/customers", axum::routing::post(create_customer))
    .route("/customers", axum::routing::get(get_customers))
    .route("/accounts", axum::routing::post(create_account))
    .with_state(mm)
    ;
  router
}

pub async fn create_customer(
  // this argument tells axum to parse the request body
  // as JSON into a `CreateUser` type
  State(mm): State<ModelManager>,
  Json(payload): Json<Customer>,
) -> Response {
  // insert your application logic here
  let Ok(count) = CustomerBmc::create(&mm, payload.clone()).await else {
    return (StatusCode::INTERNAL_SERVER_ERROR, () ).into_response();
  };

  info!("{:<12} - {:?}\n", "CREATE", &payload);
  // this will be converted into a JSON response
  // with a status code of `201 Created`
  (StatusCode::CREATED, Json(payload)).into_response()
}

pub async fn get_customers(
  // this argument tells axum to parse the request body
  // as JSON into a `CreateUser` type
  State(mm): State<ModelManager>,
) -> Response {
  // insert your application logic here
  let list = CustomerBmc::list(&mm).await;
  dbg!(&list);

  let Ok(customers) = CustomerBmc::list(&mm).await else {
    return (StatusCode::INTERNAL_SERVER_ERROR, () ).into_response();
  };

  info!("{:<12} - {:?}\n", "LIST OF", &customers);
  // this will be converted into a JSON response
  // with a status code of `201 Created`
  (StatusCode::OK, Json(customers)).into_response()
}


pub async fn create_account(
  // this argument tells axum to parse the request body
  // as JSON into a `CreateUser` type
  State(mm): State<ModelManager>,
  Json(payload): Json<Account>,
) -> (StatusCode, Json<Account>) {
  // insert your application logic here

  AccountBmc::create(&mm, payload.clone()).await.unwrap();

  info!("{:<12} - {:?}\n", "CREATE", &payload);
  // this will be converted into a JSON response
  // with a status code of `201 Created`
  (StatusCode::CREATED, Json(payload))
}