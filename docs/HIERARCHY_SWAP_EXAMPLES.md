# TreeSheets Hierarchy Swap - Visual Examples

This document provides visual examples of how the Hierarchy Swap feature transforms data structures.

## Example 1: Simple Employee-Project Swap

### Initial Structure (Before F8 on "Alice")

```
Grid (grandparent level)
├── Project A (parent level)
│   └── Task 1
│       └── Alice (selected cell - to be swapped)
└── Project B (parent level)
    └── Task 2
        └── Alice
```

### After Hierarchy Swap

```
Grid (grandparent level)
└── Alice (promoted to parent level)
    ├── Project A (demoted)
    │   └── Task 1
    └── Project B (demoted)
        └── Task 2
```

**What happened:**
1. All "Alice" cells were found in the hierarchy
2. Each "Alice" was promoted up, and its parents became its children
3. All "Alice" cells were merged into a single cell at the grandparent level
4. The parent/child relationship was inverted

## Example 2: Multiple Matches at Different Levels

### Initial Structure (Before F8 on "Red")

```
Colors
├── Warm
│   ├── Red (match 1)
│   └── Orange
├── Cool
│   └── Blue
└── Mixed
    └── Purple
        └── Red (match 2 - deeper in hierarchy)
```

### After Hierarchy Swap

```
Colors
└── Red (merged result)
    ├── Warm (from match 1's parent)
    └── Purple (from match 2's parent)
        └── Mixed (from match 2's grandparent)
```

**What happened:**
1. First "Red" found under "Warm" - promoted with "Warm" becoming its child
2. Second "Red" found under "Purple" → "Mixed" - promoted with entire parent chain
3. Both "Red" cells merged at the Colors level
4. The merged "Red" has both parent chains as children

## Example 3: Preventing Infinite Loop (Circular Reference)

### Initial Structure (Before F8 on "Category")

```
Root
└── Category (level 1)
    └── Category (level 2 - same name!)
        └── Item
```

### After Hierarchy Swap

```
Root
└── Category (promoted)
    └── Category (child - from level 1)
        └── Item
```

**What happened:**
1. First "Category" at level 2 is found and promoted
2. The algorithm detects parent also has name "Category"
3. Sets `done = true` to prevent infinite swapping
4. Stops searching for more matches to avoid loop

## Example 4: Grid Merge with Existing Hierarchy

### Initial Structure (Before F8 on "Tag")

```
Main
├── Branch1
│   └── Tag (has subgrid with: A, B, C)
└── Branch2
    └── Tag (has subgrid with: D, E, F)
```

### After Hierarchy Swap

```
Main
└── Tag (merged with combined subgrids)
    ├── Branch1
    │   └── (empty - cleaned up)
    ├── Branch2
    │   └── (empty - cleaned up)
    ├── A (from first Tag's subgrid)
    ├── B
    ├── C
    ├── D (from second Tag's subgrid)
    ├── E
    └── F
```

**What happened:**
1. Both "Tag" cells found and promoted
2. Their subgrids (A,B,C and D,E,F) are merged
3. The merged result contains all children from both original Tags
4. Empty parent grids are deleted

## Example 5: Real-World Use Case - Task Management

### Before: Organized by Project

```
Projects
├── Website Redesign
│   ├── Design mockups (Alice)
│   ├── Frontend code (Bob)
│   └── Testing (Charlie)
├── Mobile App
│   ├── UI Design (Alice)
│   ├── Backend API (Bob)
│   └── Testing (Charlie)
└── Marketing Campaign
    ├── Content creation (Alice)
    └── Analytics (Charlie)
```

### After: Swap on "Alice" - Now Organized by Person

```
Projects
├── Alice
│   ├── Website Redesign
│   │   └── Design mockups
│   ├── Mobile App
│   │   └── UI Design
│   └── Marketing Campaign
│       └── Content creation
├── [Bob and Charlie cells remain under their projects]
```

### After: Swap on each person - Complete Reorganization

```
Projects
├── Alice
│   ├── Website Redesign → Design mockups
│   ├── Mobile App → UI Design
│   └── Marketing Campaign → Content creation
├── Bob
│   ├── Website Redesign → Frontend code
│   └── Mobile App → Backend API
└── Charlie
    ├── Website Redesign → Testing
    ├── Mobile App → Testing
    └── Marketing Campaign → Analytics
```

