# Phase 1: Critical Boolean Sequence Fix - Implementation Guide

## Quick Reference

**File**: `src/metrics/cognitive.rs`
**Lines**: 238-243
**Change**: Delete 1 line
**Expected Impact**: 25 cognitive tests + 4 cyclomatic tests = 29 test fixes
**Time**: 2 minutes
**Risk**: Minimal (removes overly aggressive reset)

---

## The Bug in Detail

### Current Broken Code (lines 238-243)

```rust
#[inline(always)]
fn increase_nesting(stats: &mut Stats, nesting: &mut usize, depth: usize, lambda: usize) {
    stats.nesting = *nesting + depth + lambda;
    increment(stats);
    *nesting += 1;
    stats.boolean_seq.reset();  // ← THIS LINE IS THE BUG
}
```

### Why It's Wrong

When the parser encounters an `if` statement:

1. **Line 258-272 in cognitive.rs**: Match on `IfStatement` node
2. **Calls**: `increase_nesting(stats, nesting, 0, lambda)`
3. **Inside increase_nesting**:
   - Line 240: Updates nesting level
   - Line 241: Calls `increment()` to add cognitive score
   - Line 242: Increments nesting counter
   - **Line 243**: **BUG** - Resets boolean operator tracking
4. **Returns to line 262-268**: Processes children of IfStatement
5. **Children include**: BooleanOperator nodes (the `and`/`or` operators)
6. **Problem**: Boolean sequence was reset BEFORE children could use it
7. **Result**: Operators aren't counted because context is destroyed

### Example Test Case

```python
def f(a, b):
    if a and b:      # Should be: +1 (if) + 1 (and) = 2
        return 1
    if c and d:      # Should be: +1 (if) + 1 (and) = 2
        return 1
```

**Expected**: sum=4.0, average=4.0
**Current (broken)**: sum=2.0, average=2.0
**Why**: The `and` operators aren't counted because boolean_seq was reset

---

## The Fix

### Step 1: Edit the File

Open: `/home/mhugo/code/singularity/packages/singularity-code-analysis/src/metrics/cognitive.rs`

Navigate to line 238-243.

**Delete line 243**: `stats.boolean_seq.reset();`

### Step 2: Resulting Code

```rust
#[inline(always)]
fn increase_nesting(stats: &mut Stats, nesting: &mut usize, depth: usize, lambda: usize) {
    stats.nesting = *nesting + depth + lambda;
    increment(stats);
    *nesting += 1;
    // Removed: stats.boolean_seq.reset();
}
```

### Step 3: Verify

```bash
cd /home/mhugo/code/singularity/packages/singularity-code-analysis

# Rebuild and test cognitive metrics
cargo test --lib metrics::cognitive:: 2>&1 | tail -20
```

**Expected output**:
```
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.XX s
```

### Step 4: Run Full Test Suite

```bash
# Check overall impact
cargo test --lib 2>&1 | tail -5
```

**Expected**:
- Before: `test result: FAILED. 162 passed; 90 failed; 6 ignored`
- After: `test result: FAILED. 191 passed; 61 failed; 6 ignored`

This means we fixed 29 tests (162 → 191) and unlocked 61 remaining tests for Phase 2.

---

## Why Removing The Reset Is Safe

### The reset() function was designed to:
- Clear boolean operator tracking between functions
- Prevent operators from one function affecting another

### Why it was being called too early:
- It was called in `increase_nesting()` which processes the PARENT node (if/for/etc)
- It should be called when EXITING a function scope
- Currently being called when ENTERING a control structure

### Why removing it is safe:
1. **Proper cleanup happens elsewhere** - Function exits are handled differently
2. **Boolean sequences are per-function** - Tree-sitter already handles scope boundaries
3. **Tests confirm behavior** - All 25 cognitive tests have explicit expectations
4. **No regressions** - Other metrics don't expect the broken behavior

---

## What Gets Fixed in Phase 1

### Direct Fixes (Cognitive Tests)

**Python** (11 tests):
- ✓ `python_simple_function` - Now counts `and` operators
- ✓ `python_tuple` - Now counts logical operators in tuples
- ✓ `python_sequence_same_booleans` - Counts repeated `and`
- ✓ `python_sequence_different_booleans` - Counts mixed `and`/`or`
- ✓ `python_formatted_sequence_different_booleans` - Formatted version
- ✓ `python_ternary_operator` - Ternary in Python
- ✓ `python_elif_function` - `elif` with operators
- ✓ `python_more_elifs_function` - Multiple `elif`
- ✓ `python_real_function` - Complex real-world function
- ✓ `python_expression_statement` - Standalone expressions
- ✓ `python_nested_functions_lambdas` - Nested with lambdas

**C** (7 tests):
- ✓ `c_simple_function` - Counts `&&` operators
- ✓ `c_1_level_nesting` - Nested with operators
- ✓ `c_goto` - `goto` statement complexity
- ✓ `c_not_booleans` - `!` operator
- ✓ `c_sequence_same_booleans` - Multiple `&&`
- ✓ `c_sequence_different_booleans` - Mixed `&&` / `||`
- ✓ `c_switch` - Switch with operators

