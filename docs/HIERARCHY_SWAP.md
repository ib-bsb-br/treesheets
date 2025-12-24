# Hierarchy Swap Deep Dive

This document replaces prior hierarchy-swap docs and restates the feature directly from the current source code. It covers prerequisites, code entry points, algorithm flow, and corrected end-to-end examples (including the cases that previously contained mistakes).

## Command Surface
- **Menu / Shortcut:** Organization → Hierarchy Swap (F8), defined in [`tsframe.h`](../src/tsframe.h#L333-L348).
- **Action ID:** `A_HSWAP` in [`main.cpp`](../src/main.cpp#L130-L180).
- **Validation and dispatch:** `Document::Action()` at [`document.h`](../src/document.h#L1860-L1920) enforces:
  - The selected cell has a parent and grandparent.
  - Both parent and grandparent grids are 1×N or N×1.
  - An undo step is recorded before calling the swap.

## Code Map
| Area | Location | Role |
| --- | --- | --- |
| Grid-level search | [`Grid::FindExact`](../src/grid.h#L867-L888) | Recursively finds the first cell whose text matches the selected tag. |
| Main algorithm | [`Grid::HierarchySwap`](../src/grid.h#L889-L938) | Promotes each match, rebuilds its parent chain under it, merges like-tag peers, and restarts the search until no matches remain. |
| Parent cleanup | [`Grid::DeleteTagParent`](../src/grid.h#L940-L964) | Removes the promoted node from its old location, deleting empty 1×1 ancestors on the way up. |
| Merge helpers | [`Grid::MergeTagCell`](../src/grid.h#L966-L991) and [`Grid::MergeTagAll`](../src/grid.h#L993-L1000) | Merge promoted cells (and their grids) when the target level already contains the same tag. |
| Parent pointer fix-up | [`Grid::ReParent`](../src/grid.h#L1010-L1014) | Retargets parent pointers whenever a grid is transplanted. |
| Cell-level match | [`Cell::FindExact`](../src/cell.h#L484-L503) | Base-case exact-text comparison used by `Grid::FindExact`. |

## Algorithm (from the current implementation)
1. **Search scope:** Start in the grandparent grid of the selected cell (the grid that owns the parent’s parent). Iterate each direct child that has a grid.
2. **Find first match:** Use `Grid::FindExact(tag)` to locate the first cell in that child grid whose text equals the selected tag (case-sensitive). If none, continue to the next child.
3. **Build reversed chain:** For the found cell `f`, walk its parent chain up to (but not including) the grandparent cell that owns the running grid. For each ancestor `p`:
   - If `p->text` matches the tag, set `done = true` to stop after this promotion and avoid infinite swaps.
   - Clone `p` into a new cell attached under `f`, transfer `f`’s current grid to that clone, call `ReParent`, and give `f` a fresh 1×1 grid containing the clone. This makes every ancestor become a child nested under the promoted tag.
4. **Detach the original chain:** Call `DeleteTagParent` repeatedly while walking upward, deleting empty 1×1 ancestors and cleaning up the spot where the match used to live.
5. **Merge at target level:**
   - If the target grid was empty, place `f` there.
   - Otherwise call `MergeTagCell`, which either merges `f` into an existing like-named cell (combining grids via `MergeTagAll` when both have grids) or appends it if no duplicate exists. The first merged/added cell becomes the returned selection.
6. **Restart search:** `goto lookformore` restarts the sweep so newly created structure is also scanned. The loop ends when no further matches remain or `done` was set because an ancestor already matched the tag.
7. **Return selection:** The function returns a `Selection` pointing to the promoted/merged tag at the target level.

### Behavioral Notes
- **Grid shape:** The operation only runs on 1×N or N×1 grids (checked before calling the algorithm).
- **Exact text match:** Matching is literal and case-sensitive (`Cell::FindExact`).
- **Search order:** Because the search restarts after every promotion, newly merged structures can be processed in subsequent passes.
- **Ancestor protection:** If an ancestor already has the same tag, `done` stops further promotions after that chain is processed to prevent cycling the same text upward forever.
- **Merge semantics:** When the target grid already contains the tag, children from both structures are merged under the surviving tag cell.
- **Two-level hops:** Each keypress works against the selected cell’s grandparent grid, so very deep matches may need multiple presses to bubble all the way to the top-level grid where siblings live.

## Corrected Examples
The following scenarios are constructed directly against the algorithm above; they supersede the earlier examples.

### 1) Baseline: Single Match Promotion
**Before** (select "Alice", grandparent grid owns `Project`):
```
Projects
├─ Project A
│  └─ Alice
└─ Project B
   └─ Bob
```

**After F8 on "Alice":**
```
Projects
├─ Project B
│  └─ Bob
└─ Alice
   └─ Project A
```
- The `Alice` branch moves to the grandparent grid. `Project A` becomes a child under `Alice`. Other siblings stay put.

### 2) Multiple Matches at Different Depths (corrected)
**Before** (grandparent grid is `Colors`):
```
Colors
├─ Warm
│  ├─ Red (match 1)
│  └─ Orange
├─ Cool
│  └─ Blue
└─ Mixed
   └─ Purple
      └─ Red (match 2)
```

**After F8 on either "Red":**
```
Colors
├─ Warm
│  └─ Orange
├─ Cool
│  └─ Blue
└─ Red
   ├─ Warm
   └─ Mixed
      └─ Purple
```
- First match promotes `Warm` under `Red`, leaving `Orange` behind.
- Second match promotes `Mixed → Purple` under another `Red`.
- The two `Red` results merge at the `Colors` level; children from both chains are preserved.

**Alternate representations for testing:**
- Plain text (pre-swap): `colors\n  warm\n    red\n    orange\n  cool\n    blue\n  mixed\n    purple\n      red\n`
- Plain text (post-swap): `colors\n  warm\n    orange\n  cool\n    blue\n  red\n    warm\n    mixed\n      purple\n`
- XML (pre/post) mirrors the structures above and helps diffing serialized `.cts` files.

### 3) Single-Path with Repeated Tags
**Before** (select the deeper "Tag", grandparent grid is the root):
```
Root
└─ Tag (outer)
   └─ Tag (inner)
      └─ Item
```

**After F8 on inner "Tag":**
```
Root
└─ Tag
   └─ Tag
      └─ Item
```
- The inner match is promoted to the root grid.
- The original outer tag is preserved as a child under the promoted tag (the ancestor clone step).
- Because an ancestor shared the tag, `done` stops further passes after this promotion.

### 4) Grid Merge with Existing Hierarchy (corrected)
**Before** (select any `tag` cell; grandparent grid is `main`):
```
main
├─ branch1
│  └─ tag
│     ├─ a
│     ├─ b
│     └─ c
└─ branch2
   └─ tag
      ├─ d
      ├─ e
      └─ f
```

**After F8 on `tag`:**
```
main
└─ tag
   ├─ branch1
   │  ├─ a
   │  ├─ b
   │  └─ c
   └─ branch2
      ├─ d
      ├─ e
      └─ f
```
- Each `tag` is promoted; their parent chains (`branch1`, `branch2`) become children under the promoted tags.
- The promoted tags merge at the `main` level, combining both sub-branches under one `tag`.

### 5) Deep Match with Mixed Siblings (depth-dependent passes)
**Before (same starting state for all runs):**
```
Departments
├─ Sales
│  └─ Q4
│     └─ Jamie   ← "shallow" (two presses to reach Departments)
├─ Support
│  └─ Jamie      ← "mid-depth" (one press to reach Departments)
└─ Engineering
   └─ Backend
      └─ Team A
         └─ Jamie   ← "deep" (three presses to reach Departments)
```

**If you press F8 on the shallow Jamie (under `Sales → Q4`):**
- *After the first press (scope = `Sales` grid):*
  ```
  Departments
  ├─ Sales
  │  └─ Jamie
  │     └─ Q4
  ├─ Support
  │  └─ Jamie
  └─ Engineering
     └─ Backend
        └─ Team A
           └─ Jamie
  ```
- *After the second press (scope = `Departments` grid):*
  ```
  Departments
  └─ Jamie
     ├─ Sales
     │  └─ Q4
     ├─ Support
     └─ Engineering
        └─ Backend
           └─ Team A
  ```
  - First press bubbles the match two levels (grandparent = `Sales`).
  - Second press runs at `Departments`, finds all three `Jamie` matches, promotes each, and merges them at the top level.

**If you press F8 on the mid-depth Jamie (under `Support`):**
```
Departments
└─ Jamie
   ├─ Sales
   │  └─ Q4
   ├─ Support
   └─ Engineering
      └─ Backend
         └─ Team A
```
- One press is enough because the selected cell’s grandparent grid is already `Departments`, so all three matches are discovered and merged in a single pass.

**If you press F8 on the deep Jamie (under `Engineering → Backend → Team A`):**
- *After one press (scope = `Backend` grid):*
  ```
  Departments
  ├─ Sales
  │  └─ Q4
  │     └─ Jamie
  ├─ Support
  │  └─ Jamie
  └─ Engineering
     └─ Backend
        └─ Jamie
           └─ Team A
  ```
- *After two presses (scope = `Engineering` grid):*
  ```
  Departments
  ├─ Sales
  │  └─ Q4
  │     └─ Jamie
  ├─ Support
  │  └─ Jamie
  └─ Engineering
     └─ Jamie
        └─ Backend
           └─ Team A
  ```
- *After three presses (scope = `Departments` grid):*
  ```
  Departments
  └─ Jamie
     ├─ Sales
     │  └─ Q4
     ├─ Support
     └─ Engineering
        └─ Backend
           └─ Team A
  ```
- Each press operates two levels up from the current selection, so the deep match must be swapped three times to reach the shared `Departments` grid where the merge can occur.

**Press counts by depth:**
| Selected `Jamie` | Initial depth relative to `Departments` | Presses to merge all three | Why |
| --- | --- | --- | --- |
| Shallow (`Sales → Q4 → Jamie`) | Great-grandchild | 2 | First press moves into the parent’s parent (`Sales`); second press runs in `Departments` and merges everything. |
| Mid-depth (`Support → Jamie`) | Grandchild | 1 | Already two levels below `Departments`; one pass finds all matches. |
| Deep (`Engineering → Backend → Team A → Jamie`) | Great-great-grandchild | 3 | Needs three hops (Backend → Engineering → Departments) because each swap uses the current grandparent grid. |

## Practical Testing Checklist
- Verify swaps only run when both parent and grandparent grids are 1×N or N×1.
- Confirm merged results keep every child grid (use Examples 2 and 4).
- Exercise the ancestor-tag guard by creating a chain with repeated tags (Example 3).
- Ensure undo works: perform a swap and Ctrl+Z to restore.

