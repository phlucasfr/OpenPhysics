use axum::{routing::post, Router};

use crate::controllers::manhattan_distance;

pub fn router() -> Router {
    Router::new().route("/", post(manhattan_distance::calculate))
}