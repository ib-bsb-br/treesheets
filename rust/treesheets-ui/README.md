# TreeSheets UI

Graphical user interface for TreeSheets, built in Rust.

## UI Toolkit Evaluation

### Requirements
- **Keyboard-centric navigation** (CRITICAL): Must support comprehensive keyboard shortcuts and navigation
- **Cross-platform**: Primary target is Linux arm64 (Debian 11), but cross-platform is desired
- **Hierarchical grid rendering**: Display tree-like grids with zoom capabilities
- **Performance**: Handle large datasets efficiently
- **Embedded media**: Support for images and rich formatting
- **Mature ecosystem**: Active development and good documentation

### Evaluated Options

#### 1. egui (Recommended)
**Pros:**
- Immediate mode GUI - simple mental model
- Excellent keyboard support
- Very good performance
- Cross-platform (native + web via WASM)
- Active development and great docs
- Works well with custom rendering
- Good for prototyping

**Cons:**
- Somewhat unconventional for traditional desktop apps
- Custom widgets may require more work

**Verdict**: ⭐ Best choice for rapid prototyping and keyboard-centric workflows

#### 2. Iced
**Pros:**
- Inspired by Elm architecture (clean, predictable)
- Native feel
- Cross-platform
- Growing ecosystem

**Cons:**
- Still maturing
- Less flexible for custom complex widgets
- Smaller community than egui

**Verdict**: Good alternative, but less proven

#### 3. Druid
**Pros:**
- Native widgets
- Good architecture

**Cons:**
- Development has slowed
- Less active community
- Uncertain long-term maintenance

**Verdict**: Not recommended due to maintenance concerns

#### 4. Slint
**Pros:**
- Declarative UI with custom markup language
- Good performance
- Commercial backing

**Cons:**
- Learning curve for custom language
- Less Rust-native feel
- Licensing considerations

**Verdict**: Interesting but adds complexity

### Selected: egui

We've chosen **egui** because:
1. Excellent keyboard navigation support (matches TreeSheets' keyboard-centric nature)
2. Fast development cycle for prototyping
3. Cross-platform with minimal effort
4. Strong community and documentation
5. Flexible enough for custom hierarchical grid widgets

## Architecture Plan

```
treesheets-ui/
├── src/
│   ├── lib.rs              # Main library entry
│   ├── app.rs              # Application state and main loop
│   ├── widgets/
│   │   ├── mod.rs
│   │   ├── sheet_view.rs   # Main hierarchical grid widget
│   │   ├── cell_editor.rs  # In-place cell editing
│   │   └── toolbar.rs      # Command palette / toolbar
│   ├── input/
│   │   ├── mod.rs
│   │   ├── keyboard.rs     # Keyboard navigation and shortcuts
│   │   └── commands.rs     # Command system
│   ├── rendering/
│   │   ├── mod.rs
│   │   ├── grid.rs         # Grid rendering logic
│   │   └── zoom.rs         # Zoom/scale handling
│   └── state/
│       ├── mod.rs
│       ├── selection.rs    # Cell selection state
│       └── history.rs      # Undo/redo system
└── examples/
    └── demo.rs             # Standalone demo application
```

## Development Phases

### Phase 1: Basic UI Shell (CURRENT)
- [x] Evaluate UI toolkits
- [ ] Set up egui dependencies
- [ ] Create basic window with menu
- [ ] Load and display a sheet
- [ ] Basic keyboard navigation (arrows, tab)

### Phase 2: Core Navigation
- [ ] Cell selection (single and range)
- [ ] Keyboard shortcuts (copy, paste, edit)
- [ ] Cell editing with modal/inline editor
- [ ] Undo/redo system

### Phase 3: Advanced Features
- [ ] Zoom in/out
- [ ] Grid folding/unfolding
- [ ] Drag and drop
- [ ] Context menus
- [ ] Search/filter

### Phase 4: Rich Content
- [ ] Text formatting (bold, italic, colors)
- [ ] Image embedding
- [ ] Export/import
- [ ] Clipboard integration

### Phase 5: Polish
- [ ] Themes and customization
- [ ] Performance optimization
- [ ] Accessibility improvements
- [ ] User documentation

## Building and Running

```bash
# Run the demo application
cd treesheets-ui
cargo run --example demo

# Run tests
cargo test
```

## Status

Currently in **Phase 1: Basic UI Shell**. The UI crate is a placeholder pending implementation.
