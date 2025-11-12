use serde::{Deserialize, Serialize};

/// Cell type determines how the cell is evaluated and displayed
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CellType {
    /// Regular data cell
    Data,
    /// Cell containing an operation/formula
    Code,
    /// Variable assignment
    VarAssign,
    /// Variable read
    VarRead,
    /// Horizontal view layout
    ViewHorizontal,
    /// Vertical view layout
    ViewVertical,
}

impl Default for CellType {
    fn default() -> Self {
        CellType::Data
    }
}

/// Text styling flags
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct StyleBits(pub u32);

impl StyleBits {
    pub const BOLD: u32 = 1 << 0;
    pub const ITALIC: u32 = 1 << 1;
    pub const FIXED: u32 = 1 << 2;
    pub const UNDERLINE: u32 = 1 << 3;
    pub const STRIKETHRU: u32 = 1 << 4;

    pub fn new() -> Self {
        StyleBits(0)
    }

    pub fn is_bold(&self) -> bool {
        self.0 & Self::BOLD != 0
    }

    pub fn is_italic(&self) -> bool {
        self.0 & Self::ITALIC != 0
    }

    pub fn is_fixed(&self) -> bool {
        self.0 & Self::FIXED != 0
    }

    pub fn is_underline(&self) -> bool {
        self.0 & Self::UNDERLINE != 0
    }

    pub fn is_strikethru(&self) -> bool {
        self.0 & Self::STRIKETHRU != 0
    }

    pub fn set_bold(&mut self, enabled: bool) {
        if enabled {
            self.0 |= Self::BOLD;
        } else {
            self.0 &= !Self::BOLD;
        }
    }

    pub fn set_italic(&mut self, enabled: bool) {
        if enabled {
            self.0 |= Self::ITALIC;
        } else {
            self.0 &= !Self::ITALIC;
        }
    }
}

impl Default for StyleBits {
    fn default() -> Self {
        StyleBits::new()
    }
}

/// Color representation (RGBA)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const DEFAULT_CELL: Color = Color::WHITE;
    pub const DEFAULT_TEXT: Color = Color::BLACK;
    pub const DEFAULT_BORDER: Color = Color::WHITE;

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }
}

impl Default for Color {
    fn default() -> Self {
        // Default to white for cell/border colors
        // Text color should use default_text_color() function instead
        Color::WHITE
    }
}

/// Grid layout mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GridLayout {
    /// Horizontal layout
    Horizontal,
    /// Vertical layout (default)
    Vertical,
}

impl Default for GridLayout {
    fn default() -> Self {
        GridLayout::Vertical
    }
}

/// A single TreeSheets cell containing text and zero or more child cells.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Cell {
    /// Textual content of the cell. TreeSheets allows free-form text, so this
    /// representation keeps the payload as an owned string.
    pub text: String,

    /// Child cells that form the hierarchical grid within this cell.
    #[serde(default)]
    pub children: Vec<Cell>,

    /// Cell type (data, code, variable, view)
    #[serde(default)]
    pub cell_type: CellType,

    /// Text styling flags
    #[serde(default)]
    pub style: StyleBits,

    /// Relative text size adjustment
    #[serde(default)]
    pub rel_size: i32,

    /// Cell background color
    #[serde(default = "default_cell_color")]
    pub cell_color: Color,

    /// Text color
    #[serde(default = "default_text_color")]
    pub text_color: Color,

    /// Whether the grid is folded/collapsed
    #[serde(default)]
    pub folded: bool,

    /// Grid layout orientation
    #[serde(default)]
    pub layout: GridLayout,

    /// Image data (base64 encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// Border color for grid
    #[serde(default = "default_border_color")]
    pub border_color: Color,
}

// Helper functions for serde defaults
fn default_cell_color() -> Color {
    Color::DEFAULT_CELL
}

fn default_text_color() -> Color {
    Color::DEFAULT_TEXT
}

fn default_border_color() -> Color {
    Color::DEFAULT_BORDER
}

