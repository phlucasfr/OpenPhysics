use serde::Deserialize;

use crate::models::point::Point;

#[derive(Deserialize)]
pub struct DistanceParams {
    pub init: Point,
    pub end: Point,
}
