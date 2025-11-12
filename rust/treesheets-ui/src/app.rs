use eframe::egui;
use treesheets_core::{Cell, Sheet};

/// Main application state for TreeSheets
pub struct TreeSheetsApp {
    /// The currently loaded sheet
    sheet: Sheet,
    /// Currently selected cell path (indices from root)
    selected_path: Vec<usize>,
    /// Whether we're in edit mode
    editing: bool,
    /// Edit buffer for the current cell
    edit_buffer: String,
}

impl TreeSheetsApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Start with a sample sheet
        Self {
            sheet: Sheet::sample(),
            selected_path: vec![],
            editing: false,
            edit_buffer: String::new(),
        }
    }

    /// Get the currently selected cell, if any
    fn get_selected_cell(&self) -> Option<&Cell> {
        let mut current = &self.sheet.root;
        for &index in &self.selected_path {
            current = current.children.get(index)?;
        }
        Some(current)
    }

    /// Get the currently selected cell mutably, if any
    fn get_selected_cell_mut(&mut self) -> Option<&mut Cell> {
        let mut current = &mut self.sheet.root;
        for &index in &self.selected_path {
            current = current.children.get_mut(index)?;
        }
        Some(current)
    }

    /// Handle keyboard navigation
    fn handle_keyboard(&mut self, ctx: &egui::Context) {
        // Check for keyboard input
        ctx.input(|i| {
            // Enter to edit current cell
            if i.key_pressed(egui::Key::Enter) && !self.editing {
                if let Some(cell) = self.get_selected_cell() {
                    let text = cell.text.clone();
                    self.editing = true;
                    self.edit_buffer = text;
                }
            }

            // Escape to cancel editing
            if i.key_pressed(egui::Key::Escape) && self.editing {
                self.editing = false;
                self.edit_buffer.clear();
            }

            // Arrow keys for navigation (only when not editing)
            if !self.editing {
                if i.key_pressed(egui::Key::ArrowDown) {
                    self.navigate_down();
                }
                if i.key_pressed(egui::Key::ArrowUp) {
                    self.navigate_up();
                }
                if i.key_pressed(egui::Key::ArrowRight) {
                    self.navigate_into();
                }
                if i.key_pressed(egui::Key::ArrowLeft) {
                    self.navigate_out();
                }
            }
        });
    }

    /// Navigate to the next sibling
    fn navigate_down(&mut self) {
        if let Some(&last) = self.selected_path.last() {
            let parent = self.get_parent_cell();
            if let Some(parent) = parent {
                if last + 1 < parent.children.len() {
                    if let Some(last_mut) = self.selected_path.last_mut() {
                        *last_mut += 1;
                    }
                }
            }
        }
    }

    /// Navigate to the previous sibling
    fn navigate_up(&mut self) {
        if let Some(last) = self.selected_path.last_mut() {
            if *last > 0 {
                *last -= 1;
            }
        }
    }

    /// Navigate into the first child
    fn navigate_into(&mut self) {
        if let Some(cell) = self.get_selected_cell() {
            if !cell.children.is_empty() {
                self.selected_path.push(0);
            }
        }
    }

    /// Navigate to parent
    fn navigate_out(&mut self) {
        self.selected_path.pop();
    }

    /// Get the parent of the currently selected cell
    fn get_parent_cell(&self) -> Option<&Cell> {
        if self.selected_path.is_empty() {
            return Some(&self.sheet.root);
        }
        let mut current = &self.sheet.root;
        for &index in &self.selected_path[..self.selected_path.len() - 1] {
            current = current.children.get(index)?;
        }
        Some(current)
    }

    /// Render the sheet hierarchy
    fn render_cell(
        &self,
        ui: &mut egui::Ui,
        cell: &Cell,
        path: Vec<usize>,
        depth: usize,
        selected_path: &[usize],
        on_select: &mut dyn FnMut(Vec<usize>),
    ) {
        let is_selected = path == selected_path;

        ui.horizontal(|ui| {
            // Indentation
            ui.add_space(depth as f32 * 20.0);

            // Selection indicator
            if is_selected {
                ui.label(egui::RichText::new("▶").color(egui::Color32::LIGHT_BLUE));
            } else {
                ui.label(" ");
            }

            // Cell text
            let text = if cell.children.is_empty() {
                format!("  {}", cell.text)
            } else {
                format!("▼ {}", cell.text)
            };

            let label = if is_selected {
                egui::RichText::new(text)
                    .color(egui::Color32::LIGHT_BLUE)
                    .strong()
            } else {
                egui::RichText::new(text)
            };

            if ui.selectable_label(is_selected, label).clicked() {
                on_select(path.clone());
            }
        });

        // Render children (only if not folded)
        if !cell.folded {
            for (i, child) in cell.children.iter().enumerate() {
                let mut child_path = path.clone();
                child_path.push(i);
                self.render_cell(ui, child, child_path, depth + 1, selected_path, on_select);
            }
        }
    }
}

impl eframe::App for TreeSheetsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard input
        self.handle_keyboard(ctx);

        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.sheet = Sheet::new("New Sheet", Cell::new("Root"));
                        self.selected_path.clear();
                    }
                    if ui.button("Sample").clicked() {
                        self.sheet = Sheet::sample();
                        self.selected_path.clear();
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Edit Cell (Enter)").clicked() {
                        if let Some(cell) = self.get_selected_cell() {
                            let text = cell.text.clone();
                            self.editing = true;
                            self.edit_buffer = text;
                        }
                    }
                });

                ui.menu_button("Help", |ui| {
                    ui.label("Keyboard Shortcuts:");
                    ui.label("  ↑/↓: Navigate siblings");
                    ui.label("  ←/→: Navigate parent/child");
                    ui.label("  Enter: Edit cell");
                    ui.label("  Escape: Cancel editing");
                });
            });
        });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.sheet.title);
            ui.separator();

            // Sheet view with scrolling
            egui::ScrollArea::both().show(ui, |ui| {
                let selected_path = self.selected_path.clone();
                let mut new_selection = None;
                self.render_cell(
                    ui,
                    &self.sheet.root,
                    vec![],
                    0,
                    &selected_path,
                    &mut |path| {
                        new_selection = Some(path);
                    },
                );
                if let Some(path) = new_selection {
                    self.selected_path = path;
                    self.editing = false;
                }
            });
        });

        // Edit dialog (modal)
        if self.editing {
            egui::Window::new("Edit Cell")
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Edit cell text:");
                    let response = ui.text_edit_singleline(&mut self.edit_buffer);

                    // Auto-focus the text edit
                    let should_save =
                        response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                    let mut ok_clicked = false;
                    let mut cancel_clicked = false;

                    ui.horizontal(|ui| {
                        if ui.button("OK").clicked() {
                            ok_clicked = true;
                        }
                        if ui.button("Cancel").clicked() {
                            cancel_clicked = true;
                        }
                    });

                    // Request focus on the text edit
                    response.request_focus();

                    if should_save || ok_clicked {
                        // Save changes
                        let new_text = self.edit_buffer.clone();
                        if let Some(cell) = self.get_selected_cell_mut() {
                            cell.text = new_text;
                        }
                        self.editing = false;
                        self.edit_buffer.clear();
                    } else if cancel_clicked {
                        self.editing = false;
                        self.edit_buffer.clear();
                    }
                });
        }

        // Status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(cell) = self.get_selected_cell() {
                    ui.label(format!(
                        "Selected: {} | Children: {} | Path depth: {}",
                        cell.text,
                        cell.children.len(),
                        self.selected_path.len()
                    ));
                } else {
                    ui.label("No selection");
                }
            });
        });
    }
}
