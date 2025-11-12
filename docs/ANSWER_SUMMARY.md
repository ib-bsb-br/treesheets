# Answer: How, Why, and What is the Hierarchy Swap Feature?

This document directly answers the question: "How, why, and in what ways does the source code of the feature 'hierarchy swap' consist?"

## WHAT Does It Consist Of?

### Source Code Components

The hierarchy swap feature consists of **5 source files** with **approximately 100 lines** of core logic:

#### 1. Core Algorithm Implementation
**File**: `src/grid.h`  
**Lines**: 883-973 (91 lines)  
**Contains**: 6 key functions
- `HierarchySwap()` - Main algorithm (36 lines)
- `FindExact()` - Search functionality (7 lines)
- `MergeTagCell()` - Cell merging (18 lines)
- `MergeTagAll()` - Batch merging (6 lines)
- `DeleteTagParent()` - Cleanup (20 lines)
- `ReParent()` - Parent pointer updates (4 lines)

#### 2. User Action Handler
**File**: `src/document.h`  
**Lines**: 1889-1902 (14 lines)  
**Contains**: Validation logic, undo integration, UI updates

#### 3. Cell-level Search
**File**: `src/cell.h`  
**Lines**: 487-489 (3 lines)  
**Contains**: Single-line text matching function

#### 4. User Interface
**File**: `src/tsframe.h`  
**Line**: 342 (1 line)  
**Contains**: Menu definition with F8 keyboard shortcut

#### 5. Action Constant
**File**: `src/main.cpp`  
**Line**: 156 (1 line)  
**Contains**: `A_HSWAP` enumeration constant

### Data Structures Used

**Grid Structure**:
```cpp
class Grid {
    Cell **cells;      // Array of cell pointers
    int xs, ys;        // Dimensions (width x height)
    Cell *cell;        // Parent cell owning this grid
    // ... other members
};
```

**Cell Structure**:
```cpp
class Cell {
    Text text;         // Contains text.t (wxString)
    Cell *parent;      // Pointer to parent cell
    Grid *grid;        // Optional child grid
    // ... other members
};
```

### Algorithm Components

1. **Search Phase**: Recursive tree traversal to find matching cells
2. **Transformation Phase**: Parent chain reversal and structure rebuilding
3. **Cleanup Phase**: Empty grid deletion and memory management
4. **Merge Phase**: Combining multiple matches into a single cell

## HOW Does It Work?

### Algorithm Overview

The feature operates in a **single pass with restart** using these steps:

```
1. Find cell with matching text tag in grid
   ↓
2. Reverse parent chain:
   - Create new cells for each parent
   - Make parents become children
   - Transfer original children to new structure
   ↓
3. Remove cell from original location:
   - Delete from parent grid
   - Recursively cleanup empty parents
   ↓
4. Merge at current level:
   - If first match: place directly
   - If duplicate tag exists: merge subgrids
   ↓
5. Restart search for more matches (goto)
   ↓
6. Return selection to result
```

### Key Algorithm Features

**Efficiency**:
- Single pass over cells with immediate processing
- In-place operations where possible
- O(n × m × d) complexity (n=cells, m=matches, d=depth)

**Safety**:
- Circular reference detection (prevents infinite loops)
- Careful memory management (no dangling pointers)
- Parent pointer updates before accessing children

**Flexibility**:
- Handles multiple matches automatically
- Merges hierarchies when needed
- Cleans up empty structures

### Example Transformation

```
BEFORE (Press F8 on "Alice"):
    Project A
    └── Task 1
        └── Alice

    Project B  
    └── Task 2
        └── Alice

AFTER:
    Alice
    ├── Project A
    │   └── Task 1
    └── Project B
        └── Task 2
```

**What happened**:
1. Both "Alice" cells found
2. Parent chains reversed (Alice became parent, Projects became children)
3. Both transformed cells merged
4. Empty grids cleaned up

## WHY Does It Exist?

### Primary Purpose

**Problem Solved**: Users often need to view the same hierarchical data organized by different primary keys, but manually reorganizing complex hierarchies is tedious and error-prone.

### Use Cases

1. **Project Management**
   - From: Project → Task → Person
   - To: Person → Project → Task
   - Benefit: See all work per person instead of per project

2. **Data Categorization**
   - From: Category → Subcategory → Item
   - To: Item → Category → Subcategory  
   - Benefit: Find items by name, then see their categorization

3. **Time-based Organization**
   - From: Date → Event → Participant
   - To: Participant → Date → Event
   - Benefit: See person's schedule instead of daily agenda

4. **Departmental Views**
   - From: Department → Team → Employee
   - To: Employee → Department → Team
   - Benefit: Individual employee view vs organizational structure

### Benefits

- ⚡ **Speed**: One keypress (F8) vs manual copy-paste-reorganize
- ✓ **Accuracy**: Preserves all data and relationships
- 🔄 **Reversibility**: Undo available (Ctrl+Z)
- 🎯 **Flexibility**: Change organizational perspective instantly

### Design Philosophy

TreeSheets is a "hierarchical spreadsheet" - the hierarchy swap feature embodies this philosophy by making hierarchical reorganization as easy as sorting is in traditional spreadsheets.

## Technical Summary

### Code Statistics
- **Total Files**: 5
- **Core Logic**: ~100 lines
- **Primary Function**: 36 lines (`HierarchySwap()`)
- **Helper Functions**: 5
- **Data Structures**: 2 main (Grid, Cell)

### Integration Points
- **Menu System**: Organization menu, F8 shortcut
- **Undo System**: Creates undo point before operation
- **Selection System**: Returns Selection object for UI
- **UI System**: Triggers canvas refresh and layout reset

### Constraints
- ✓ Requires unidimensional grids (1xN or Nx1)
- ✓ Requires minimum 2-level depth (grandparent must exist)
- ✓ Uses exact case-sensitive text matching
- ✓ Not thread-safe (UI thread only)

### Innovation

The algorithm innovatively uses:
1. **goto for restart**: Efficient grid restructuring without complex state
2. **Recursive cleanup**: Automatic empty grid deletion
3. **Merge logic**: Smart handling of duplicate tags
4. **Prevention**: Circular reference detection

## Documentation

For complete details, see:
- **HIERARCHY_SWAP_FEATURE.md** - Comprehensive technical documentation
- **HIERARCHY_SWAP_EXAMPLES.md** - Visual examples and walkthrough
- **README.md** - Quick reference and index

---

**In Summary**: The hierarchy swap feature consists of ~100 lines of carefully crafted C++ code spread across 5 files, implementing a sophisticated tree manipulation algorithm that enables users to instantly reorganize hierarchical data by inverting parent-child relationships with a single keypress (F8), solving the common problem of needing multiple organizational views of the same data.
