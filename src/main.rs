#![allow(dead_code)]
extern crate enum_map_derive;
extern crate rand;

mod generator;
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
    let mut lab = FlatLabyrinth::<(), ()>::new(LABYRINTH_WIDTH, LABYRINTH_WIDTH, true);
    generator::random_walk(&mut lab);
    let lab_view = lab.view();
    let surface = TextSurface::create_and_draw(&lab_view);
    print!("{}", surface);
}
