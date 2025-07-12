use crate::{mechanics::manhattan_distance::manhattan_distance, models::point::Point};

pub fn distance_traveled(points: &[Point]) -> Result<i32, String> {
    let mut distance = 0;
    for i in 1..points.len() {
        distance += manhattan_distance(&points[i - 1], &points[i])?;
    }
    Ok(distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_traveled_basic() {
        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 3, y: 0 },
            Point { x: 3, y: 4 },
        ];

        let result = distance_traveled(&points);
        assert_eq!(result, Ok(7));
    }

    #[test]
    fn test_distance_traveled_single_point() {
        let points = vec![Point { x: 0, y: 0 }];
        let result = distance_traveled(&points);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_distance_traveled_empty() {
        let points = vec![];
        let result = distance_traveled(&points);
        assert_eq!(result, Ok(0));
    }
}
