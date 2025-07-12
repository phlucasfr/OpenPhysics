use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::{
    mechanics::distance_traveled::distance_traveled,
    models::{api::ApiResponse, point::Point},
};

pub async fn calculate(
    Json(params): Json<Vec<Point>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match distance_traveled(&params) {
        Ok(distance) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                data: Some(distance),
                message: Some("distance_traveled".to_string()),
            }),
        )),
        Err(msg) => Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()> {
                data: None,
                message: Some(msg),
            }),
        )),
    }
}
