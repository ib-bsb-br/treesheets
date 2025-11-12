# TreeSheets Hierarchy Swap Feature - Technical Documentation

## Overview

The **Hierarchy Swap** feature is a powerful data reorganization operation in TreeSheets that allows users to restructure hierarchical data by swapping cell relationships. It enables users to "promote" cells with specific text tags up through the hierarchy, inverting parent-child relationships in the process.

## Purpose and Use Case (WHY)

### Problem Solved
In hierarchical data structures, it's common to realize that the organization needs to be inverted. For example:
- You might have organized data by **Project → Task → Employee**, but realize it would be better organized as **Employee → Project → Task**
- Or you might have **Category → Subcategory → Item** and want to reorganize to **Item → Category → Subcategory**

Without this feature, manually reorganizing such data would be tedious and error-prone, requiring:
1. Copying cells
2. Creating new hierarchies
3. Moving data between structures
4. Deleting old structures

### Feature Benefits
- **Efficiency**: Reorganizes complex hierarchical data with a single operation (F8 key)
- **Data Integrity**: Preserves all data and relationships during reorganization
- **Flexibility**: Allows different views of the same data by changing hierarchy levels
- **Productivity**: Saves significant time when restructuring large datasets

## How It Works (HOW)

### User Interface

**Menu Location**: Organization → Hierarchy Swap (F8)

**Menu Description**: "Swap all cells with this text at this level (or above) with the parent"

**Requirements**:
1. The selected cell must have a parent and grandparent (at least 2 levels deep)
2. The parent grid must be 1xN or Nx1 (unidimensional)
3. The grandparent grid must be 1xN or Nx1 (unidimensional)

### Algorithm Overview

The hierarchy swap operation performs these high-level steps:

1. **Find All Matching Cells**: Search for all cells at the current level (or in subgrids) that have the same text as the selected cell
2. **Invert Hierarchy**: For each found cell, swap it with its parent chain, making the child become the parent
3. **Preserve Children**: Move the original parent's other properties down into the newly promoted cell's hierarchy
4. **Cleanup**: Remove empty parent cells after extraction
5. **Merge**: Merge all promoted cells with the same tag at the target level

### Detailed Algorithm Flow

```
Input: tag (the text of the selected cell)

1. Initialize: selcell = null, done = false

2. For each cell c in the current grid:
   a. If c has a subgrid:
      i. Search for cell f with exact tag match in c's subgrid
      ii. If found:
          - Collect parent chain (from f up to current cell)
          - Create new nested structure with parents as children
          - Remove f from its original location
          - Merge f at the current level
          - Repeat search for more matches

3. Return selection pointing to the resulting cell
```

### Example Transformation

**Before Hierarchy Swap** (F8 on "Alice"):
```
Project A
  ├─ Task 1
  │   └─ Alice
  └─ Task 2
      └─ Alice

Project B
  └─ Task 3
      └─ Alice
```

**After Hierarchy Swap**:
```
Alice
  ├─ Project A
  │   ├─ Task 1
  │   └─ Task 2
  └─ Project B
      └─ Task 3
```

## Code Structure (WHAT)

### Files Involved

#### 1. **src/grid.h** (Primary Implementation)
Contains the core implementation of the hierarchy swap feature.

**Key Functions**:

- **`HierarchySwap(wxString tag)`** (lines 883-919)
  - Main entry point for the hierarchy swap operation
  - Searches for all cells matching the tag
  - Orchestrates the restructuring process
  - Returns a Selection object pointing to the result

- **`FindExact(const wxString &s)`** (lines 875-881)
  - Recursively searches for cells with exact text match
  - Used to locate all instances of the target tag

- **`MergeTagCell(Cell *f, Cell *&selcell)`** (lines 948-966)
  - Merges a cell into the current grid level
  - Handles cases where cells with the same tag already exist
  - Combines hierarchies when both cells have subgrids

- **`MergeTagAll(Cell *into)`** (lines 968-973)
  - Helper function to merge all cells from one grid into another
  - Used during the merge phase of hierarchy swap

- **`DeleteTagParent(Cell *tag, Cell *basecell, Cell *found)`** (lines 926-946)
  - Removes cells from their parent grid
  - Cleans up empty grids after extraction
  - Recursively deletes parent cells that become empty

- **`ReParent(auto p)`** (lines 921-924)
  - Updates parent references for all cells in a grid
  - Used when transferring grids between cells

#### 2. **src/document.h** (Command Handler)
Handles the user action and validates preconditions.

**Location**: Lines 1889-1902

**Key Responsibilities**:
- Validates that the cell has sufficient hierarchy depth (checks for grandparent)
- Validates that parent and grandparent grids are unidimensional (Nx1 or 1xN)
- Creates undo point before operation
- Calls `HierarchySwap()` on the grandparent grid
- Updates UI after operation (resets layout, refreshes canvas)

**Error Messages**:
- "Cannot move this cell up in the hierarchy." - No grandparent exists
- "Can only move this cell into a Nx1 or 1xN grid." - Grandparent grid invalid
- "Can only move this cell from a Nx1 or 1xN grid." - Parent grid invalid

#### 3. **src/cell.h** (Cell-level Search)
Provides cell-level exact text matching.

**Location**: Lines 487-489

**Key Function**:
- **`FindExact(const wxString &s)`**
  - Returns this cell if text matches, otherwise searches recursively in subgrid
  - Base case for Grid's FindExact function

#### 4. **src/tsframe.h** (Menu Definition)
Defines the menu entry and keyboard shortcut.

**Location**: Line 342

**Configuration**:
- Menu text: "Hierarchy &Swap"
- Keyboard shortcut: F8
- Tooltip: "Swap all cells with this text at this level (or above) with the parent"

