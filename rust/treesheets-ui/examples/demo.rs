/// TreeSheets UI Demo
///
/// Run with: cargo run --example demo
///
/// This demo application showcases the basic TreeSheets GUI with:
/// - Hierarchical tree/grid display
/// - Keyboard navigation (arrows, Enter, Escape)
/// - Cell editing
/// - Sample sheet loading

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Enable logging
    treesheets_ui::run()
}
