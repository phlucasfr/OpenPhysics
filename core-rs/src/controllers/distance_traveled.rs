use axum::{extract::Json, http::StatusCode, response::IntoResponse};

use crate::{
    mechanics::distance_traveled::distance_traveled,
    models::{api::ApiResponse, point::Point},
};

pub async fn calculate(Json(params): Json<Vec<Point>>) -> impl IntoResponse {
    let distance = distance_traveled(&params);
    (
        StatusCode::OK,
        Json(ApiResponse {
            data: Some(distance),
            message: Some("distance_traveled".to_string()),
        }),
    )
}
