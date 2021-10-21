//! A library for generating and analysing labyrinths.
//! 
//! This is meant as a sandbox to play around with approaching labyrinths
//! more algorithmically.
//! 
//! Eventual goals:
//! * Visualise the labyrinth using a GUI lib.
//! * Generate labyrinths of various geometries.
//! * Analyse congregation points in labyrinths.
//! 
//! Stretch goal: run bots that try to solve the labyrinth.

pub mod generator;
mod geometry;
pub mod labyrinth;
pub mod text_display;
mod views;