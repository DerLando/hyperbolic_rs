mod geometry;

use geometry::{EuclideanCircle, Point, DRAW_SCALE_FACTOR, GeodesicLine};
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run()
        ;
}

struct Model {
    unit_circle: EuclideanCircle,
    g_line: GeodesicLine
}

fn model(_app: &App) -> Model {
    let c1 = EuclideanCircle::c_infinity();
    let a = Point::new(0.5, 0.5);
    let b = Point::new(-0.5, 0.6);
    let line = GeodesicLine::new(&a, &b);

    Model {
        unit_circle: c1,
        g_line: line
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    model.unit_circle.draw(&draw).color(BLACK).stroke_weight(2.0);
    model.g_line.draw(&draw).color(STEELBLUE);

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