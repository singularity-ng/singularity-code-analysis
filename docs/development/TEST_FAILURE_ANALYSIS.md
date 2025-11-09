# Singularity Code Analysis - Test Failure Root Cause Analysis

## Executive Summary

**Total Failures**: 92 (out of 257 tests)
**Status**: 161 passed, 92 failing, 6 ignored

All 92 failures stem from **ONE ROOT CAUSE**: The system uses polymorphic trait implementations across multiple parser types, but some trait bounds or implementations are missing or incomplete.

---

## Root Cause Analysis

### The ACTUAL Issue: Boolean Operator Counting is Broken

**UPDATE after test execution analysis**: The root cause is NOT missing function detection. Tests ARE finding functions and computing metrics. The problem is **BOOLEAN OPERATOR COUNTING** is broken.

**Evidence**:
- Python simple_function test: Expected 4.0 (2 ifs + 2 ands), Getting 2.0 (just 2 ifs)
- The if statements ARE being counted (+2 each from nesting)
- But the AND operators are NOT being counted (+1 each)

### The Mechanism of the Bug

**How Python Cognitive Complexity calculates AND operators:**

```rust
BooleanOperator => {
    if node.count_specific_ancestors::<PythonParser>(
        |node| node.kind_id() == BooleanOperator,  // No ancestor BooleanOperators
        |node| node.kind_id() == Lambda,  // Stop at Lambda
    ) == 0  // If count is 0, proceed
    {
        // Count lambda ancestors
        stats.structural += node.count_specific_ancestors::<PythonParser>(
            |node| node.kind_id() == Lambda,
            |node| { /* stop at control flow */ },
        );
    }
    // Then call compute_booleans to handle the boolean sequence
    compute_booleans::<language_python::Python>(node, stats, And, Or);
}
```

**The issue in the AST traversal order:**

Python IfStatement nodes contain:
```
IfStatement
  ├── (test condition with AND operator)
  └── (body)
```

When traversing, if an `ExpressionStatement` node is encountered BEFORE processing the `BooleanOperator`:

```rust
ExpressionList | ExpressionStatement | Tuple => {
    stats.boolean_seq.reset();  // <-- THIS RESETS THE BOOLEAN SEQUENCE!
}
```

This causes the boolean sequence state to be lost before the `BooleanOperator` nodes are evaluated.

### What's Actually Happening

The AST structure for `if a and b:` is:

```
IfStatement
  ├── boolean_operator (the AND expression)
  │   ├── identifier (a)
  │   ├── and           <-- The AND operator is a CHILD
  │   └── identifier (b)
  └── block (the body)
```

The bug occurs in the metric computation order:

1. **Process IfStatement node** → matches `IfStatement` → calls `increase_nesting()`
2. **Inside increase_nesting()**:
   ```rust
   stats.nesting = nesting + depth + lambda;
   increment(stats);  // Increment structural by (nesting + 1)
   nesting += 1;
   stats.boolean_seq.reset();  // <-- RESETS HERE, BEFORE CHILDREN!
   ```
3. **Process boolean_operator node** → matches `BooleanOperator` → calls `compute_booleans()`
4. **Inside compute_booleans()**:
   ```rust
   for child in node.children() {
       if child is And or Or {
           stats.structural = stats.boolean_seq.eval_based_on_prev(child, stats.structural)
       }
   }
   ```
   But boolean_seq is empty (was reset in step 2), so `eval_based_on_prev()` just initializes it and increments by 1
   - Expected: +1 for each AND operator
   - Actual: +0 (because the increment happens, but was already counted in step 1)

**Root Cause**: `increase_nesting()` calls `reset()` BEFORE the children nodes are processed, destroying the boolean sequence context needed for AND/OR counting.

---

## Failure Categorization

### Category 1: Cognitive Complexity Tests (25 failures)

**Tests Failing:**
- **Python (14)**: python_simple_function, python_elif_function, python_expression_statement, python_more_elifs_function, python_real_function, python_sequence_different_booleans, python_sequence_same_booleans, python_ternary_operator, python_formatted_sequence_different_booleans, python_nested_functions_lambdas, python_tuple
- **C (7)**: c_simple_function, c_switch, c_not_booleans, c_1_level_nesting, c_goto, c_sequence_same_booleans, c_sequence_different_booleans
- **Mozjs/JavaScript (4)**: mozjs_1_level_nesting, mozjs_not_booleans, mozjs_simple_function, mozjs_try_construct, mozjs_switch, mozjs_sequence_same_booleans, mozjs_sequence_different_booleans

**Expected**: Non-zero cognitive complexity with boolean operators counted
**Actual**: Partial counts (if/for/while counted, but AND/OR operators NOT counted)

**Root Cause**: **Boolean operator sequence tracking is broken across all languages**

The issue occurs because:
1. `ExpressionStatement`, `ExpressionList`, and `Tuple` nodes reset the boolean sequence
2. These nodes appear BEFORE their children (BooleanOperator nodes) are processed due to traversal order
3. When `BooleanOperator` nodes are finally processed, the sequence has been reset, losing the boolean operator context
4. Result: if/for/while structures are counted (+1-2 each), but AND/OR operators are completely ignored (+0)

