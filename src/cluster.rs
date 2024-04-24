use std::fmt::Display;

use crate::point::Point;

/// Represents cluster
pub struct Cluster<'a> {
    pub centroid: Point,
    pub points: Vec<&'a Point>,
}

impl<'a> Cluster<'a> {
    /// Creates new [`Cluster`] with given point as center of gravity
    pub fn new<T>(center: T) -> Self
    where
        T: Into<Point>,
    {
        Self {
            centroid: center.into(),
            points: vec![],
        }
    }

    /// Adds [`Point`] to the cluster
    pub fn add(&mut self, point: &'a Point) {
        self.points.push(point);
    }

    /// Clears the cluster
    pub fn clear(&mut self) {
        self.points.clear();
    }

    /// Calculates center of gravity of the [`Cluster`]
    pub fn calc_center(&mut self) -> f64 {
        let mut sum = self
            .points
            .iter()
            .fold(Point::zero(), |acc, &p| acc + p.to_owned());
        sum.div(self.points.len() as f64);

        let dist = sum.dist(&self.centroid);
        self.centroid = sum;

        dist
    }
}

impl<'a> Display for Cluster<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Centroid: {}", self.centroid)?;
        let points = self
            .points
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "Points: {}", points)
    }
}
