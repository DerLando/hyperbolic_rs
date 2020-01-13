use super::Point;
use std::ops::{Index, IndexMut};

pub struct Polyline {
    points: Vec<Point>
}

impl Index<usize> for Polyline {
    type Output = Point;

    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}

impl IndexMut<usize> for Polyline {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.points[index]
    }
}