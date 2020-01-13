use nannou::geom::scalar::Default;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: Default,
    pub y: Default
}

impl Point {
    pub const fn origin() -> Point {
        Point {
            x: 0.0,
            y: 0.0
        }
    }

    pub fn new(x: Default, y: Default) -> Point {
        Point {
            x: x,
            y: y
        }
    }

    pub fn distance_to(&self, other: &Point) -> Default {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }

    pub fn as_inversed(&self) -> Point {
        let inversion_dist = 1.0 / (self.x.powi(2) + self.y.powi(2));
        Point {
            x: self.x * inversion_dist,
            y: self.y * inversion_dist
        }
    }
}