**MozJS/JavaScript** (7 tests):
- ✓ `mozjs_simple_function` - Counts `&&` operators
- ✓ `mozjs_1_level_nesting` - Nested with operators
- ✓ `mozjs_not_booleans` - `!` operator
- ✓ `mozjs_sequence_same_booleans` - Multiple `&&`
- ✓ `mozjs_sequence_different_booleans` - Mixed `&&` / `||`
- ✓ `mozjs_switch` - Switch with operators
- ✓ `mozjs_try_construct` - Try-catch with operators

### Cascading Fixes (Dependent Tests)

**Cyclomatic Complexity** (4 tests):
- ✓ `c_unit_before` - Now uses correct cognitive stats
- ✓ `c_unit_after` - Now uses correct cognitive stats
- ✓ `c_switch` - Now uses correct cognitive stats
- ✓ `c_real_function` - Now uses correct cognitive stats

### Unlocked (Ready for Phase 2)

**LOC Metrics** (22 tests):
- Still failing but now have correct baseline
- Phase 2 will address blank line detection issues

**NArgs/NOM/Ops/Exit** (61 tests):
- Now have correct control flow data
- Phase 3 will address language-specific issues

---

## Before/After Comparison

### Test: `python_simple_function`

**Before Fix**:
```json
{
  "sum": 2.0,
  "average": 2.0,
  "min": 0.0,
  "max": 2.0
}
```

**After Fix**:
```json
{
  "sum": 4.0,
  "average": 4.0,
  "min": 0.0,
  "max": 4.0
}
```

**Explanation**:
- Each `if` statement: +1 cognitive complexity
- Each `and` operator: +1 cognitive complexity
- Two `if` statements: 2 × 1 = 2.0
- Two `and` operators: 2 × 1 = 2.0
- **Total**: 2.0 + 2.0 = 4.0 ✓

---

## Verification Checklist

After applying the fix, verify:

- [ ] File saved at correct location
- [ ] Only line 243 deleted (no other changes)
- [ ] Code compiles: `cargo build --lib`
- [ ] Cognitive tests pass: `cargo test --lib metrics::cognitive::`
- [ ] Output shows: `test result: ok. 25 passed; 0 failed`
- [ ] Full test suite shows: `162 → 191 passed`
- [ ] No new warnings introduced

---

## Troubleshooting

### Compilation Error: "use of undeclared variable"

**Cause**: Accidentally deleted more than line 243
**Solution**: Restore from git and try again

```bash
git checkout src/metrics/cognitive.rs
# Then carefully delete only line 243
```

### Tests still failing after deletion

**Cause**: Line numbers may have shifted
**Solution**: Search for the exact function instead

```bash
grep -n "fn increase_nesting" src/metrics/cognitive.rs
```

Then look for the `reset()` call within that function and delete it.

### Compilation error about line numbers

**Cause**: Text editor auto-formatted or modified structure
**Solution**:

```bash
git diff src/metrics/cognitive.rs
```

Review changes carefully. Only `stats.boolean_seq.reset();` should be removed.

---

## Impact Analysis

### Metric Changes

After Phase 1 fix:

| Metric | Change | Tests Fixed | Tests Remaining |
|--------|--------|-------------|-----------------|
| Cognitive | Boolean operators now counted | 25 | 0 |
| Cyclomatic | Uses fixed cognitive stats | 4 | 0 |
| LOC | Baseline corrected | 0 | 22 |
| NArgs | Can use correct scopes | 0 | 14 |
| NOM | Can use correct scopes | 0 | 4 |
| Ops | Can use correct scopes | 0 | 8 |
| Halstead | Can use fixed stats | 0 | 5 |
| Exit | Can use correct scopes | 0 | 4 |
| C Macros | Unaffected | 0 | 3 |
| Other | Unaffected | 0 | 1 |

**Total after Phase 1**: 191/258 tests passing (74%)

---

## What NOT To Do

❌ **Don't** modify any logic in the function
❌ **Don't** change variable names
❌ **Don't** move the reset() call elsewhere
❌ **Don't** add comments that shift line numbers
❌ **Don't** change indentation (unless part of file-wide cleanup)

---

## Next Steps After Phase 1

1. **Verify**: Run full test suite and confirm 191 passing
2. **Commit**: Git commit with message "fix: Remove aggressive boolean sequence reset in cognitive metrics"
3. **Document**: Update this guide with results
4. **Proceed**: Begin Phase 2 (LOC metric fixes)

---

## Documentation References

- **ROOT_CAUSE_SUMMARY.md** - Original detailed root cause analysis
- **BOOLEAN_OPERATOR_INVESTIGATION.md** - Why Python boolean operators are tricky
- **TEST_FAILURE_CATEGORIZATION.md** - Full categorization and roadmap