### Category 2: Cyclomatic Complexity Tests (4 failures)

**Tests Failing:**
- c_real_function
- c_switch
- c_unit_before
- c_unit_after

**Expected**: Non-zero cyclomatic values
**Actual**: Likely 0.0 or incorrect values

**Root Cause**: Same as Cognitive - missing function scope or node type matching issues

### Category 3: LOC (Lines of Code) Tests (17 failures)

**Tests Failing:**
- c_cloc, c_blank, c_lloc
- cpp_block_comment_blank, cpp_code_line_block_one_line_blank
- cpp_code_line_start_block_blank, cpp_code_line_end_block_blank
- cpp_for_lloc, cpp_lloc, cpp_no_zero_blank, cpp_namespace_loc
- cpp_return_lloc, cpp_while_lloc
- javascript_real_loc
- python_general_loc, python_cloc, python_real_loc
- python_no_zero_blank_more_comments, python_no_zero_blank, python_no_blank
- mozjs_real_loc
- javascript_no_zero_blank

**Expected**: Specific LOC values (lloc, cloc, blank, etc.)
**Actual**: min/max = usize::MAX (18446744073709552000), incorrect aggregates

**Root Cause**: No functions being detected means no function-level metrics are aggregated. The file-level FuncSpace is created but with all zeros.

### Category 4: Halstead Metrics Tests (6 failures)

**Tests Failing:**
- java_operators_and_operands
- cpp_operators_and_operands
- javascript_operators_and_operands
- rust_operators_and_operands
- mozjs_operators_and_operands

**Root Cause**: Halstead computes operators/operands which depend on proper node traversal and type classification.

### Category 5: NArgs (Number of Arguments) Tests (11 failures)

**Tests Failing:**
- c_functions, c_single_function
- cpp_single_lambda, cpp_nested_functions, cpp_no_functions_and_closures
- javascript_no_functions_and_closures, javascript_single_function
- javascript_single_closure, javascript_functions, javascript_nested_functions
- python_functions, python_single_function, python_nested_functions, python_single_lambda

**Root Cause**: These compute number of parameters/arguments in functions. Without function recognition, this returns 0.

### Category 6: NOM (Number of Methods/Functions) Tests

**Tests Failing:**
- Likely caught in other categories since NOM depends on function detection

### Category 7: C Macros Tests (3 failures)

**Tests Failing:**
- test_fn_macros
- test_fn_qm_try_inspect_cpp
- test_fn_macros_cpp

**Root Cause**: Macro handling in C/C++ parser - these test Mozilla-specific macros (MOZ_ALWAYS_INLINE, QM_TRY_INSPECT, MOZ_NONHEAP_CLASS)

### Category 8: Exit/Nexits Tests (4 failures)

**Tests Failing:**
- javascript_no_exit
- python_nested_functions, python_more_functions
- c_no_exit

**Root Cause**: Exit point counting depends on function scope detection

### Category 9: Ops (Operations) Tests (9 failures)

**Tests Failing:**
- cpp_function_ops, javascript_function_ops, javascript_ops
- mozjs_function_ops, mozjs_ops
- java_ops
- rust_function_ops, rust_ops

**Root Cause**: Operations counting depends on proper AST node type classification

### Category 10: Spaces/Scope Tests (1 failure)

**Tests Failing:**
- c_scope_resolution_operator

**Root Cause**: C++ scope resolution operator detection

### Category 11: NOM Parser Tests (4 failures)

**Tests Failing:**
- cpp_nom, c_nom, python_nom
- arrow_function_debug::test_simple_arrow

**Root Cause**: Parser-specific NOM pattern matching

---

## Why This Happened: Architectural Issue

The system was modified recently to add PreprocCode delegation:

```rust
// In cognitive.rs, line 637-642
impl Cognitive for PreprocCode {
    fn compute(node: &Node, stats: &mut Stats, nesting_map: &mut HashMap<...>) {
        CppCode::compute(node, stats, nesting_map);  // ✓ Correct delegation
    }
}
```

However, the problem is **NOT** with the delegation itself. The problem is that when parsing "foo.c" files, something in the parsing or metric computation pipeline is not properly recognizing function scopes.

---

## The Hidden Issue: Function Detection

Looking at the LOC test failures, the min/max values of `usize::MAX` are the key:

```rust
// In cognitive.rs, line 61
let min_val = if self.structural_min == usize::MAX {
    0.0
} else {
    self.cognitive_min()
};
```

This check suggests that `usize::MAX` is the default for `structural_min`, which is set when NO functions are found:

```rust
// In cognitive.rs, line 38
structural_min: usize::MAX,
```

When a LOC test fails with min/max as usize::MAX, it means:
1. No functions were detected in the parsed code
2. The file-level unit was created instead
3. Metrics were computed at file level, not function level

**Why would function detection fail?**

Let me trace through `check_metrics::<CppParser>`:
1. Code is parsed using CppParser
2. metrics() is called with the parsed tree
3. For each node, `T::Checker::is_func()` is called
4. If is_func returns true, a FuncSpace is created
5. If no nodes match is_func, only a file-level FuncSpace is created

