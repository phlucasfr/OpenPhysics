use crate::models::point::Point;

pub fn manhattan_distance(init: &Point, end: &Point) -> Result<i32, String> {
    if init.x < 0 || end.x < 0 || init.y < 0 || end.y < 0 {
        return Err("coordinates cannot be negative".to_string());
    }

    Ok((end.x - init.x).abs() + (end.y - init.y).abs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::point::Point;

    #[test]
    fn basic_test_manhattan() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 4, y: 6 };

        let distance = manhattan_distance(&p1, &p2);
        assert_eq!(distance, Ok(7));
    }

    #[test]
    fn error_on_negative_coordinates() {
        let p1 = Point { x: -1, y: 2 };
        let p2 = Point { x: 4, y: 6 };

        let distance = manhattan_distance(&p1, &p2);
        assert!(distance.is_err());
        assert_eq!(distance.unwrap_err(), "coordinates cannot be negative");
    }
}
