# TreeSheets Feature Documentation Index

This directory contains detailed documentation for specific features in TreeSheets.

## Available Documentation

### [Hierarchy Swap Feature](HIERARCHY_SWAP_FEATURE.md)
**Complete Technical Documentation**

Comprehensive guide covering:
- **What** the feature consists of (code structure, files, functions)
- **How** it works (algorithm, implementation details, control flow)
- **Why** it exists (purpose, use cases, benefits)

Topics covered:
- Purpose and use cases
- User interface and keyboard shortcuts
- Algorithm overview and detailed flow
- Code structure and file organization
- Key functions and their roles
- Data structures used
- Memory management
- Edge cases and error handling
- Performance considerations
- Testing recommendations

### [Hierarchy Swap Examples](HIERARCHY_SWAP_EXAMPLES.md)
**Visual Examples and Walkthrough**

Practical demonstrations including:
- Before/after transformation examples
- Multiple real-world use cases
- Step-by-step algorithm walkthrough
- Edge case illustrations
- Error scenarios with explanations
- Performance characteristics
- Implementation notes

Use this document to:
- Understand how the feature behaves with actual data
- See visual representations of transformations
- Learn from concrete examples
- Understand error conditions

## Quick Reference

### Feature Summary

**Name**: Hierarchy Swap  
**Keyboard Shortcut**: F8  
**Menu Location**: Organization → Hierarchy Swap  
**Purpose**: Reorganize hierarchical data by inverting parent-child relationships

### Source Files

| File | Lines | Purpose |
|------|-------|---------|
| `src/grid.h` | 883-973 | Core implementation of swap algorithm |
| `src/document.h` | 1889-1902 | Command handler and validation |
| `src/cell.h` | 487-489 | Cell-level text matching |
| `src/tsframe.h` | 342 | Menu definition |
| `src/main.cpp` | 156 | Action constant definition |

### Key Functions

| Function | Location | Purpose |
|----------|----------|---------|
| `HierarchySwap()` | grid.h:883 | Main algorithm entry point |
| `FindExact()` | grid.h:875 | Searches for cells by text |
| `MergeTagCell()` | grid.h:948 | Merges cells with same tag |
| `DeleteTagParent()` | grid.h:926 | Removes and cleans up cells |
| `MergeTagAll()` | grid.h:968 | Merges all children |
| `ReParent()` | grid.h:921 | Updates parent references |

### Algorithm at a Glance

```
1. Find all cells with matching text tag
2. For each match:
   - Reverse the parent chain (parents become children)
   - Remove cell from original location
   - Merge at target level
3. Clean up empty parent grids
4. Return selection to result
```

### Requirements

- ✓ Cell must be at least 2 levels deep (has grandparent)
- ✓ Parent grid must be 1xN or Nx1 (unidimensional)
- ✓ Grandparent grid must be 1xN or Nx1 (unidimensional)

### Common Use Cases

1. **Project → Person reorganization**: Convert project-based task lists to person-based assignments
2. **Category restructuring**: Change from Category → Item to Item → Category
3. **Time-based to entity-based**: Switch from Date → Event to Event → Date
4. **Departmental views**: Transform Department → Employee to Employee → Department

## For Developers

### Understanding the Code

Start with these files in order:
1. **HIERARCHY_SWAP_FEATURE.md** - Read the "Code Structure" section
2. **src/grid.h** - Study the `HierarchySwap()` function (line 883)
3. **HIERARCHY_SWAP_EXAMPLES.md** - Follow the step-by-step walkthrough
4. **src/document.h** - See how the feature is invoked (line 1889)

### Making Changes

Before modifying the hierarchy swap feature:
1. Read the complete documentation
2. Understand the edge cases (circular references, grid merging)
3. Review memory management patterns
4. Consider the undo/redo system integration
5. Test with the examples provided

### Testing Strategy

Recommended test cases:
- ✓ Simple 2-level swap
- ✓ Multiple matches at same level
- ✓ Deep hierarchy (3+ levels)
- ✓ Circular reference prevention
- ✓ Grid merging with existing hierarchy
- ✓ Error conditions (2D grids, no grandparent)

## Feature Relationships

### Related Features

- **Hierarchify** (`A_HIFY`): Converts flat grids to hierarchical structure
- **Flatten** (`A_FLATTEN`): Opposite of Hierarchify - converts hierarchy to flat grid
- **Sort** (`A_SORT`, `A_SORTD`): Organizes cells within a grid level

These features work together to provide flexible data organization in TreeSheets.

## Contributing

When contributing improvements to this feature:

1. **Code Changes**: Update relevant sections in HIERARCHY_SWAP_FEATURE.md
2. **New Examples**: Add to HIERARCHY_SWAP_EXAMPLES.md
3. **Bug Fixes**: Document the edge case in both files
4. **Performance**: Update performance characteristics section

## Questions?

Common questions answered in the documentation:

- **Q**: Why only 1xN or Nx1 grids?  
  **A**: See "Technical Constraints" in HIERARCHY_SWAP_FEATURE.md

- **Q**: What happens with duplicate tags?  
  **A**: See "Example 2: Multiple Matches" in HIERARCHY_SWAP_EXAMPLES.md

- **Q**: Why use goto?  
  **A**: See "Implementation Notes" in HIERARCHY_SWAP_EXAMPLES.md

- **Q**: How is infinite swapping prevented?  
  **A**: See "Key Algorithms" section in HIERARCHY_SWAP_FEATURE.md

## License

This documentation is part of the TreeSheets project. See the main repository license for details.

## Version

Documentation created: 2025-11-12  
TreeSheets version: All versions supporting the hierarchy swap feature  
Last updated: 2025-11-12
