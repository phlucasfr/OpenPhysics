use crate::models::point::Point;

pub fn displacement_vector(init: &Point, end: &Point) -> Point {
    Point {
        x: end.x - init.x,
        y: end.y - init.y,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_displacement_positive_direction() {
        let a = Point { x: 1, y: 2 };
        let b = Point { x: 4, y: 6 };
        let result = displacement_vector(&a, &b);
        assert_eq!(result.x, 3);
        assert_eq!(result.y, 4);
    }

    #[test]
    fn test_displacement_negative_direction() {
        let a = Point { x: 5, y: 5 };
        let b = Point { x: 2, y: 1 };
        let result = displacement_vector(&a, &b);
        assert_eq!(result.x, -3);
        assert_eq!(result.y, -4);
    }

    #[test]
    fn test_displacement_zero_vector() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 0, y: 0 };
        let result = displacement_vector(&a, &b);
        assert_eq!(result.x, 0);
        assert_eq!(result.y, 0);
    }
}