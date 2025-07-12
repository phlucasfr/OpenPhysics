use axum::{routing::post, Router};

use crate::controllers::manhattan_distance;

pub fn router() -> Router {
    Router::new().route("/manhattan", post(manhattan_distance::calculate))
}