impl Cell {
    /// Creates a new cell with the provided text and no children.
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self {
            text: text.into(),
            children: Vec::new(),
            cell_type: CellType::default(),
            style: StyleBits::default(),
            rel_size: 0,
            cell_color: Color::DEFAULT_CELL,
            text_color: Color::DEFAULT_TEXT,
            folded: false,
            layout: GridLayout::default(),
            image: None,
            border_color: Color::DEFAULT_BORDER,
        }
    }

    /// Creates a cell with custom styling
    pub fn with_style<T: Into<String>>(text: T, style: StyleBits, color: Color) -> Self {
        let mut cell = Self::new(text);
        cell.style = style;
        cell.text_color = color;
        cell
    }

    /// Creates a cell with a specific type
    pub fn with_type<T: Into<String>>(text: T, cell_type: CellType) -> Self {
        let mut cell = Self::new(text);
        cell.cell_type = cell_type;
        cell
    }

    /// Returns `true` when the cell does not contain any children.
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Returns the number of direct children stored in the cell.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Adds the provided child cell to the end of the current cell's children.
    pub fn add_child(&mut self, child: Cell) {
        self.children.push(child);
    }

    /// Provides depth-first traversal over the cell and all descendants.
    pub fn walk<F>(&self, depth: usize, f: &mut F)
    where
        F: FnMut(&Cell, usize),
    {
        f(self, depth);
        for child in &self.children {
            child.walk(depth + 1, f);
        }
    }

    /// Provides mutable depth-first traversal over the cell and all descendants.
    pub fn walk_mut<F>(&mut self, depth: usize, f: &mut F)
    where
        F: FnMut(&mut Cell, usize),
    {
        f(self, depth);
        for child in &mut self.children {
            child.walk_mut(depth + 1, f);
        }
    }

    /// Returns true if this cell has any styling applied
    pub fn has_styling(&self) -> bool {
        self.style.0 != 0
            || self.rel_size != 0
            || self.cell_color != Color::DEFAULT_CELL
            || self.text_color != Color::DEFAULT_TEXT
    }

    /// Returns true if this cell has content (text or children)
    pub fn has_content(&self) -> bool {
        !self.text.is_empty() || !self.children.is_empty()
    }

    /// Folds/unfolds the cell's grid
    pub fn toggle_fold(&mut self) {
        self.folded = !self.folded;
    }

    /// Sets the layout orientation
    pub fn set_layout(&mut self, layout: GridLayout) {
        self.layout = layout;
    }
}

/// Represents the complete hierarchical sheet starting from a root cell.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Sheet {
    /// Short description to show in window titles or exports.
    pub title: String,
    /// Root cell that stores the entire tree hierarchy.
    pub root: Cell,
}

impl Sheet {
    /// Creates a new sheet using the provided title and root cell.
    pub fn new<T: Into<String>>(title: T, root: Cell) -> Self {
        Self {
            title: title.into(),
            root,
        }
    }

    /// Builds an example sheet used for smoke testing and bootstrapping demos.
    pub fn sample() -> Self {
        let mut root = Cell::new("TreeSheets Rust Prototype");

        let mut left = Cell::new("Personal");
        left.add_child(Cell::new("Tasks"));
        left.add_child(Cell::new("Notes"));

        let mut right = Cell::new("Work");
        let mut project = Cell::new("TreeSheets RS");
        project.add_child(Cell::new("Implement sheet data model"));
        project.add_child(Cell::new("Design CLI workflows"));
        right.add_child(project);
        right.add_child(Cell::new("Retrospective"));

        root.add_child(left);
        root.add_child(right);

        Sheet::new("Sample Sheet", root)
    }

