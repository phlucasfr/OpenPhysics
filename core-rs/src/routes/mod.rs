use axum::{Router, routing::post};

use crate::{controllers::distance_traveled, controllers::manhattan_distance};

pub fn router() -> Router {
    Router::new()
        .route("/manhattan", post(manhattan_distance::calculate))
        .route("/distance-traveled", post(distance_traveled::calculate))
}
