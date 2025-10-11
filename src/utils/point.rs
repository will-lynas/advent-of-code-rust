use std::{
    fmt::{
        self,
        Debug,
        Formatter,
    },
    ops::{
        Add,
        AddAssign,
        Mul,
        Sub,
    },
};

use num::integer::gcd;

use super::direction::Direction;

#[derive(PartialOrd, Ord, Hash, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub y: i32,
    pub x: i32,
}

impl Point {
    pub const ORIGIN: Self = Self::new(0, 0);

    pub const UP: Self = Self::new(0, -1);
    pub const DOWN: Self = Self::new(0, 1);
    pub const LEFT: Self = Self::new(-1, 0);
    pub const RIGHT: Self = Self::new(1, 0);

    pub const UP_LEFT: Self = Self::new(-1, -1);
    pub const UP_RIGHT: Self = Self::new(1, -1);
    pub const DOWN_RIGHT: Self = Self::new(1, 1);
    pub const DOWN_LEFT: Self = Self::new(-1, 1);

    pub const ORTHOGONALS: [Self; 4] = [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT];
    pub const DIRS: [Self; 8] = [
        Self::UP,
        Self::UP_RIGHT,
        Self::RIGHT,
        Self::DOWN_RIGHT,
        Self::DOWN,
        Self::DOWN_LEFT,
        Self::LEFT,
        Self::UP_LEFT,
    ];

    pub const fn new(x: i32, y: i32) -> Self {
        Self { y, x }
    }

    pub fn rotate_clockwise(&mut self) {
        (self.x, self.y) = (-self.y, self.x);
    }

    #[must_use]
    pub fn rotated(&self, direction: Direction) -> Self {
        match direction {
            Direction::Clockwise => Self::new(-self.y, self.x),
            Direction::Anticlockwise => Self::new(self.y, -self.x),
        }
    }

    pub fn orthogonals(&self) -> Vec<Self> {
        Self::ORTHOGONALS
            .iter()
            .map(move |&dir| *self + dir)
            .collect()
    }

    pub fn neighbors(&self) -> Vec<Self> {
        Self::DIRS.iter().map(move |&dir| *self + dir).collect()
    }

    #[must_use]
    pub fn normalized(&self) -> Self {
        if self == &Self::ORIGIN {
            return Self::ORIGIN;
        }
        let n = gcd(self.x, self.y);
        Self {
            x: self.x / n,
            y: self.y / n,
        }
    }

    pub fn norm1(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn dot(&self, other: Self) -> i32 {
        self.x * other.x + self.y * other.y
    }

    pub fn orthogonal(&self, other: Self) -> bool {
        self.dot(other) == 0
    }

    pub fn between(&self, other: Self) -> impl Iterator<Item = Self> {
        (self.y..=other.y).flat_map(move |y| (self.x..=other.x).map(move |x| Self::new(x, y)))
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, n: i32) -> Self {
        Self::new(self.x * n, self.y * n)
    }
}

impl Mul<usize> for Point {
    type Output = Point;

    fn mul(self, n: usize) -> Self {
        Self::new(self.x * n as i32, self.y * n as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalized_origin() {
        let origin = Point::new(0, 0);
        assert_eq!(origin.normalized(), origin);
    }

    #[test]
    fn test_normalized_positive() {
        let point = Point::new(6, 8);
        let normalized = point.normalized();
        assert_eq!(normalized, Point::new(3, 4));
    }

    #[test]
    fn test_normalized_negative() {
        let point = Point::new(-6, -8);
        let normalized = point.normalized();
        assert_eq!(normalized, Point::new(-3, -4));
    }

    #[test]
    fn test_normalized_mixed_signs() {
        let point = Point::new(6, -8);
        let normalized = point.normalized();
        assert_eq!(normalized, Point::new(3, -4));
    }

    #[test]
    fn test_normalized_already_normalized() {
        let point = Point::new(3, 4);
        assert_eq!(point.normalized(), point);
    }
}
