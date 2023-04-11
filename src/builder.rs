use egui::{Rect, Pos2, Vec2, Margin, Align, Ui, Sense, Response, Layout};
use egui_extras::Size;
use crate::{
    sizing::Sizing,
    grid::*
};

/// Builder for creating a new [`Grid`].
///
/// Used to create grid-based layouts. Uses egui_extra's [`Size`](https://docs.rs/egui_extras/latest/egui_extras/enum.Size.html) for specificizing the space taken up by rows & cells.
///
/// In contrast to normal egui behavior, grid cells do not grow with its children!
///
/// Allocate new rows using [`Self::new_row`], with the size given being what the row's cells will inherit.
/// Then populate the row with cells using [`Self::cell`] or [`Self::cells`], each cell having it's own horizontal size and inheriting the size of the row it's being placed in.
/// Since cells do not wrap, [`Self::new_row`] can be called again to allocate a new row which can be populated with more cells.
///
/// Build the grid using [`Self::show`], and add it's contents to the ui using [`Grid::cell`].
/// Will panic if the number of cells called to display is more than the amount pre-allocated.
///
/// One can customize how the grid gets built and how the cells are displayed using [`Self::rows_as_columns`], [`Self::spacing`], [`Self::align`], among others.
///
/// ## Exmaple
/// ```
/// use egui_grid::GridBuilder;
/// use egui_extras::Size;
///
/// GridBuilder::new()
///     // Allocate a new row
///     .new_row(Size::exact(200.0))
///     // Give this row a couple cells
///     .cell(Size::exact(85.0))
///     .cell(Size::remainder())
///     // Allocate another row
///     .new_row(Size::remainder())
///     // Batch method, allocate multiple cells at once
///     .cells(Size::remainder(), 3)
///     .show(ui, |mut grid| {
///         // Cells are represented as they were allocated
///         grid.cell(|ui| {
///             ui.label("Top row, left cell");
///         });
///         grid.cell(|ui| {
///             ui.label("Top row, right cell");
///         });
///         grid.cell(|ui| {
///             ui.label("Bottom row, left cell");
///         });
///         grid.empty();
///         grid.cell(|ui| {
///             ui.label("Bottom row, right cell");
///         });
///     });
/// ```
#[derive(Clone)]
pub struct GridBuilder {
    units: Vec<Row>,
    spacing: Vec2,
    row_as_col: bool,
    creation_cache: Vec<(usize, usize)>
}

impl GridBuilder {
    /// Create new grid builder.
    pub fn new() -> GridBuilder {
        GridBuilder {
            units: Vec::new(),
            spacing: Vec2::ZERO,
            row_as_col: false,
            creation_cache: Vec::new()
        }
    }

    /// Set cell spacing. By default spacing is 0 on both axis.
    /// Spacing will not effect the spacing of any nested grids.
    pub fn spacing(mut self, width: f32, height: f32) -> Self {
        self.spacing = Vec2 { x: width, y: height };
        self
    }

    /// Set cell spacing using a [`Vec2`](https://docs.rs/egui/latest/egui/struct.Vec2.html).
    pub fn spacing_vec2(mut self, vec: Vec2) -> Self {
        self.spacing = vec;
        self
    }

    /// Inherit spacing from a [`Ui`](https://docs.rs/egui/latest/egui/struct.Ui.html).
    pub fn inherit_spacing(mut self, ui: &mut Ui) -> Self {
        self.spacing = ui.style_mut().spacing.item_spacing.clone();
        self
    }

    /// Allocate a new row with given [`Size`](https://docs.rs/egui_extras/latest/egui_extras/enum.Size.html). Rows are represented top-to-bottom.
    pub fn new_row(mut self, size: Size) -> Self {
        self.units.push(Row::new(size, Align::Min));
        self
    }

    /// Allocate a new row with a given [`Size`](https://docs.rs/egui_extras/latest/egui_extras/enum.Size.html), with a custom Align for any cells it'll have.
    /// An align will not effect how cells are allocated, just which side of the row they'll align with, in the case they don't fill the entirety of a row.
    ///
    /// Default align for [`Self::new_row`] is [`Align::Min`](https://docs.rs/egui/latest/egui/enum.Align.html).
    pub fn new_row_align(mut self, size: Size, align: Align) -> Self {
        self.units.push(Row::new(size, align));
        self
    }

