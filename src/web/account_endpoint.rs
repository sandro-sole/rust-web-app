use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use tracing::info;
use crate::model::account::{Account, AccountBmc, CreateAccount};
use crate::model::ModelManager;

pub fn routes(mm: ModelManager) -> Router {
  let router = Router::new()
// `GET /` goes to `root`
    //.route("/customers", axum::routing::post(create_customer))
    .route("/", axum::routing::get(get_accounts))
    .route("/", axum::routing::post(post_account))
    .route("/:id", axum::routing::get(get_account))
    .with_state(mm)
    ;
  router
}

pub async fn get_account(
  State(mm): State<ModelManager>,
  Path((id,)): Path<(String,)>
) -> Response {
  let Ok(Some(customer)) = AccountBmc::get(&mm, id.as_str()).await else {
    return (StatusCode::INTERNAL_SERVER_ERROR, () ).into_response();
  };
  (StatusCode::OK, Json(customer)).into_response()
}

pub async fn create_customer(
  // this argument tells axum to parse the request body
  // as JSON into a `CreateUser` type
  State(mm): State<ModelManager>,
  Json(payload): Json<Account>,
) -> Response {
  // insert your application logic here
  let Ok(count) = AccountBmc::create(&mm, &payload).await else {
    return (StatusCode::INTERNAL_SERVER_ERROR, () ).into_response();
  };

  info!("{:<12} - {:?}\n", "CREATE", &payload);
  // this will be converted into a JSON response
  // with a status code of `201 Created`
  (StatusCode::CREATED, Json(payload)).into_response()
}

pub async fn get_accounts(
  // this argument tells axum to parse the request body
  // as JSON into a `CreateUser` type
  State(mm): State<ModelManager>,
) -> Response {
  let Ok(customers) = AccountBmc::list(&mm).await else {
    return (StatusCode::INTERNAL_SERVER_ERROR, () ).into_response();
  };

  info!("{:<12} - {:?}\n", "LIST OF", &customers);
  // this will be converted into a JSON response
  // with a status code of `201 Created`
  (StatusCode::OK, Json(customers)).into_response()
}

pub async fn post_account(
  // this argument tells axum to parse the request body
  // as JSON into a `CreateUser` type
  State(mm): State<ModelManager>,
  Json(payload): Json<CreateAccount>,
) -> (StatusCode, Json<Account>) {
  // insert your application logic here
  let account = payload.into();
  AccountBmc::create(&mm, &account).await.unwrap();

  info!("{:<12} - {:?}\n", "CREATE", &account);
  // this will be converted into a JSON response
  // with a status code of `201 Created`
  (StatusCode::CREATED, Json(account))
}