use axum::{debug_handler, extract::State, handler, routing::post, Json};
use headers::UserAgent;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::http::outgoing::UserLogin;

use super::{outgoing, AppContext};

pub fn router() -> axum::Router<AppContext> {
    axum::Router::new()
        .route("/", post(auth_handler))
        // .route("/", todo!())
}

async fn auth_handler(ctx: State<AppContext>, Json(req): Json<UserLogin>) -> super::Result<Json<LoginResponse>> {
    match outgoing::log_in(req.username, req.password).await {
        Ok(output) => {
            Ok(Json(LoginResponse { phone: output.data.user.user_phone, token: output.data.user.token }))
        },
        Err(e) => {
            warn!("Error {e}");
            // HttpError::
            Err(crate::http::error::HttpError::Unauthorized)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginResponse {
    phone: String,
    token: String,
}

async fn update_handler(ctx: State<AppContext>) -> super::Result<String> {
    todo!()
}