    /// Set the cell [`Align`](https://docs.rs/egui/latest/egui/enum.Align.html) of the most recently allocated row.
    /// This will work regardless if the row has been populated with cells or not.
    ///
    /// Does nothing unless at least one row has been allocated.
    pub fn align(mut self, align: Align) -> Self {
        let len = self.units.len();
        if len > 0 {
            self.units[len-1].align(align);
        }
        self
    }

    /// Add a cell to the most recently allocated row. Cells are represented left-to-right.
    /// Does nothing unless at least one row has been allocated.
    pub fn cell(mut self, size: Size) -> Self {
        self.add_cells(size, 1, Margin::same(0.)); self
    }

    /// Add multiple cells all with the same size to the most recently allocated row. Cells are represented left-to-right.
    /// Does nothing unless at least one row has been allocated.
    pub fn cells(mut self, size: Size, amount: i32) -> Self {
        self.add_cells(size, amount, Margin::same(0.)); self
    }

    // Old code
    // ----------------------------------------
    // Add a cell to the most recently allocated row, with a custom [`Margin`](https://docs.rs/egui/latest/egui/style/struct.Margin.html). 
    // Cells allocated using [`Self::cell`] or [`Self::cells`] have a default margin of 0 on all sides.
    //pub fn m_cell(mut self, size: Size, margin: Margin) -> Self {
    //    self.add_cells(size, 1, margin); self
    //}

    // Add multiple cells all with same [`Size`](https://docs.rs/egui_extras/latest/egui_extras/enum.Size.html) to the most recently allocated row, 
    // all with the same custom [`Margin`](https://docs.rs/egui/latest/egui/style/struct.Margin.html). 
    //pub fn m_cells(mut self, size: Size, amount: i32, margin: Margin) -> Self {
    //    self.add_cells(size, amount, margin); self
    //}
    // ----------------------------------------

    /// Give the most recently allocated cells a custom [`Margin`](https://docs.rs/egui/latest/egui/style/struct.Margin.html).
    /// Can be used after [`Self::cells`] to give multiple cells a margin at once.
    ///
    /// ## Example
    /// ```
    /// let grid = GridBuilder::new()
    ///     .new_row(Size::remainder())
    ///     // This 'cell' will have a custom margin
    ///     .cell(Size::exact(100.0)) .with_margin(Margin::same(10.0))
    ///     // All 4 cells allocated here will have matching margins
    ///     .cells(Size::exact(50.0), 4) .with_margin(Margin::same(6.0))
    ///     // This will overwrite the last 4 allocated, but not the first one
    ///     .with_margin(Margin::symmetric(6.0, 4.0));
    /// ```
    pub fn with_margin(mut self, margin: Margin) -> Self {
        // the creation cache can only be bigger than one if cells and therefor rows have already been created.
        if self.creation_cache.len() > 0 {
            for item in self.creation_cache.iter() {
                self.units[item.0].cells[item.1].edit_margin(margin);
            }
        }
        self
    }

    /// Give the most recently allocated cells a custom [`Layout`](https://docs.rs/egui/latest/egui/struct.Layout.html).
    /// 
    /// Behavior matches [`Self::with_margin`].
    pub fn with_layout(mut self, layout: Layout) -> Self {
        if self.creation_cache.len() > 0 {
            for item in self.creation_cache.iter() {
                self.units[item.0].cells[item.1].edit_layout(layout);
            }
        }
        self
    }

