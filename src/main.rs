mod geometry;

use geometry::{EuclideanCircle, Point, DRAW_SCALE_FACTOR, GeodesicLine, GeodesicPolyline, Polygon, PolygonType};
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run()
        ;
}

struct Model {
    unit_circle: EuclideanCircle,
    g_line: GeodesicLine,
    g_poly: GeodesicPolyline,
    g_poly_type: PolygonType
}

impl Model {
    pub fn next_poly_type(&mut self) {
        match self.g_poly_type {
            PolygonType::triangle => self.g_poly_type = PolygonType::square,
            PolygonType::square => self.g_poly_type = PolygonType::pentagon,
            PolygonType::pentagon => self.g_poly_type = PolygonType::hexagon,
            PolygonType::hexagon => self.g_poly_type = PolygonType::septagon,
            PolygonType::septagon => self.g_poly_type = PolygonType::octagon,
            PolygonType::octagon => self.g_poly_type = PolygonType::triangle
        };
    }

    pub fn regenerate_poly(&mut self, center: Point, radius: f32) {
        self.g_poly = Polygon::new(center, radius, self.g_poly_type);
    }
}

fn model(app: &App) -> Model {

    app.new_window().event(event).view(view).build().unwrap();

    let c1 = EuclideanCircle::c_infinity();
    let a = Point::new(0.5, 0.5);
    let b = Point::new(-0.5, 0.6);
    let line = GeodesicLine::new(&a, &b);
    let g_poly = Polygon::new(Point::origin(), 0.6, PolygonType::triangle);

    Model {
        unit_circle: c1,
        g_line: line,
        g_poly: g_poly,
        g_poly_type: PolygonType::triangle,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let target = Point::new(app.mouse.x / DRAW_SCALE_FACTOR, app.mouse.y / DRAW_SCALE_FACTOR);
    // model.g_line = GeodesicLine::new(&Point::new(0.5, 0.5), &target);
    model.regenerate_poly(target, 0.6);
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MouseReleased(_button) => {
            model.next_poly_type();
        },
        _ => ()
    };
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    model.unit_circle.draw(&draw).color(BLACK).stroke_weight(2.0);
    model.g_line.draw(&draw).color(STEELBLUE);
    model.g_poly.draw(&draw);

    draw.ellipse()
        .x_y(0.5 * DRAW_SCALE_FACTOR, 0.5 * DRAW_SCALE_FACTOR)
        .w_h(2.0, 2.0)
        .color(RED);
    draw.ellipse()
        .x_y(-0.5 * DRAW_SCALE_FACTOR, 0.6 * DRAW_SCALE_FACTOR)
        .w_h(2.0, 2.0)
        .color(RED);

    draw.to_frame(app, &frame).unwrap();
    }