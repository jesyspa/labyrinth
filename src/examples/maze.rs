//! Display a simple example maze.

#![deny(missing_docs)]

use labyrinth::labyrinth::{FlatLabyrinth, Labyrinth};
use labyrinth::text_display::TextSurface;
use labyrinth::generator::random_walk;

const LABYRINTH_WIDTH: usize = 6;
const LABYRINTH_HEIGHT: usize = 4;

fn main() {
    let mut lab = FlatLabyrinth::<(), ()>::new(LABYRINTH_WIDTH, LABYRINTH_HEIGHT, true);
    random_walk(&mut lab);
    let lab_view = lab.view();
    let surface = TextSurface::create_and_draw(&lab_view);
    print!("{}", surface);
}
