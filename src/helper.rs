use crate::{
    builder::{Cell, Row},
    sizing::Sizing,
};
use egui::{Pos2, Rect, Vec2};

// Moved code to functions so the into_real_cells method doesn't look as cluttered
pub(crate) fn row_set_as_f32(rows: &[Row], spacing: &f32, whole: &f32) -> Vec<f32> {
    let mut row_sizes = Vec::new();
    for row in rows.iter() {
        row_sizes.push(row.size);
    }
    Sizing::from(row_sizes).to_lengths(*whole, *spacing)
}

pub(crate) fn cell_set_as_f32(cells: &[Cell], spacing: &f32, whole: &f32) -> Vec<f32> {
    let mut row_sizes = Vec::new();
    for row in cells.iter() {
        row_sizes.push(row.size);
    }
    Sizing::from(row_sizes).to_lengths(*whole, *spacing)
}

// This effectively reflects the rectangle on a line of symmetry where y=-x
// input for the rect being reflected, focal for the offset to the center of symmetry
pub(crate) fn reflect(input: Rect, focal: Pos2) -> Rect {
    let offset = input.min - focal;
    Rect {
        min: Pos2::new(offset.y + focal.x, offset.x + focal.y),
        max: Pos2::new(
            offset.y + focal.x + input.height(),
            offset.x + focal.y + input.width(),
        ),
    }
}

pub(crate) fn swap_spacing(spacing: Vec2, swap: bool) -> Vec2 {
    if swap {
        Vec2 {
            x: spacing.y,
            y: spacing.x,
        }
    } else {
        spacing
    }
}
