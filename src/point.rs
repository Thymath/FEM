use std::ops::Sub;

pub struct Point<T> {
    pub x: T,
    pub y: T,
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
