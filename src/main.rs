#![allow(dead_code)]
extern crate enum_map_derive;

mod geometry;
mod labyrinth;
mod text_display;
mod views;

use crate::labyrinth::FlatLabyrinth;
use crate::labyrinth::Labyrinth;
use crate::text_display::TextSurface;

const LABYRINTH_WIDTH: usize = 6;
const LABYRINTH_HEIGHT: usize = 4;

fn main() {
    let lab = FlatLabyrinth::<(), ()>::new(LABYRINTH_WIDTH, LABYRINTH_WIDTH, false);
    let lab_view = lab.view();
    let surface = TextSurface::create_and_draw(&lab_view);
    print!("{}", surface);
}