#### 5. **src/main.cpp** (Action Constant)
Defines the action identifier.

**Location**: Line 156

**Constant**: `A_HSWAP` - Action identifier used throughout the codebase

### Data Structures

#### Grid Structure
- **cells**: Array of Cell pointers arranged in an xs × ys grid
- **xs, ys**: Dimensions of the grid (width and height)
- **cell**: Pointer to the parent Cell that owns this grid

#### Cell Structure
- **text.t**: wxString containing the cell's text content
- **parent**: Pointer to the parent Cell
- **grid**: Pointer to the child Grid (if this cell has hierarchy)

### Key Algorithms

#### 1. Parent Chain Reversal
```cpp
// Add all parent tags as extra hierarchy inside the cell
for (auto p = f->parent; p != cell; p = p->parent) {
    auto t = new Cell(f, p);
    t->text = p->text;
    t->text.cell = t;
    t->grid = f->grid;
    if (t->grid) t->grid->ReParent(t);
    f->grid = new Grid(1, 1);
    f->grid->cell = f;
    *f->grid->cells = t;
}
```
This loop iterates up the parent chain, creating new cells that will become children of the promoted cell. Each parent becomes a child in reverse order.

#### 2. Infinite Loop Prevention
```cpp
if (p->text.t == tag) done = true;
```
Special case check: If any parent has the same text as the tag being swapped, it would cause infinite swapping. The `done` flag prevents finding more matches after this case.

#### 3. Recursive Cleanup
```cpp
for (auto r = f; r && r != cell; r = r->parent->grid->DeleteTagParent(r, cell, f));
```
After extracting a cell, this loop walks up the parent chain deleting empty containers.

### Control Flow

```
User presses F8 on cell with text "X"
    ↓
Document::Action() case A_HSWAP
    ↓
Validate preconditions (grandparent exists, grids are 1D)
    ↓
Create undo point
    ↓
Call pp->grid->HierarchySwap(cell->text.t)
    ↓
    For each cell in grid:
        ↓
        Search subgrids for matching tag
            ↓
            If found:
                ↓
                Reverse parent chain (parents become children)
                ↓
                Remove cell from original location
                ↓
                Merge at current level
                ↓
                Continue searching for more matches
    ↓
Reset layout and refresh UI
    ↓
Return success
```

## Implementation Details

### Memory Management
- New cells and grids are allocated with `new`
- Deleted cells/grids are freed with `delete`
- Parent pointers must be carefully updated to avoid dangling references
- The `ReParent()` function ensures all cell parent pointers remain valid

### Edge Cases Handled
1. **Duplicate Tags**: When multiple cells with the same tag exist, they are merged at the target level
2. **Circular References**: The algorithm detects when a parent has the same tag and stops to prevent infinite loops
3. **Empty Grids**: After extraction, empty parent grids are automatically deleted
4. **Grid Dimensions**: Only works with unidimensional grids (Nx1 or 1xN) to ensure clean reorganization

### Performance Considerations
- Uses `goto lookformore` for efficiency (avoids nested loops and complex state management)
- Performs all operations in-place where possible
- Single pass over cells with immediate processing

## User Workflow

### Typical Usage Scenario

1. **Identify the Tag**: Select a cell whose text represents the new top-level hierarchy
2. **Invoke Command**: Press F8 or select Organization → Hierarchy Swap
3. **Observe Result**: All cells with matching text are promoted and merged at the current level
4. **Undo if Needed**: Use Ctrl+Z to undo if the result isn't what you expected

### Best Practices

- **Use on Clean Hierarchies**: Works best with well-structured, unidimensional hierarchies
- **Preview Structure**: Understand your current hierarchy before swapping
- **Undo Available**: Don't hesitate to try it - you can always undo
- **Unique Tags**: Works most intuitively when tags are relatively unique within a hierarchy level

## Technical Constraints

### Requirements
1. **Minimum Depth**: Cell must be at least 2 levels deep (has grandparent)
2. **Grid Layout**: Parent and grandparent must be unidimensional (Nx1 or 1xN)
3. **Text Match**: Uses exact string matching (case-sensitive)

### Limitations
1. Cannot swap in multidimensional grids (NxM where N>1 and M>1)
2. Cannot swap cells at the root level
3. Cannot swap cells without matching text in the hierarchy

## Related Features

- **Hierarchify** (A_HIFY): Converts flat NxN grids into hierarchical 1xN grids
- **Flatten** (A_FLATTEN): Inverse operation - converts hierarchy to flat grid
- **Sort**: Can organize cells within a level before swapping

## Testing Considerations

To test this feature effectively:

1. **Basic Swap**: Create simple 2-level hierarchy and swap
2. **Multiple Matches**: Test with multiple cells having the same tag
3. **Deep Hierarchy**: Test with 3+ levels of nesting
4. **Edge Cases**:
   - Duplicate parent names (circular reference prevention)
   - Empty cells after extraction
   - Single-cell grids
5. **Error Cases**:
   - Attempt swap at root level
   - Attempt swap in 2D grid
   - Attempt swap without grandparent

## Code Quality Notes

### Strengths
- Clear comments explaining key steps
- Handles edge cases gracefully
- Efficient single-pass algorithm
- Proper memory management

### Areas for Potential Enhancement
- The `goto` statement could be replaced with a proper loop structure for better code clarity
- More detailed error messages could help users understand why a swap failed
- Could add preview functionality to show what the result would look like

## Summary

The Hierarchy Swap feature is a sophisticated data reorganization tool that:
- **Inverts hierarchical relationships** between cells based on text matching
- **Preserves all data** during the transformation
- **Handles complex edge cases** like circular references and grid merging
- **Provides immediate visual feedback** through UI updates

It demonstrates advanced tree manipulation algorithms and careful memory management in a GUI application context.
