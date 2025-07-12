use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}