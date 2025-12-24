Grounded within the the repository source files and functions related to hierarchy swap, like, for example:

```
Source Files
File	Lines	Purpose
src/grid.h	883-973	Core implementation of swap algorithm
src/document.h	1889-1902	Command handler and validation
src/cell.h	487-489	Cell-level text matching
src/tsframe.h	342	Menu definition
src/main.cpp	156	Action constant definition

Key Functions
Function	Location	Purpose
HierarchySwap()	grid.h:883	Main algorithm entry point
FindExact()	grid.h:875	Searches for cells by text
MergeTagCell()	grid.h:948	Merges cells with same tag
DeleteTagParent()	grid.h:926	Removes and cleans up cells
MergeTagAll()	grid.h:968	Merges all children
ReParent()	grid.h:921	Updates parent references
```

your tasks, as the AI ASSISTANT, are all the ones needed to properly construct extensive examples and documentations to fill each and every test usecase regarding the 'hierarchy swap' feature within 'treesheets'. Like, for example, the actual within the repository 'docs' folder, aka, 'Example 4: Grid Merge with Existing Hierarchy' is wrong, because it should be as follows:

```original
<?xml version="1.0" encoding="UTF-8"?>
@DOCTYPE cell [
<!ELEMENT cell (grid)
@ELEMENT grid (row*)
@ELEMENT row (cell*)
]>
<cell colorbg="0xCCDCE2">\n  <grid>
  <row>
  <cell>main\n    <grid>
    <row>
    <cell>branch1\n      <grid>
      <row>
      <cell>tag\n        <grid>
        <row>
        <cell>a\n        </cell>
        </row>
        <row>
        <cell>b\n        </cell>
        </row>
        <row>
        <cell>c\n        </cell>
        </row>
        </grid>
      </cell>
      </row>
      </grid>
    </cell>
    </row>
    <row>
    <cell>branch2\n      <grid>
      <row>
      <cell>tag\n        <grid>
        <row>
        <cell>d\n        </cell>
        </row>
        <row>
        <cell>e\n        </cell>
        </row>
        <row>
        <cell>f\n        </cell>
        </row>
        </grid>
      </cell>
      </row>
      </grid>
    </cell>
    </row>
    </grid>
  </cell>
  </row>
  </grid>
</cell>
```

~~~after_swap
<?xml version="1.0" encoding="UTF-8"?>
@DOCTYPE cell [
<!ELEMENT cell (grid)
@ELEMENT grid (row*)
@ELEMENT row (cell*)
]>
<cell colorbg="0xCCDCE2">\n  <grid>
  <row>
  <cell>main\n    <grid>
    <row>
    <cell>tag\n      <grid>
      <row>
      <cell>branch1\n        <grid>
        <row>
        <cell>a\n        </cell>
        </row>
        <row>
        <cell>b\n        </cell>
        </row>
        <row>
        <cell>c\n        </cell>
        </row>
        </grid>
      </cell>
      </row>
      <row>
      <cell>branch2\n        <grid>
        <row>
        <cell>d\n        </cell>
        </row>
        <row>
        <cell>e\n        </cell>
        </row>
        <row>
        <cell>f\n        </cell>
        </row>
        </grid>
      </cell>
      </row>
      </grid>
    </cell>
    </row>
    </grid>
  </cell>
  </row>
  </grid>
</cell>
~~~

Other cases:

	
"""
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
├── Warm
│   └── Orange
├── Cool
│   └── Blue
└── Red
    ├── Warm
    └── Mixed
        └── Purple
```

"""

Another way of presenting the above given information:

'''

### Initial Structure (Before F8 on "Red")

```
colors\n  warm\n    red\n    orange\n  cool\n    blue\n  mixed\n    purple\n      red\n```

### After Hierarchy Swap

```
colors\n  warm\n    orange\n  cool\n    blue\n  red\n    warm\n    mixed\n      purple\n```
'''

An third way of presenting the above given information showing the error from the original docs:

'''

### Initial Structure (Before F8 on "Red")

```
<?xml version="1.0" encoding="UTF-8"?>
@DOCTYPE cell [
<!ELEMENT cell (grid)
@ELEMENT grid (row*)
@ELEMENT row (cell*)
]>
<cell colorbg="0xCCDCE2">\n  <grid>
  <row>
  <cell>colors\n    <grid>
    <row>
    <cell>warm\n      <grid>
      <row>
      <cell>red\n      </cell>
      </row>
      <row>
      <cell>orange\n      </cell>
      </row>
      </grid>
    </cell>
    </row>
    <row>
    <cell>cool\n      <grid>
      <row>
      <cell>blue\n      </cell>
      </row>
      </grid>
    </cell>
    </row>
    <row>
    <cell>mixed\n      <grid>
      <row>
      <cell>purple\n        <grid>
        <row>
        <cell>red\n        </cell>
        </row>
        </grid>
      </cell>
      </row>
      </grid>
    </cell>
    </row>
    </grid>
  </cell>
  </row>
  </grid>
</cell>

```

### After Hierarchy Swap

```
<?xml version="1.0" encoding="UTF-8"?>
@DOCTYPE cell [
<!ELEMENT cell (grid)
@ELEMENT grid (row*)
@ELEMENT row (cell*)
]>
<cell colorbg="0xCCDCE2">\n  <grid>
  <row>
  <cell>colors\n    <grid>
    <row>
    <cell>warm\n      <grid>
      <row>
      <cell>orange\n      </cell>
      </row>
      </grid>
    </cell>
    </row>
    <row>
    <cell>cool\n      <grid>
      <row>
      <cell>blue\n      </cell>
      </row>
      </grid>
    </cell>
    </row>
    <row>
    <cell>red\n      <grid>
      <row>
      <cell>warm\n      </cell>
      </row>
      <row>
      <cell>mixed\n        <grid>
        <row>
        <cell>purple\n        </cell>
        </row>
        </grid>
      </cell>
      </row>
      </grid>
    </cell>
    </row>
    </grid>
  </cell>
  </row>
  </grid>
</cell>

```
'''

One more example, but the AI ASSISTANT must construct way more other extensive examples to circumvent each and every usecase that the source code and functions allows:

:::example
Deep Match with Mixed Siblings
**Before** (grandparent grid is `Departments`):
```
Departments
├─ Sales
│  └─ Q4
│     └─ Jamie
├─ Support
│  └─ Jamie
└─ Engineering
   └─ Backend
      └─ Team A
         └─ Jamie
```

**After F8 one-time on the "shallow Jamie":**
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

**After F8 two-times on the "shallow Jamie":**
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



**After F8 one-time on the "mid-depth Jamie":**
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

**After F8 one-time on the "deep Jamie":**
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

**After F8 two-times on the "deep Jamie":**
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

**After F8 three-times on the "deep Jamie":**
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

- All three `Jamie` matches (one shallow, one mid-depth, one deep) are promoted and merged at the `Departments` level, but only when the 'shallow Jamie' is hit twice by the F8 function; the 'mid-depth is hit once by the F8 function; and the 'deep Jamie' is hit thrice by the F8 function.
:::
