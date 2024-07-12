use std::ops::Sub;

#[derive(Debug, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T> Sub for Point<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Point<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substraction_of_points() {
        let first = Point::new(3, 2);
        let second = Point::new(2, -1);
        let result = Point::new(1, 3);
        assert_eq!(result, first - second);
    }
}
