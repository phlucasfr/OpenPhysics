use crate::{
    mechanics::displacement_vector::displacement_vector,
    models::{api::ApiResponse, manhattan_distance::DistanceParams},
};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};

pub async fn calculate(Json(params): Json<DistanceParams>) -> impl IntoResponse {
    let distance = displacement_vector(&params.init, &params.end);
    (
        StatusCode::OK,
        Json(ApiResponse {
            data: Some(distance),
            message: Some("displacement_vector".to_string()),
        }),
    )
}
