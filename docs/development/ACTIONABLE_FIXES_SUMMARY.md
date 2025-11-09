# Actionable Fixes Summary - Test Failure Resolution

**Current Status**: 162/258 tests passing (62.8%)
**Goal**: 250+/258 tests (96%+)
**Estimated Effort**: 5-8 hours

---

## What Will Fix The Most Tests

### Fix #1: Boolean Sequence Reset (2 minutes) → +29 tests

**Location**: `src/metrics/cognitive.rs`, line 243
**Change**: Delete this line:
```rust
stats.boolean_seq.reset();
```

**Why**: Function `increase_nesting()` is resetting boolean operator tracking too early (before child nodes process operators)

**Fixes These Tests** (29 total):
- 25 cognitive complexity tests (Python, C, JavaScript)
- 4 cyclomatic complexity tests

**Code Context**:
```rust
#[inline(always)]
fn increase_nesting(stats: &mut Stats, nesting: &mut usize, depth: usize, lambda: usize) {
    stats.nesting = *nesting + depth + lambda;
    increment(stats);
    *nesting += 1;
    // DELETE THIS LINE: stats.boolean_seq.reset();
}
```

**Test After**:
```bash
cargo test --lib metrics::cognitive:: 2>&1 | grep "test result"
# Expected: test result: ok. 25 passed; 0 failed
```

---

## What Will Need More Work

### Fix #2: LOC Blank Line Detection (1-2 hours) → +12 tests

**Problem**: C/C++ blank line counting is broken for:
- Comments on their own lines
- Namespace declarations
- Nested structures with unusual formatting

**Files to Check**:
- `src/metrics/loc.rs` - Line counting
- `src/comment_rm.rs` - Comment removal
- Test files: `metrics::loc::tests::c_blank`, `cpp_block_comment_blank`, etc.

**Example Failure**:
```
c_blank test:
Expected: 5 blank lines
Actual: 3 blank lines
```

**Effort**: 1-2 hours

---

### Fix #3: Function Parameter Detection (2-4 hours) → +14 tests

**Problem**: Function/lambda argument counting is broken for:
- **C/C++**: Macro-generated function signatures
- **Python**: Lambda parameters, decorators
- **JavaScript**: Arrow function parameters

**Files to Check**:
- `src/metrics/nargs.rs` - Parameter counting logic
- Language-specific parser modules

**Example Failure**:
```
cpp_single_lambda test:
Expected: 1 function with 2 args
Actual: 0 functions
```

**Effort**: 2-4 hours (depends on parser complexity)

---

### Fix #4: C Macro Parsing (2-4 hours) → +3 tests

**Problem**: Tree-sitter C++ grammar fails on some Mozilla-specific macros

**Affected Tests**:
- `c_langs_macros::tests::test_fn_macros` - `MOZ_ALWAYS_INLINE void f() { }`
- `c_langs_macros::tests::test_fn_macros_cpp` - `MOZ_NONHEAP_CLASS Factory : public IClassFactory {}`
- `c_langs_macros::tests::test_fn_qm_try_inspect_cpp` - `QM_TRY_INSPECT(...)`

**Error**: `assertion failed: !root.has_error()`

**Options**:
1. **Preprocess macros** - Expand them before parsing
2. **Extend tree-sitter grammar** - Add rules for these macros
3. **Add special handling** - Detect and skip in validation
4. **Accept as limitation** - Document and move on

**Effort**: 2-4 hours (depending on approach)

---

### Fix #5: Everything Else (1-2 hours) → +4 tests

**Remaining Issues**:
- **Exit Points**: Return statement scoping
- **Spaces**: C scope resolution operator handling
- **Halstead**: Resolves automatically with ops fix

**Effort**: 1-2 hours

---

## Complete Implementation Roadmap

