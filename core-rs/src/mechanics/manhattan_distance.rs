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
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 4, y: 6 };

        let distance = manhattan_distance(&p1, &p2);
        assert_eq!(distance, 7);
    }
}
