use crate::models::point::Point;

pub fn manhattan_distance(init: &Point, end: &Point) -> i32 {
    (end.x - init.x).abs() + (end.y - init.y).abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::point::Point;

    #[test]
    fn basic_test_manhattan() {
        let p1 = Point{x: 10, y: 0};
        let p2 = Point{x: 10, y: -5};

        let distance = manhattan_distance(&p1, &p2);
        assert_eq!(distance, 5);
    }
}