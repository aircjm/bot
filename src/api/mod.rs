use std::sync::Arc;

use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put, Router},
    Extension, Json,
};

use crate::{context::Context, infra::error_status::ErrorStatus};

use serde::{Deserialize, Serialize};

pub mod common;

//
// pub fn make_api_doc_route(route: Router) -> Router {
//     cloud_infra::with_api_doc(route, ApiDoc::openapi(), env!("CARGO_PKG_NAME"))
// }

pub fn make_rest_route(ctx: Arc<Context>) -> Router {
    Router::new()
        .route("/healthz", get(common::health_check))
        .route("/ping", get(common::ping))
        .route("/user", get(query_user))
        // // TODO: Will consider this permission in the future
        // .route(
        //     "/workspace/:id/blob/:name",
        //     get(blobs::get_blob_in_workspace),
        // )
        // .nest(
        //     "/",
        //     Router::new()
        //         .route(
        //             "/workspace",
        //             get(workspace::get_workspaces).post(workspace::create_workspace),
        //         )
        //         .route(
        //             "/workspace/:id",
        //             get(workspace::get_workspace_by_id)
        //                 .post(workspace::update_workspace)
        //                 .delete(workspace::delete_workspace),
        //         )
        //         .route("/workspace/:id/doc", get(workspace::get_doc))
        //         .route("/workspace/:id/search", post(workspace::search_workspace))
        //         .route(
        //             "/workspace/:id/permission",
        //             get(permissions::get_members)
        //                 .post(permissions::invite_member)
        //                 .delete(permissions::leave_workspace),
        //         )
        //         .route("/workspace/:id/blob", put(blobs::upload_blob_in_workspace))
        //         .route("/permission/:id", delete(permissions::remove_user))
        //         .route("/resource/usage", get(blobs::get_user_resource_usage))
        //         // .layer(make_firebase_auth_layer(ctx.key.jwt_decode.clone())),
        // )
}


pub async fn query_user(
    Extension(ctx): Extension<Arc<Context>>,
    Query(payload): Query<UserQuery>,
) -> Response {
    
    // info!("query_user enter");
    // if let (Some(email), Some(workspace_id)) = (payload.email, payload.workspace_id) {
    //     if let Ok(user) = ctx
    //         .db
    //         .get_user_in_workspace_by_email(workspace_id, &email)
    //         .await
    //     {
    //         Json(vec![user]).into_response()
    //     } else {
    //         ErrorStatus::InternalServerError.into_response()
    //     }
    // } else {
        ErrorStatus::BadRequest.into_response()
    // }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserQuery {
    pub email: Option<String>,
    pub workspace_id: Option<String>,
}