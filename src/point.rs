use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

/// Represents 3D point
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    /// Creates new [`Point`] with given coordinates
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Creates new [`Point`] with all coords zero
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Calculates distance between current [`Point`] and given [`Point`]
    pub fn dist(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2)
            + (self.y - other.y).powi(2)
            + (self.z - other.z).powi(2))
        .sqrt()
    }

    pub fn div(&mut self, div: f64) {
        self.x /= div;
        self.y /= div;
        self.z /= div;
    }
}

impl From<(i64, i64, i64)> for Point {
    fn from((x, y, z): (i64, i64, i64)) -> Self {
        Self::new(x as f64, y as f64, z as f64)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res += rhs;
        res
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.x as i64, self.y as i64, self.z as i64
        )
    }
}
