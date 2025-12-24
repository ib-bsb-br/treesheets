# TreeSheets Feature Documentation Index

This directory contains detailed documentation for specific features in TreeSheets.

## Available Documentation

### [Hierarchy Swap Deep Dive](HIERARCHY_SWAP.md)
Complete, source-backed reference for the hierarchy swap command:
- Command surface (menu/shortcut, action ID, validation)
- File map and key functions with line anchors
- Algorithm walkthrough grounded in current code
- Corrected and expanded examples (multi-depth merges, depth-dependent multi-pass scenarios, grid merges)
- Alternate representations (plain text/XML snippets) for regression checks
- Testing checklist

### [Answer Summary](ANSWER_SUMMARY.md)
High-level overview of the hierarchy swap feature, source file footprint, and why the feature exists.

## Quick Reference

**Name**: Hierarchy Swap  
**Keyboard Shortcut**: F8  
**Menu Location**: Organization → Hierarchy Swap  
**Purpose**: Reorganize hierarchical data by promoting cells whose text matches the selection and merging like-tagged structures.

### Source Files

| File | Purpose |
|------|---------|
| `src/grid.h` | Core implementation of the swap algorithm (search, promote, merge, cleanup). |
| `src/document.h` | Command handler, validation, undo integration, and UI refresh. |
| `src/cell.h` | Cell-level text matching used by the search routine. |
| `src/tsframe.h` | Menu definition and shortcut binding. |
| `src/main.cpp` | Action enumeration (`A_HSWAP`). |

### Key Functions

| Function | Location | Purpose |
|----------|----------|---------|
| `HierarchySwap()` | `grid.h` | Main algorithm entry point. |
| `FindExact()` | `grid.h` / `cell.h` | Recursive exact-text search. |
| `MergeTagCell()` | `grid.h` | Merges promoted tags at the target grid level. |
| `DeleteTagParent()` | `grid.h` | Removes promoted nodes and prunes empty ancestors. |
| `MergeTagAll()` | `grid.h` | Merges all children from a grid into another. |
| `ReParent()` | `grid.h` | Resets parent pointers after grid transplantation. |

### Algorithm at a Glance

```
1. Find all cells with matching text under each child grid of the grandparent.
2. For each match:
   - Clone the ancestor chain under the match (reverse the chain).
   - Detach the match from its original parent(s), pruning empties.
   - Merge/append the promoted node at the target grid.
3. Repeat the search (restart) until no matches remain or an ancestor already used the tag.
4. Return selection to the promoted/merged tag.
```

### Requirements

- Cell must be at least two levels deep (needs a grandparent grid).  
- Parent and grandparent grids must be 1×N or N×1.  
- Matching is exact and case-sensitive.  
- Undo is created before the swap; layout refresh follows the operation.

## Contributing

When updating the hierarchy swap feature, ensure both documents reflect any behavioral changes and that examples remain consistent with the current implementation.