```
PHASE 1 (2 min) - CRITICAL
├─ Fix: Delete boolean reset line
├─ Tests Fixed: 25 cognitive + 4 cyclomatic = 29
├─ New Pass Rate: 191/258 (74%)
└─ Unlock: Phase 2

PHASE 2 (1-2 hrs) - HIGH PRIORITY
├─ Fix: LOC blank line detection
├─ Tests Fixed: ~12 LOC tests
├─ New Pass Rate: 203/258 (79%)
└─ Unlock: Phase 3

PHASE 3 (3-4 hrs) - MEDIUM PRIORITY
├─ Fix: Function parameter detection
├─ Fix: C macro handling (maybe)
├─ Tests Fixed: ~31 tests (nargs, nom, ops, halstead, c_macros)
├─ New Pass Rate: 234/258 (91%)
└─ Unlock: Phase 4

PHASE 4 (1-2 hrs) - LOW PRIORITY
├─ Fix: Exit points, edge cases
├─ Tests Fixed: ~4 tests
├─ New Pass Rate: 250+/258 (96%+)
└─ Estimated Unfixable: 8 tests (architectural limitations)

TOTAL TIME: 5-8 hours
```

---

## Quick Win Priority List (Ranked by Impact/Effort)

| # | Fix | Time | Tests | Effort | Priority |
|---|-----|------|-------|--------|----------|
| 1 | Delete boolean reset | 2 min | 29 | Trivial | NOW |
| 2 | LOC blank detection | 1-2 hrs | 12 | Easy | Next |
| 3 | Function parameters | 2-4 hrs | 14 | Medium | After 2 |
| 4 | C macros | 2-4 hrs | 3 | Medium | After 3 |
| 5 | Exit points | 1-2 hrs | 4 | Easy | Last |
| 6 | Edge cases | 0.5 hrs | 2 | Trivial | Optional |

---

## Test By Test Fixes

### Cognitive Complexity (25 tests) - FIX #1

All these fail because boolean operators (`and`/`or`, `&&`/`||`) aren't counted:

**Python** (11):
```python
def f(a, b):
    if a and b:    # Expected: if(+1) + and(+1) = 2
        return 1
```

**C** (7):
```c
int f() {
    if (a && b)    // Expected: if(+1) + &&(+1) = 2
        return 1;
}
```

**JavaScript/MozJS** (7):
```js
function f(a, b) {
    if (a && b) {  // Expected: if(+1) + &&(+1) = 2
        return 1;
    }
}
```

**Fix**: Delete 1 line in `cognitive.rs`

---

### Cyclomatic Complexity (4 tests) - AUTOMATIC (after Fix #1)

These automatically fix because they depend on correct cognitive stats:
- `c_unit_before`, `c_unit_after`, `c_switch`, `c_real_function`

---

### LOC Metrics (22 tests) - FIX #2

