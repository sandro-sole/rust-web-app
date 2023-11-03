use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use tracing::info;
use crate::model::account::{Account, AccountBmc};
use crate::model::customer::{Customer, CustomerBmc};
use crate::model::ModelManager;
use crate::model::role::{CreateRole, Role, RoleBmc};

pub fn routes(mm: ModelManager) -> Router {
  let router = Router::new()
    .route("/:cid/:aid", axum::routing::post( post_role))
    .with_state(mm)
    ;
  router
}

pub async fn post_role(
  // this argument tells axum to parse the request body
  // as JSON into a `CreateUser` type
  State(mm): State<ModelManager>,
  Path((cid,aid,)): Path<(String,String,)>,
  Json(role_payload): Json<CreateRole>,
) -> Response {
  // insert your application logic here
  /*
  RELATE "customer:⟨shree-id⟩"->role->account:⟨sree-account⟩
	CONTENT {
        type: "Owner",
        created_at: time::now(),
		status: "Active"
	};
   */
  let Ok(Some(customer)) = CustomerBmc::get(&mm, &cid).await else {
    return (StatusCode::INTERNAL_SERVER_ERROR, () ).into_response();
  };
  let Ok(Some(account)) = AccountBmc::get(&mm, &aid).await else {
    return (StatusCode::INTERNAL_SERVER_ERROR, () ).into_response();
  };

  dbg!(&customer);
  dbg!(&account);
  dbg!(&role_payload);
  //info!("{:<12} - {:?}\n", "CREATE", &role_payload);
  let Ok(count) = RoleBmc::relate(&mm, &customer, &account, &role_payload).await else {
    return (StatusCode::INTERNAL_SERVER_ERROR, () ).into_response();
  };

  // this will be converted into a JSON response
  // with a status code of `201 Created`
  (StatusCode::CREATED, Json(json!({}))).into_response()
}
