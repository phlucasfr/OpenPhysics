use axum::{Json, extract::Json as ExtractJson, http::StatusCode, response::IntoResponse};

use crate::{
    mechanics::manhattan_distance::manhattan_distance,
    models::{api::ApiResponse, manhattan_distance::DistanceParams},
};

pub async fn calculate(ExtractJson(params): ExtractJson<DistanceParams>) -> Result<impl IntoResponse, impl IntoResponse> {
    match manhattan_distance(&params.init, &params.end) {
        Ok(distance) => Ok((
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(distance),
                message: Some("manhattan_distance".to_string()),
            }),
        )),
        Err(msg) => Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some(msg),
            }),
        )),
    }
}
