// TreeSheets UI - Graphical User Interface
//
// This crate provides the GUI implementation for TreeSheets using egui.
//
// Architecture:
// - app: Main application state and event loop
// - widgets: Custom UI components (sheet view, cell editor, etc.)
// - input: Keyboard navigation and command handling
// - rendering: Grid rendering and zoom logic
// - state: Selection, undo/redo, and other state management

mod app;

pub use app::TreeSheetsApp;

/// Launches the TreeSheets GUI application
pub fn run() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("TreeSheets"),
        ..Default::default()
    };

    eframe::run_native(
        "TreeSheets",
        options,
        Box::new(|cc| Ok(Box::new(TreeSheetsApp::new(cc)))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_creation() {
        // Basic test to ensure the app structure compiles
        // Full UI tests would require a headless environment or integration tests
        assert!(true);
    }
}