    /// Nest a grid at the most recently allocated cell.
    /// Does nothing in the absence of any rows or the most recently allocated row being absent of any cells.
    ///
    /// ## Example
    /// ```
    /// // A grid of 4 cells which all take up equal space
    /// let nested_grid = GridBuilder::new()
    ///     .new_row(Size::remainder()).cells(Size::remainder(), 2)
    ///     .new_row(Size::remainder()).cells(Size::remainder(), 2);
    ///
    /// let parent_grid = GridBuilder::new()
    ///     .new_row(Size::remainder())
    ///     .cells(Size::remainder(), 2)
    ///     // Despite being called after a batch cell allocation, 
    ///     // ONLY the last cell will have the grid nested
    ///     .nest(nested_grid)
    ///     .show(ui, |mut grid| {
    ///         grid.cell(|ui| {
    ///             ui.label("Left cell");
    ///         });
    ///         grid.cell(|ui| {
    ///             ui.label("Nested cell top-left");
    ///         });
    ///         grid.cell(|ui| {
    ///             ui.label("Nested cell top-right");
    ///         });
    ///         grid.cell(|ui| {
    ///             ui.label("Nested cell bottom-left");
    ///         });
    ///         grid.cell(|ui| {
    ///             ui.label("Nested cell bottom-right");
    ///         });
    ///     });
    /// ```
    pub fn nest(mut self, grid: GridBuilder) -> Self {
        let len = self.units.len();
        if len > 0 {
            // get last
            let cell_len = self.units[len-1].cells.len();
            if cell_len > 0 {
                self.units[len-1].cells[cell_len-1].nest(grid);
            }
        }
        self
    }

    /// Nest a grid at a given row in a given cell. Nothing will happen if a cell doesn't exist at the given coordinates.
    pub fn nest_at(mut self, row: i32, cell: i32, grid: GridBuilder) -> Self {
        let u_row = row as usize;
        let u_cell = cell as usize;
        if self.units.get(u_row).is_some() {
            if self.units[u_row].cells.get(u_cell).is_some() {
                self.units[u_row].cells[u_cell].nest(grid);
            }
        }
        self
    }

    /// Rows are positioned top-to-bottom spanning horizontally, and cells within rows left-to-right.
    ///
    /// The cells of a nested grid will be represented in place of the cell that held it.
    pub fn show(self, ui: &mut Ui, grid: impl FnOnce(Grid)) -> Response {
        let allocated_space = ui.available_rect_before_wrap();
        let rects = self.as_rects(allocated_space);
        let layouts = self.get_all_layouts();
        let mut bounds = Pos2::new(0., 0.);

        grid(Grid::new(ui, rects, layouts, &mut bounds));

        ui.allocate_rect(Rect{ min: allocated_space.min, max: bounds}, Sense::hover())
    }

    /// Setting to `true` will result in rows acting as columns when [`Self::show`] is called (with the cells within being represented top-to-bottom instead of left-to-right).
    /// This behavior will remain consistent even if this grid becomes nested within another.
    ///
    /// Calling this method will ***NOT***
    /// - Propagate to nested grids.
    /// - Affect the way cell spacing is applied. Rows will still use the 'height' component of item spacing, and width for cells.
    /// - Affect the grid creation process in any way. Rows will still be top-to-bottom and cells left-to-right until [`Self::show`] is called.
    /// - Affect the way margins are applied to cells.
    ///
    /// Default: `false`.
    pub fn rows_as_columns(mut self, vertical: bool) -> Self {
        self.row_as_col = vertical;
        self
    }

    /// General purpose method for adding cells
    fn add_cells(&mut self, size: Size, amount: i32, margin: Margin) {
        let len = self.units.len();
        if len > 0 {
            let cel_len = self.units[len-1].cells.len();
            self.creation_cache = Vec::new();
            for c in 1..=amount {
                self.units[len-1].cells.push(Cell::new(size, margin, Layout::default()));
                self.creation_cache.push((len-1, cel_len+(c as usize)-1));
            }
        }
    }

