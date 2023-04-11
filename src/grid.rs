use egui::{Rect, Pos2, Ui, Layout};

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
    cells: Vec<Rect>,
    layouts: Vec<Layout>,
    pointer: usize,
    bounds: &'b mut Pos2
}

impl Grid<'_, '_> {
    pub(crate) fn new<'a>(ui: &'a mut Ui, cells: Vec<Rect>, layouts: Vec<Layout>, bounds: &'a mut Pos2) -> Grid<'a, 'a> {
        Grid {
            ui: ui,
            cells: cells,
            layouts: layouts,
            pointer: 0,
            bounds: bounds
        }
    }

    /// Add contents to this cell
    pub fn cell(&mut self, add_contents: impl FnOnce(&mut Ui)) {
        let cell_rect = self.cells[self.pointer];
        let cell_layout = self.layouts[self.pointer];

        if cell_rect.max.y > self.bounds.y { self.bounds.y = cell_rect.max.y; }
        if cell_rect.max.x > self.bounds.x { self.bounds.x = cell_rect.max.x; }

        add_contents(&mut self.ui.child_ui(cell_rect, cell_layout));
        self.pointer += 1;
    }

    /// Populate this cell with nothing. It will still take up space in the grid, but will be empty.
    pub fn empty(&mut self) {
        let cell_rect = self.cells[self.pointer];
        
        if cell_rect.max.y > self.bounds.y { self.bounds.y = cell_rect.max.y; }
        if cell_rect.max.x > self.bounds.x { self.bounds.x = cell_rect.max.x; }

        self.pointer += 1;
    }
}