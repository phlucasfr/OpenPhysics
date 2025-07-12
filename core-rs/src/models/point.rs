use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}