    /// Applies the provided closure to every cell in depth-first order.
    pub fn for_each_cell<F>(&self, mut f: F)
    where
        F: FnMut(&Cell, usize),
    {
        self.root.walk(0, &mut f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn cell_construction_and_traversal() {
        let mut cell = Cell::new("root");
        cell.add_child(Cell::new("child-1"));
        let mut second = Cell::new("child-2");
        second.add_child(Cell::new("leaf"));
        cell.add_child(second);

        assert!(!cell.is_leaf());
        assert_eq!(cell.child_count(), 2);

        let mut seen = Vec::new();
        cell.walk(0, &mut |node, depth| {
            seen.push((node.text.clone(), depth));
        });

        assert_eq!(
            seen,
            vec![
                ("root".to_string(), 0),
                ("child-1".to_string(), 1),
                ("child-2".to_string(), 1),
                ("leaf".to_string(), 2),
            ]
        );
    }

    #[test]
    fn sheet_sample_structure() {
        let sheet = Sheet::sample();
        assert_eq!(sheet.title, "Sample Sheet");

        let mut labels = Vec::new();
        sheet.for_each_cell(|cell, depth| {
            labels.push((cell.text.clone(), depth));
        });

        assert_eq!(labels[0], ("TreeSheets Rust Prototype".into(), 0));
        assert_eq!(labels.len(), 9);
    }

    #[test]
    fn cell_with_styling() {
        let mut style = StyleBits::new();
        style.set_bold(true);
        style.set_italic(true);

        let cell = Cell::with_style("Styled Text", style, Color::rgb(255, 0, 0));

        assert!(cell.style.is_bold());
        assert!(cell.style.is_italic());
        assert!(!cell.style.is_fixed());
        assert_eq!(cell.text_color.r, 255);
        assert_eq!(cell.text_color.g, 0);
        assert_eq!(cell.text_color.b, 0);
        assert!(cell.has_styling());
    }

    #[test]
    fn cell_type_variations() {
        let data_cell = Cell::with_type("Data", CellType::Data);
        let code_cell = Cell::with_type("=SUM(A1:A10)", CellType::Code);
        let var_assign = Cell::with_type("x = 42", CellType::VarAssign);

        assert_eq!(data_cell.cell_type, CellType::Data);
        assert_eq!(code_cell.cell_type, CellType::Code);
        assert_eq!(var_assign.cell_type, CellType::VarAssign);
    }

    #[test]
    fn cell_folding() {
        let mut cell = Cell::new("Parent");
        cell.add_child(Cell::new("Child 1"));
        cell.add_child(Cell::new("Child 2"));

        assert!(!cell.folded);
        cell.toggle_fold();
        assert!(cell.folded);
        cell.toggle_fold();
        assert!(!cell.folded);
    }

    #[test]
    fn cell_layout_orientation() {
        let mut cell = Cell::new("Container");
        assert_eq!(cell.layout, GridLayout::Vertical);

        cell.set_layout(GridLayout::Horizontal);
        assert_eq!(cell.layout, GridLayout::Horizontal);
    }

    #[test]
    fn cell_walk_mut() {
        let mut cell = Cell::new("root");
        cell.add_child(Cell::new("child1"));
        cell.add_child(Cell::new("child2"));

        // Modify all cells during traversal
        cell.walk_mut(0, &mut |c, depth| {
            c.rel_size = depth as i32;
            c.folded = depth > 0;
        });

        assert_eq!(cell.rel_size, 0);
        assert!(!cell.folded);
        assert_eq!(cell.children[0].rel_size, 1);
        assert!(cell.children[0].folded);
        assert_eq!(cell.children[1].rel_size, 1);
        assert!(cell.children[1].folded);
    }

    #[test]
    fn color_constants() {
        assert_eq!(Color::WHITE.r, 255);
        assert_eq!(Color::WHITE.g, 255);
        assert_eq!(Color::WHITE.b, 255);

        assert_eq!(Color::BLACK.r, 0);
        assert_eq!(Color::BLACK.g, 0);
        assert_eq!(Color::BLACK.b, 0);

        assert_eq!(Color::DEFAULT_CELL, Color::WHITE);
        assert_eq!(Color::DEFAULT_TEXT, Color::BLACK);
    }

    #[test]
    fn style_bits_operations() {
        let mut style = StyleBits::new();
        assert!(!style.is_bold());
        assert!(!style.is_italic());

        style.set_bold(true);
        assert!(style.is_bold());
        assert!(!style.is_italic());

        style.set_italic(true);
        assert!(style.is_bold());
        assert!(style.is_italic());

        style.set_bold(false);
        assert!(!style.is_bold());
        assert!(style.is_italic());
    }
}
