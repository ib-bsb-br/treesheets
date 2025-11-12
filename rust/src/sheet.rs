use serde::{Deserialize, Serialize};

/// A single TreeSheets cell containing text and zero or more child cells.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Cell {
    /// Textual content of the cell. TreeSheets allows free-form text, so this
    /// representation keeps the payload as an owned string.
    pub text: String,
    /// Child cells that form the hierarchical grid within this cell.
    #[serde(default)]
    pub children: Vec<Cell>,
}

impl Cell {
    /// Creates a new cell with the provided text and no children.
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self {
            text: text.into(),
            children: Vec::new(),
        }
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

    /// Provides depth-first traversal over the cell and all descendants, with early termination.
    /// The closure should return `true` to continue traversal, or `false` to stop.
    pub fn walk<F>(&self, depth: usize, f: &mut F) -> bool
    where
        F: FnMut(&Cell, usize) -> bool,
    {
        if !f(self, depth) {
            return false;
        }
        for child in &self.children {
            if !child.walk(depth + 1, f) {
                return false;
            }
        }
        true
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
        self.root.walk(0, &mut |cell, depth| {
            f(cell, depth);
            true
        });
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
            true
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

        assert_eq!(
            labels,
            vec![
                ("TreeSheets Rust Prototype".into(), 0),
                ("Personal".into(), 1),
                ("Tasks".into(), 2),
                ("Notes".into(), 2),
                ("Work".into(), 1),
                ("TreeSheets RS".into(), 2),
                ("Implement sheet data model".into(), 3),
                ("Design CLI workflows".into(), 3),
                ("Retrospective".into(), 2),
            ]
        );
    }
}
