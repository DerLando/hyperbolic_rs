use super::{Point, EuclideanCircle, Radians, DRAW_SCALE_FACTOR};
use nannou::app::{Draw};
use nannou::draw::Drawing;
use nannou::geom::scalar::Default;
use nannou::draw::primitive::path::Path;
use nannou::geom::pt2;

pub struct EuclideanLine {
    from: Point,
    to: Point,
}

impl EuclideanLine {
    pub const fn new() -> EuclideanLine {
        EuclideanLine {
            from: Point::origin(),
            to: Point::origin(),
        }
    }

    pub fn from(&mut self, from: Point) -> &mut EuclideanLine {
        self.from = from;
        self
    }

    pub fn to(&mut self, to: Point) -> &mut EuclideanLine {
        self.to = to;
        self
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GeodesicLine {
    from: Point,
    to: Point,
    circle: EuclideanCircle,
}

impl GeodesicLine {
    pub fn new(from: &Point, to: &Point) -> GeodesicLine {
        let helper = EuclideanCircle::from_3_points(from, to, &from.as_inversed());
        GeodesicLine {
            from: *from,
            to: *to,
            circle: helper,
        }
    }

    pub fn draw<'a>(self, draw: &'a Draw) -> Drawing<'a, Path> {
        let start_angle = self.circle.angle_at_point(self.from);
        let end_angle = self.circle.angle_at_point(self.to);

        let start: f32;
        let end: f32;
        let fac: f32;
        match start_angle < end_angle {
            true => {
                start = start_angle;
                end = end_angle;
            },
            false => {
                start = end_angle;
                end = start_angle;
            }
        };

        let angle_step = (start - end).abs() / 10.0;

        let points = (0..11).map(|i| {
            let pt = self.circle.point_at_angle_radians(Radians{0:start + angle_step * (i as f32)});
            (pt2(pt.x * DRAW_SCALE_FACTOR, pt.y * DRAW_SCALE_FACTOR))
        });

        draw.polyline()
            .stroke_weight(2.0)
            .points(points)
    }
}