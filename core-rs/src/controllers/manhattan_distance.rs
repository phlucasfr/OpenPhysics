use crate::{
    mechanics::manhattan_distance::manhattan_distance,
    models::{api::ApiResponse, manhattan_distance::DistanceParams},
};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};

pub async fn calculate(Json(params): Json<DistanceParams>) -> impl IntoResponse {
    let distance = manhattan_distance(&params.init, &params.end);
    (
        StatusCode::OK,
        Json(ApiResponse {
            data: Some(distance),
            message: Some("manhattan_distance".to_string()),
        }),
    )
}
