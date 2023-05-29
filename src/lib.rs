//! This is my attempt at taking a stab at the *'egui complex layout'* problem,
//! Built to provide dynamic grid layouts for [egui](https://github.com/emilk/egui).
//!
//! Relies on structs and enums from both [`egui`](https://github.com/emilk/egui) and [`egui_extras`](https://crates.io/crates/egui_extras).
//! 
//! This crate includes 2 items, [`GridBuilder`] and [`Grid`], which are used to create grids
//! with behavior similar to the StripBuilder found in eui_extras, though being much more compact and with additional features.
//!
mod sizing;
mod grid;
mod helper;
mod builder;

pub use crate::grid::Grid;
pub use crate::builder::GridBuilder;
