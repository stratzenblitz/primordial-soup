use nannou::prelude::*;
use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Coord {
    _x: i16,
    _y: i16
}

struct Thing {
    _pos: Coord,
    _vel: Coord
}

struct Model {
    _window: window::Id,
    _things: Vec<Thing>,
    _size: i16
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let _size = 200;

    let mut _things = Vec::new();

    let position_range = Uniform::from(0.._size);
    let velocity_range = Uniform::from(-1..1);
    let mut rng = rand::thread_rng();


    for i in 0..5000 {
        let pos = Coord {
            _x: position_range.sample(&mut rng),
            _y: position_range.sample(&mut rng)
        };
        let vel = Coord {
            _x: velocity_range.sample(&mut rng),
            _y: velocity_range.sample(&mut rng)
        };
        _things.push(Thing { _pos: pos, _vel: vel});
    }

    Model { _window, _things, _size}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    for thing in _model._things.iter_mut() {
        thing._pos._x += thing._vel._x;
        thing._pos._y += thing._vel._y;

        thing._pos._x = (thing._pos._x + _model._size) % _model._size;
        thing._pos._y = (thing._pos._y + _model._size) % _model._size;
    }

    let mut collision_indices = Vec::new();

    for (i,thing_i) in _model._things.iter().enumerate() {
        for (j,thing_j) in _model._things[i+1.._model._things.len()].iter().enumerate() {
            if thing_i._pos._x == thing_j._pos._x && thing_i._pos._y == thing_j._pos._y {
                collision_indices.push((i,j));
            }
        }
    }

    for collision_pair in collision_indices.iter() {
        _model._things[collision_pair.1]._vel._x -= 1;
        _model._things[collision_pair.1]._vel._y = 0;
    } 

    while !collision_indices.is_empty() {
        let collision_pair = collision_indices.pop().unwrap();
        _model._things.remove(collision_pair.0);
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let scale = 5.0;
    let full_size = _model._size as f32;
    let half_size = _model._size as f32 / 2.0;
    draw.background().color(PLUM);
    draw.rect()
        .w_h(scale*full_size, scale*full_size)
        .color(BLACK);
    for thing in _model._things.iter() {
        let shift_x = scale*(thing._pos._x as f32 - half_size);
        let shift_y = scale*(thing._pos._y as f32 - half_size);
        draw.ellipse()
            .color(STEELBLUE)
            .x_y(shift_x, shift_y)
            .w_h(5.0, 5.0);
    }
    draw.to_frame(app, &frame).unwrap();
}