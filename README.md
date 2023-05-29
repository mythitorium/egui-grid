# egui_grid

[![Latest version](https://img.shields.io/crates/v/egui_grid.svg)](https://crates.io/crates/egui_grid)
[![Documentation](https://docs.rs/egui_grid/badge.svg)](https://docs.rs/egui_grid)

Create dynamic grid layouts for [`egui`](https://github.com/emilk/egui).

Grids are flexible, easy to create, with behavior similar to egui_extra's strip creation. They're compact and allow for more complex layouts using less indentation,
with functionalities like nesting grids and aligning cells within a row.

## Installing

Add this to your `Cargo.toml`:

```toml
[dependencies]
egui_grid = "0.1.0"
```

## Example

``` rust
// Quick example, by no means does it fully demo
// how flexible building grids can be.
use egui_grid::GridBuilder;
use egui_extras::Size;

GridBuilder::new()
    // Allocate a new row
    .new_row(Size::exact(200.0))
    // Give this row a couple cells
    .cell(Size::exact(85.0))
    .cell(Size::remainder())
    // Allocate another row
    .new_row(Size::remainder())
    // Batch method, allocate multiple cells at once
    .cells(Size::remainder(), 3)
    .show(ui, |mut grid| {
        // Cells are represented as they were allocated
        grid.cell(|ui| {
            ui.label("Top row, left cell");
        });
        grid.cell(|ui| {
            ui.label("Top row, right cell");
        });
        grid.cell(|ui| {
            ui.label("Bottom row, left cell");
        });
        grid.empty();
        grid.cell(|ui| {
            ui.label("Bottom row, right cell");
        });
    });
```

## Usage

Check the [docs](https://docs.rs/egui_grid/latest/egui_grid/) for details and more info on usage.

## Issues

Currently this is feature complete. Bug fixes, optimizations, and staying up to date with [egui](https://github.com/emilk/egui) releases are the only ways this crate will be expanding for the foreseeable future.

While you are free to open an issue, contacting me (Mythitorium#4918) on the [egui discord server](https://discord.gg/wdkZkEdXks) might be more worth your time.

