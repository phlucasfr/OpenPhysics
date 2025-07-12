use axum::{routing::post, Router};

use crate::controllers::mechanics;

pub fn router() -> Router {
    Router::new().route("/", post(mechanics::calculate))
}