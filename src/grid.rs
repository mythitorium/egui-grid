use egui::{Pos2, Ui};
use crate::builder::PureCell;

/// A collection of grid cells. 
///
/// Each cell has a fixed size. 
/// Cells are represented in order, as they were created (**Note**: see [`GridBuilder::rows_as_columns`](crate::builder::GridBuilder::rows_as_columns)).
///
/// The cells of a nested grid will also be included in place of the cell that contained them
/// (the cell holding the grid will not be represented & the nested cells will take the cell's place in line).
///
pub struct Grid<'a, 'b> {
    ui: &'a mut Ui,
    cells: Vec<PureCell>,
    pointer: usize,
    bounds: &'b mut Pos2
}

impl Grid<'_, '_> {
    pub(crate) fn new<'a>(ui: &'a mut Ui, cells: Vec<PureCell>, bounds: &'a mut Pos2) -> Grid<'a, 'a> {
        Grid {
            ui: ui,
            cells: cells,
            pointer: 0,
            bounds: bounds
        }
    }

    /// Add contents to this cell
    pub fn cell(&mut self, add_contents: impl FnOnce(&mut Ui)) {
        if self.pointer > self.cells.len()-1 {
            panic!("Added more `cells` than were pre-allocated ({} pre-allocated)", self.cells.len());
        }

        let cell = &self.cells[self.pointer];
        let cell_rect = cell.rect();
        let cell_layout = cell.layout();

        if cell_rect.max.y > self.bounds.y { self.bounds.y = cell_rect.max.y; }
        if cell_rect.max.x > self.bounds.x { self.bounds.x = cell_rect.max.x; }

        let mut child_ui = self.ui.child_ui(cell_rect, cell_layout);
        if cell.clip() {
            let margin = egui::Vec2::splat(self.ui.visuals().clip_rect_margin);
            let margin = margin.min(0.5 * self.ui.spacing().item_spacing);
            let clip_rect = cell_rect.expand2(margin);
            child_ui.set_clip_rect(clip_rect.intersect(child_ui.clip_rect()));
        }
        add_contents(&mut child_ui);
        self.pointer += 1;
    }

    /// Populate this cell with nothing. It will still take up space in the grid, but will be empty.
    pub fn empty(&mut self) {
        if self.pointer > self.cells.len()-1 {
            panic!("Added more `cells` than were pre-allocated ({} pre-allocated)", self.cells.len());
        }

        let cell_rect = self.cells[self.pointer].rect();
        
        if cell_rect.max.y > self.bounds.y { self.bounds.y = cell_rect.max.y; }
        if cell_rect.max.x > self.bounds.x { self.bounds.x = cell_rect.max.x; }

        self.pointer += 1;
    }
}