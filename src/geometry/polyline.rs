use super::{Point, GeodesicLine};
use std::ops::{Index, IndexMut};
use nannou::app::{Draw};

pub struct GeodesicPolyline {
    pub points: Vec<Point>
}

impl Index<usize> for GeodesicPolyline {
    type Output = Point;

    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}

impl IndexMut<usize> for GeodesicPolyline {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.points[index]
    }
}

impl GeodesicPolyline {
    fn into_lines(&self) -> Vec<GeodesicLine> {
        let len = self.points.len();
        (0..self.points.len()).map(|i| GeodesicLine::new(&self.points[i], &self.points[(i + 1) % len])).collect()
    }

    pub fn draw<'a>(&self, draw: &'a Draw) {
        for line in self.into_lines() {
            line.draw(draw);
        }
    } 
}