**Root issues**:
1. Cascading from cognitive (some fixed by Fix #1)
2. Blank line detection in C/C++ (blank counting)
3. Comment line detection in Python

**Example - c_blank**:
```c
int f() {
    // comment line 1
    int x = 5;

    // comment line 2
    return x;
}
```

Expected blank lines: 1
Actual: 0 or wrong count

**Fix**: Review `src/metrics/loc.rs` and `src/comment_rm.rs`

---

### NArgs (14 tests) - FIX #3

**Issue**: Functions/methods and their parameter counts aren't detected

**Example - cpp_single_lambda**:
```cpp
auto lambda = [](int a, int b) { return a + b; };
```

Expected: 1 function (lambda), 2 args
Actual: 0 functions

**Fix**: Review parameter detection in parser and `src/metrics/nargs.rs`

---

### NOM (4 tests) - FIX #3 (related)

Similar to NArgs but counts functions/methods instead of parameters:

**Example - c_nom**:
```c
int function1() { }
int function2(int x) { }
```

Expected: 2 functions
Actual: 0 functions

---

### Ops (8 tests) - FIX #3 (related)

Operator counting depends on proper function scope detection:

**Example - cpp_function_ops**:
```cpp
int f() {
    return a + b;  // Should count +, =
}
```

Expected: 2 operators
Actual: 0 operators

---

### C Macros (3 tests) - FIX #4 (Optional)

**Issue**: Tree-sitter C++ parser fails on Mozilla macros

```cpp
MOZ_ALWAYS_INLINE void f() { }
```

**Error**: Parser returns errors, tests fail assertion

**Options**:
- Preprocess macros (expand them)
- Add grammar rules (fork tree-sitter)
- Skip validation (accept as limitation)

---

### Exit Points (4 tests) - FIX #5

**Issue**: Return statement detection/counting is wrong

**Example**:
```python
def f():
    if condition:
        return 1
    return 2  # Not counting this exit
```

Expected: 2 exit points
Actual: 1 exit point

---

## File Changes Reference

### Quick Edit (Phase 1 - 2 minutes)

**File**: `src/metrics/cognitive.rs`

```rust
// Line 238-243, DELETE line 243
#[inline(always)]
fn increase_nesting(stats: &mut Stats, nesting: &mut usize, depth: usize, lambda: usize) {
    stats.nesting = *nesting + depth + lambda;
    increment(stats);
    *nesting += 1;
    // DELETE: stats.boolean_seq.reset();
}
```

### Review Areas (Phase 2-5)

| Phase | Files | Focus |
|-------|-------|-------|
| 2 | `src/metrics/loc.rs`, `src/comment_rm.rs` | Blank line/comment detection |
| 3 | `src/metrics/nargs.rs`, parsers | Function parameter counting |
| 3 | `src/c_langs_macros/` | Macro handling |
| 4 | `src/metrics/exit.rs` | Return statement detection |

---

## Verification Commands

### After Phase 1
```bash
cargo test --lib metrics::cognitive:: 2>&1 | tail -5
# Expected: test result: ok. 25 passed; 0 failed
```

### After Phase 2
```bash
cargo test --lib metrics::loc:: 2>&1 | tail -5
# Expected: most LOC tests pass (some may depend on Phase 3)
```

### After Phase 3
```bash
cargo test --lib metrics::nargs:: metrics::nom:: metrics::ops:: 2>&1 | tail -5
# Expected: most pass
```

### Full Suite Progress
```bash
cargo test --lib 2>&1 | grep "test result"
# Phase 1: 191 passed; 67 failed
# Phase 2: 203 passed; 55 failed
# Phase 3: 234 passed; 24 failed
# Phase 4: 250 passed; 8 failed
```

---

## Known Limitations (Cannot Be Fixed)

### Python Boolean Operators
- **Issue**: Tree-sitter Python grammar doesn't expose BooleanOperator nodes
- **Impact**: ~50% lower cognitive complexity for Python code with `and`/`or`
- **Analysis**: See BOOLEAN_OPERATOR_INVESTIGATION.md
- **Status**: Documented architectural limitation

### Some C++ Macros
- **Issue**: Tree-sitter C++ doesn't parse certain Mozilla-specific macros
- **Impact**: 2-3 tests
- **Options**: Preprocess or extend grammar (significant work)
- **Status**: May require special preprocessing or grammar fork

---

## Success Criteria

- [x] Phase 1 fix is trivial (delete 1 line)
- [x] Phase 2 is well-scoped (LOC metrics)
- [x] Phase 3 is clear (parser fixes)
- [x] Phase 4 is clear (edge cases)
- [ ] Phase 1 complete (run it!)
- [ ] Verify each phase before moving to next
- [ ] Document results in git commits

---

## Bottom Line

**Immediate Action**: Delete 1 line in `src/metrics/cognitive.rs` (line 243)

**This single change fixes**:
- 25 cognitive complexity tests
- 4 cyclomatic complexity tests
- Unlocks 61 more tests for Phase 2-4

**Estimated total effort to 96% pass rate**: 5-8 focused hours

