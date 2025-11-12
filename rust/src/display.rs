use crate::sheet::{Cell, Sheet};

/// Produces a pretty-printed textual view of the sheet hierarchy.
pub fn format_sheet(sheet: &Sheet) -> String {
    let mut buffer = String::new();
    buffer.push_str(&format!("# {}\n", sheet.title));
    render_cell(&sheet.root, 0, &mut buffer);
    buffer
}

fn render_cell(cell: &Cell, depth: usize, buffer: &mut String) {
    let indent = "  ".repeat(depth);
    buffer.push_str(&format!("{indent}- {}\n", cell.text));
    for child in &cell.children {
        render_cell(child, depth + 1, buffer);
    }
}

/// Writes the formatted sheet to stdout. The function stays small so it can be
/// reused from the CLI entry point without additional configuration.
pub fn print_sheet(sheet: &Sheet) {
    print!("{}", format_sheet(sheet));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatting_produces_expected_structure() {
        let formatted = format_sheet(&Sheet::sample());
        let expected = r#"# Sample Sheet
- TreeSheets Rust Prototype
  - Personal
    - Tasks
    - Notes
  - Work
    - TreeSheets RS
      - Implement sheet data model
      - Design CLI workflows
    - Retrospective
"#;
        assert_eq!(formatted, expected);
    }
}
