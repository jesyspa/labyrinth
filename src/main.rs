#![allow(dead_code)]
extern crate enum_map_derive;

mod geometry;
mod text_display;
mod view_geometry;
mod views;

use crate::geometry::Direction;
use crate::text_display::TextSurface;
use crate::views::{LabyrinthView, RoomView};

const LABYRINTH_WIDTH: usize = 6;
const LABYRINTH_HEIGHT: usize = 4;

fn main() {
    let mut v: Vec<RoomView> = Vec::new();
    for _ in 0..(LABYRINTH_WIDTH * LABYRINTH_HEIGHT) {
        v.push(RoomView::new(&[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]));
    }
    let lab = LabyrinthView::new(LABYRINTH_WIDTH, LABYRINTH_HEIGHT, v);
    let surface = TextSurface::create_and_draw(&lab);
    print!("{}", surface);
}