    /// Turns the input rect into many rects, representing the cells within the grid
    fn as_rects(&self, whole_rect: Rect) -> Vec<Rect> {
        let mut rects_final: Vec<Rect> = Vec::new();
        
        let row_lengths = row_set_as_f32(&self.units, &self.spacing.y, &whole_rect.height());

        let mut pointer2d = Pos2::new(whole_rect.min.x,whole_rect.min.y);
        let mut row_index = 0;
        for row_len in row_lengths {
            // Improve code readability
            let row = &self.units[row_index];

            // Get cell sizes
            let cell_lengths = cell_set_as_f32(&row.cells, &self.spacing.x, &whole_rect.width());

            // sum of the lengths + spacing
            let mut length_sum = -self.spacing.x; // minus spacing to counter balance the extra spacing added at the end of the for loop
            for length in cell_lengths.iter() { length_sum += length + self.spacing.x; }
            // apply align offset
            let grand_offset: f32 = { 
                match &row.align {
                    Align::Min => { 0. },
                    Align::Center => { (whole_rect.width() - length_sum) * 0.5 },
                    Align::Max => { whole_rect.width() - length_sum }
                }
            };
            pointer2d.x += grand_offset;

            let mut index = 0;
            for cell_len in cell_lengths {
                // Build the rect
                let mut rect = Rect {
                    min: pointer2d.clone(),
                    max: Pos2::new(pointer2d.x + cell_len, pointer2d.y + row_len)
                };

                // Apply verticality
                if self.row_as_col { rect = reflect(rect, whole_rect.min); }

                // Apply margins
                let margin = &(row.cells[index].margin);
                rect.min.x += margin.left; rect.min.y += margin.top; 
                rect.max.x -= margin.right; rect.max.y -= margin.bottom; 

                // Check and handle nested grids
                match &row.cells[index].group {
                    Option::Some(grid) => { rects_final.extend(grid.as_rects(rect)); },
                    Option::None => { rects_final.push(rect); }
                }

                // Update indexes
                pointer2d.x += cell_len + self.spacing.x;
                index += 1;
            }

            // Update indexes
            pointer2d.x = whole_rect.min.x.clone();
            pointer2d.y += row_len + self.spacing.y;
            row_index += 1;
        }

        rects_final
    }

    pub(crate) fn get_all_layouts(&self) -> Vec<Layout> {
        let mut layouts: Vec<Layout> = Vec::new();
        for row in self.units.iter() {
            for cell in row.cells.iter() {
                layouts.extend(cell.layout());
            }
        }
        layouts
    }
}

// Represents a row of cells. Useless on it's own, must be given to a GridBuilder. 
#[derive(Clone)]
struct Row {
    size: Size,
    cells: Vec<Cell>,
    align: Align
}

impl Row {
    pub fn new(size: Size, align: Align) -> Row {
        Row { size: size, cells: Vec::new(), align: align }
    }

    fn align(&mut self, align: Align) {
        self.align = align;
    }
}

// Internal struct for the grid builder to keep track of the layout details
#[derive(Clone)]
struct Cell {
    size: Size,
    margin: Margin,
    layout: Layout,
    pub group: Option<GridBuilder>,
}

impl Cell {
    pub fn new(size: Size, margin: Margin, layout: Layout) -> Cell {
        Cell { size: size, group: None, margin: margin, layout: layout }
    }

    // Nest a grid within this cell
    pub fn nest(&mut self, grid: GridBuilder) {
        self.group = Some(grid);
    }

    pub fn edit_margin(&mut self, margin: Margin) {
        self.margin = margin;
    }

    pub fn edit_layout(&mut self, layout: Layout) {
        self.layout = layout;
    }

    // Get layouts, accounting for nested grids
    pub fn layout(&self) -> Vec<Layout> {
        match &self.group {
            Option::Some(grid) => { grid.get_all_layouts() },
            Option::None => { vec![self.layout] }
        }
    }
}

// Moved code to functions so the to rect method doesn't look as cluttered
fn row_set_as_f32(rows: &Vec<Row>, spacing: &f32, whole: &f32) -> Vec<f32> {
    let mut row_sizes = Vec::new();
    for row in rows.iter() { row_sizes.push(row.size.clone()); }
    return Sizing::from(row_sizes).to_lengths(*whole, *spacing);
}

fn cell_set_as_f32(cells: &Vec<Cell>, spacing: &f32, whole: &f32) -> Vec<f32> {
    let mut row_sizes = Vec::new();
    for row in cells.iter() { row_sizes.push(row.size.clone()); }
    return Sizing::from(row_sizes).to_lengths(*whole, *spacing);
}

// This effectively reflects the rectangle on a line of symmetry where y=-x 
// input for the rect being reflected, focal for the offset to the center of symmetry
fn reflect(input: Rect, focal: Pos2) -> Rect {
    let offset = input.min - focal;
    Rect {
        min: Pos2::new(offset.y + focal.x, offset.x + focal.y),
        max: Pos2::new(offset.y + focal.x + input.height(), offset.x + focal.y + input.width())
    }
}