## Algorithm Walkthrough: Step-by-Step

Let's trace through the algorithm with a simple example:

### Input Data
```
Grid: [Cell1, Cell2]
Cell1: "Parent1" → Grid: [Cell_A]
Cell_A: "Alice" → Grid: [Item1]
Cell2: "Parent2" → Grid: [Cell_B]
Cell_B: "Alice" → Grid: [Item2]
```

### Execution Steps

**Step 1**: Call `HierarchySwap("Alice")` on the main Grid

**Step 2**: Initialize
- `selcell = nullptr`
- `done = false`

**Step 3**: Loop through Grid cells

**Step 3a**: Process Cell1 ("Parent1")
- Has subgrid, so search in it
- Find Cell_A with tag "Alice" ✓

**Step 4**: Reverse parent chain for Cell_A
- Parent chain: Cell_A → Parent1 → Grid
- Create new cell structure:
  ```
  Alice (Cell_A)
  └── Grid (1x1)
      └── Parent1 (new cell, copy of old parent)
          └── Grid (transferred from Cell_A)
              └── Item1
  ```

**Step 5**: Remove Cell_A from its original location
- Delete Cell_A from Cell1's subgrid
- Delete Cell1 (now empty) from main Grid

**Step 6**: Add reorganized Cell_A to main Grid
- Since main Grid is empty, place Cell_A directly: `*cells = Cell_A`
- Set `selcell = Cell_A`

**Step 7**: Continue loop - Process Cell2 ("Parent2")
- Has subgrid, so search in it
- Find Cell_B with tag "Alice" ✓

**Step 8**: Reverse parent chain for Cell_B
- Similar structure created as Step 4

**Step 9**: Remove Cell_B and merge
- Delete Cell_B from Cell2's subgrid
- Delete Cell2 (now empty)
- Call `MergeTagCell(Cell_B, selcell)`

**Step 10**: MergeTagCell
- Find existing cell with tag "Alice" in grid (Cell_A from step 6)
- Both have subgrids, so merge:
  - Add Cell_B's children to Cell_A's grid
  - Delete Cell_B

**Step 11**: Final Result
```
Grid: [Alice]
Alice → Grid: [Parent1, Parent2]
  Parent1 → Grid: [Item1]
  Parent2 → Grid: [Item2]
```

## Error Cases

### Error Case 1: No Grandparent

**Initial:**
```
Root
└── Child (selected)
```

**Error:** "Cannot move this cell up in the hierarchy."
**Reason:** No grandparent exists to move into

### Error Case 2: 2D Parent Grid

**Initial:**
```
Grid (grandparent)
└── Parent (2x2 grid)
    ├── A    │ B
    ├──────────
    └── C    │ D (selected)
```

**Error:** "Can only move this cell from a Nx1 or 1xN grid."
**Reason:** Parent grid is 2x2, not unidimensional

### Error Case 3: 2D Grandparent Grid

**Initial:**
```
Grid (3x3 grandparent grid)
├── A    │ B    │ C
├──────────────────
├── D    │ E    │ F
│        └── Child (selected)
├──────────────────
└── G    │ H    │ I
```

**Error:** "Can only move this cell into a Nx1 or 1xN grid."
**Reason:** Grandparent grid is 3x3, not unidimensional

## Performance Characteristics

### Time Complexity
- **Best Case**: O(n) where n = number of cells in grid
  - Single match found, no merging needed
  
- **Worst Case**: O(n * m * d) where:
  - n = number of cells in grid
  - m = number of matches found
  - d = depth of hierarchy
  
### Space Complexity
- O(d) for the parent chain
- Additional space for new cells during restructuring

## Implementation Notes

### Why Use goto?
The `goto lookformore` statement allows the algorithm to restart the cell iteration after modifying the grid structure. This is more efficient than:
- Recursive calls (stack overhead)
- Complex state tracking with nested loops
- Multiple passes over the data

### Memory Safety
The algorithm is careful about:
1. Deleting cells only after they're no longer referenced
2. Updating parent pointers before accessing children
3. Cleaning up empty grids to prevent memory leaks

### Thread Safety
The operation is not thread-safe and must be called from the UI thread, as it:
- Modifies the document structure
- Updates the UI (canvas refresh)
- Interacts with the undo system
