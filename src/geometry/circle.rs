use super::{Point, DRAW_SCALE_FACTOR};
use nannou::app::{Draw};
use nannou::draw::Drawing;
use nannou::geom::scalar::Default;
use nannou::draw::primitive::Ellipse;

pub struct Radians(pub Default);
pub struct Degrees(pub Default);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct EuclideanCircle {
    pub origin: Point,
    pub radius: Default
}

impl EuclideanCircle {
    pub const fn c_infinity() -> EuclideanCircle {
        EuclideanCircle {
            origin: Point::origin(),
            radius: 1.0
        }
    }

    pub const fn new() -> EuclideanCircle {
        EuclideanCircle {
            origin: Point::origin(),
            radius: 0.0
        }
    }

    pub fn origin(mut self, origin: Point) -> EuclideanCircle {
        self.origin = origin;
        self
    }

    pub fn radius(mut self, radius: Default) -> EuclideanCircle {
        self.radius = radius;
        self
    }

    // https://stackoverflow.com/a/14096170
    pub fn point_at_angle_radians(&self, angle: Radians) -> Point {
        let x = self.radius * angle.0.cos();
        let y = self.radius * angle.0.sin();

        Point::new(self.origin.x + x, self.origin.y + y)
    }

    pub fn angle_at_point(&self, pt: Point) -> Default {
        (pt.y - self.origin.y).atan2(pt.x - self.origin.x)
    }

    // https://stackoverflow.com/q/4103405
    pub fn from_3_points(a: &Point, b: &Point, c: &Point) -> EuclideanCircle {
        let y_delta_a = b.y - a.y;
        let x_delta_a = b.x - a.x;
        let y_delta_b = c.y - b.y;
        let x_delta_b = c.x - b.x;

        let mut origin = Point::origin();

        let a_slope = y_delta_a / x_delta_a;
        let b_slope = y_delta_b / x_delta_b;

        origin.x = (a_slope * b_slope * (a.y - c.y) + b_slope * (a.x + b.x) - a_slope * (b.x + c.x)) / (2.0 * (b_slope - a_slope));
        origin.y = -1.0 * (origin.x - (a.x + b.x) / 2.0) / a_slope + (a.y + b.y) / 2.0;

        EuclideanCircle::new()
            .origin(origin)
            .radius(origin.distance_to(a))
    }

    pub fn draw<'a>(self, draw: &'a Draw) -> Drawing<'a, Ellipse> {
        draw.ellipse()
            .width(self.radius * 2.0 * DRAW_SCALE_FACTOR)
            .height(self.radius * 2.0 * DRAW_SCALE_FACTOR)
            .x_y(self.origin.x * DRAW_SCALE_FACTOR, self.origin.y * DRAW_SCALE_FACTOR)
    } 
}

pub struct GeodesicCircle {
    geodesic_center: Point,
    euclidean_center: Point,
    radius: f64
}

impl GeodesicCircle {

}