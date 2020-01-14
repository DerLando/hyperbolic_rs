use super::{Point, GeodesicPolyline, EuclideanCircle, Radians};
use nannou::geom::scalar::Default;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub enum PolygonType {
    triangle = 3,
    square = 4,
    pentagon = 5,
    hexagon = 6,
    septagon = 7,
    octagon = 8
}

pub struct Polygon {
}

impl Polygon {
    pub fn new(center: Point, radius: Default, poly_type: PolygonType) -> GeodesicPolyline {
        let helper = EuclideanCircle::new().origin(center).radius(radius);
        let inner_angle = Polygon::inner_angle(poly_type);
        let points = (0..(poly_type as isize)).map(|i| helper.point_at_angle_radians(Radians{0: (inner_angle.0 * i as f32)}));

        GeodesicPolyline {
            points: points.collect()
        }
    }

    fn inner_angle(poly_type: PolygonType) -> Radians {
        let full_angle = 2.0 * PI;
        Radians{0: full_angle / poly_type as isize as Default}
    }
}