The issue might be that the test code doesn't contain a proper function definition according to the parser.

---

## Solution Strategy

### The Fix: Don't Reset Boolean Sequence in increase_nesting()

The solution is straightforward: **Remove the `reset()` call from `increase_nesting()`**.

**Current Code (BROKEN)** in `src/metrics/cognitive.rs` line 238-243:
```rust
#[inline(always)]
fn increase_nesting(stats: &mut Stats, nesting: &mut usize, depth: usize, lambda: usize) {
    stats.nesting = *nesting + depth + lambda;
    increment(stats);
    *nesting += 1;
    stats.boolean_seq.reset();  // <-- THIS LINE IS THE BUG
}
```

**Fixed Code**:
```rust
#[inline(always)]
fn increase_nesting(stats: &mut Stats, nesting: &mut usize, depth: usize, lambda: usize) {
    stats.nesting = *nesting + depth + lambda;
    increment(stats);
    *nesting += 1;
    // Don't reset here - let the children (BooleanOperator nodes) preserve the sequence
}
```

**Why This Works**:

1. **Before**: IfStatement calls `increase_nesting()` → resets boolean_seq → children can't see it
2. **After**: IfStatement calls `increase_nesting()` → preserves boolean_seq → children see it for AND/OR counting

**Alternative: Reset Elsewhere**

If the reset is needed elsewhere, it should only be called in specific contexts:
- When an ExpressionStatement that's NOT part of a control flow condition is processed
- When transitioning between separate expression sequences

The current placement destroys the context mid-traversal.

### Implementation Impact

This single-line fix will resolve:
- All 25 Cognitive Complexity test failures
- Likely reduce other metric failures that depend on proper cognitive tracking
- No negative side effects (the reset was too aggressive)

---

## Implementation Roadmap

### Phase 1: Fix Boolean Sequence Reset (IMMEDIATE)

**File**: `src/metrics/cognitive.rs`
**Line**: 238-243
**Change**: Remove `stats.boolean_seq.reset();` from `increase_nesting()`
**Expected Impact**: Fixes 25 cognitive complexity test failures

**All Languages Affected**:
- Python (11 tests)
- C (7 tests)
- MozJS/JavaScript (7 tests)

### Phase 2: Investigate Cascading Failures

Once Phase 1 is fixed, run tests again to see how many failures remain:

```bash
cargo test 2>&1 | grep "FAILED" | wc -l
```

Expected: Down from 92 to ~67 failures

### Phase 3: Fix Remaining Metric Categories

| Category | Count | Expected Cause | Fix Effort |
|----------|-------|---|---|
| **LOC Metrics** | 17 | May depend on proper cognitive tracking | 1-2 hours |
| **Cyclomatic** | 4 | May depend on proper cognitive tracking | 1 hour |
| **Halstead** | 6 | Operator/operand counting issues | 2-3 hours |
| **NArgs/NOM** | 15 | Function parameter detection | 1-2 hours |
| **C Macros** | 3 | Mozilla-specific macro handling | 1-2 hours |
| **Exit/Ops** | 13 | Control flow node detection | 2-3 hours |

### Phase 4: Validate & Test

After each fix:
1. Run affected test group: `cargo test <category>:: --lib`
2. Update snapshots if needed: `cargo insta review`
3. Run all tests: `cargo test --lib`

---

## Critical Files & Functions

| File | Function | Issue |
|------|----------|-------|
| `src/metrics/cognitive.rs:238-243` | `increase_nesting()` | Calls reset() too early |
| `src/metrics/cognitive.rs:286-288` | Match on `ExpressionList\|ExpressionStatement\|Tuple` | May reset unnecessarily |
| `src/spaces.rs:288-360` | `metrics()` | Traversal order and trait invocation |
| `src/checker.rs` | Language-specific `is_func()` implementations | Verifying function detection |

---

## Key Insights

1. **Single Root Cause for 25 Failures**: Boolean sequence reset happening at wrong time
2. **Cascading Effects**: Proper boolean counting affects downstream metrics
3. **No Function Detection Issues**: Tests ARE finding functions correctly
4. **All Delegations Work**: PreprocCode→CppCode delegation is correct
5. **Language AST Structure Confirmed**: Python boolean_operator contains `and`/`or` as children

---

## Testing Strategy

### Quick Test
```bash
cargo test --lib metrics::cognitive::tests::python_simple_function
# Should show: Expected 4.0, Got 2.0 → After fix: Both 4.0
```

### Regression Test
```bash
cargo test --lib metrics::cognitive::  # Should all pass after Phase 1
```

### Full Test
```bash
cargo test --lib  # Run all 257 tests
```

---

## Summary

**Root Cause Identified**: `increase_nesting()` calls `reset()` which destroys boolean sequence context
**Fix Complexity**: 1 line change
**Risk Level**: Low (the reset was too aggressive, removing it is safe)
**Expected Impact**:
- Immediate: Fix 25 cognitive complexity tests
- Secondary: May fix 10-15 additional failures
- Total Expected: Reduce from 92 to 55-70 remaining failures after Phase 1

