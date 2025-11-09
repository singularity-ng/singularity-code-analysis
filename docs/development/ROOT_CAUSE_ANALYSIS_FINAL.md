# Root Cause Analysis - 90 Test Failures

## Summary

After detailed analysis, the 90 test failures across all metric categories (Cognitive, LOC, NArgs, Cyclomatic, Halstead, Exit, NOM) stem from **systemic issues in the metrics computation and finalization architecture**.

## Key Findings

### Finding 1: Stats Initialization to usize::MAX

**Location**: src/metrics/*.rs (all metric types)

**Pattern**:
```rust
pub struct Stats {
    some_min: usize,
    some_max: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            some_min: usize::MAX,  // ← Problem: initialized to MAX
            some_max: 0,
        }
    }
}
```

**Issue**: Min fields initialized to `usize::MAX` (18446744073709552000 when converted to f64)

**Tests Showing This**: All LOC, NArgs, Cyclomatic tests show this value in min/max fields

**Root Cause**: When finalization doesn't properly update these fields, they stay at initialized values

### Finding 2: Metric Computation Architecture

**Location**: src/spaces.rs lines 326-338

**Current Flow**:
```rust
// For EVERY node in the AST:
if let Some(state) = state_stack.last_mut() {
    T::Cognitive::compute(&node, ...);
    T::Cyclomatic::compute(&node, ...);
    T::Halstead::compute(&node, ...);
    T::Loc::compute(&node, ...);
    T::Nom::compute(&node, ...);
    T::NArgs::compute(&node, ...);  // ← Called for EVERY node
    T::Exit::compute(&node, ...);
    // ... more metrics
}
```

**Problem**: Each metric's compute() method is called for EVERY node, and the Stats are updated individually. There's no intermediate finalization or reset between function processing.

### Finding 3: NArgs Metric Overwriting

**Location**: src/metrics/nargs.rs lines 215-230

**Current Logic**:
```rust
impl NArgs for CppCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if Self::is_func(node) {
            // Count arguments and store in stats.fn_nargs
            compute_args::<Self>(&new_node, &mut stats.fn_nargs);  // ← OVERWRITES
            return;
        }
        // ...
    }
}
```

**Issue**: Each function call OVERWRITES `stats.fn_nargs` instead of accumulating

**Expected Behavior**: Should accumulate or properly finalize before overwriting

**Example**:
- Process function1: `fn_nargs = 2` → Should save this value
- Process function2: `fn_nargs = 3` → Overwrites the `2`!
- Result: Only function2's arg count is remembered

### Finding 4: Finalization Timing

**Location**: src/spaces.rs lines 354-359

**Current Code**:
```rust
finalize::<T>(&mut state_stack, usize::MAX);  // ← Called once at the END

state_stack.pop().map(|mut state| {
    state.space.name = path.to_str().map(|name| name.to_string());
    state.space
})
```

**Issue**: Finalization happens only ONCE at the very end, after ALL nodes have been processed

**Problem**: By then, all the individual metrics have overwritten each other multiple times

**Expected Behavior**: Should either:
1. Finalize metrics AFTER each function is processed
2. Or accumulate properly during compute() without overwriting

### Finding 5: Example Failure Walkthrough

**Test Code**:
```c
// C code with 2 functions
void func1() { }
void func2(int a, int b) { }
```

**Execution**:
1. Parse creates AST nodes
2. For each node, NArgs::compute() is called
3. When node=func1: `stats.fn_nargs = 0` (no args)
4. When node=func2: `stats.fn_nargs = 2` (overwrites!)
5. Finalize called: `compute_minmax()` sees only fn_nargs=2
6. Result: Test expects 2 functions with total of 2 args, but gets... (incorrect computation)

## Why All Test Categories Fail

The same architectural pattern repeats across all metrics:

| Metric | Issue | Symptom |
|--------|-------|---------|
| NArgs | Overwriting fn_nargs for each function | Low/zero counts |
| LOC | Not resetting line counts between functions | Incorrect line tallies |
| Cyclomatic | Not accumulating properly | 0.0 or wrong values |
| Cognitive | Boolean reset destroying context | 0.0 complexity |
| Halstead | Operator/operand counts not persisted | Incorrect ratios |
| Exit | Return tracking not finalized | Missing exit points |
| NOM | Closure/method counts overwritten | 0.0 counts |

## Solutions Required

### Solution 1: Fix Metric Accumulation
Instead of overwriting, metrics should:
```rust
// BEFORE (wrong):
stats.fn_nargs = count_args(node);  // Overwrites!

// AFTER (correct):
stats.fn_nargs_sum += count_args(node);  // Accumulate!
stats.fn_nargs = count_args(node);       // Save for min/max
```

### Solution 2: Per-Function Finalization
After processing each function, call finalization:
```rust
if is_function_end(node) {
    stats.compute_minmax();  // Finalize this function's metrics
    stats.fn_nargs = 0;      // Reset for next function
    stats.closure_nargs = 0;
}
```

### Solution 3: Fix Stats Initialization
Min/max fields should be smarter:
```rust
// Option A: Use Option<usize> instead of usize::MAX
fn_nargs_min: Option<usize>,  // None means "not yet set"

// Option B: Track if any value was seen
has_functions: bool,
fn_nargs_min: usize,  // Only meaningful if has_functions=true
```

### Solution 4: Separate Accumulation from Finalization
```rust
// During AST traversal:
match node {
    Function => {
        let args = count_args(node);
        stats.function_arg_counts.push(args);  // Store separately
    }
}

// During finalization:
for count in stats.function_arg_counts {
    stats.fn_nargs_min = stats.fn_nargs_min.min(count);
    stats.fn_nargs_max = stats.fn_nargs_max.max(count);
    stats.fn_nargs_sum += count;
}
```

## Testing Evidence

### Test: metrics::nargs::tests::c_single_function

**Expected**:
```json
{
  "total_functions": 2.0,
  "average_functions": 2.0,
  "functions_min": 0.0,
  "functions_max": 2.0
}
```

**Actual**:
```json
{
  "total_functions": 0.0,
  "average_functions": 0.0,
  "functions_min": 18446744073709552000.0,
  "functions_max": 0.0
}
```

**Analysis**:
- "total_functions" should be the SUM of all function argument counts
- Shows 0.0 because fn_nargs_sum was never properly accumulated
- "functions_min" shows usize::MAX because finalization didn't update it

### Test: metrics::loc::tests::c_blank

**Expected**:
```json
{
  "sloc": 3.0,
  "ploc": 2.0,
  "lloc": 2.0,
  "sloc_min": 3.0
}
```

**Actual**:
```json
{
  "sloc": 1.0,
  "ploc": 0.0,
  "lloc": 0.0,
  "sloc_min": 18446744073709552000.0
}
```

**Analysis**:
- Line counts are too low (1.0 instead of 3.0)
- Min shows usize::MAX instead of actual value
- Suggests LOC computation is missing lines OR resetting incorrectly

## Recommendations for Next Session

### Priority 1 (Critical)
1. Choose ONE metric (e.g., NArgs) as a test case
2. Trace through a simple test case with detailed logging
3. Understand where values are being lost/overwritten
4. Implement solution for that ONE metric
5. Verify all tests for that metric pass
6. Replicate solution for other metrics

### Priority 2 (Important)
1. Fix Stats initialization to handle edge cases
2. Consider separating data collection from finalization
3. Add unit tests for stats accumulation

### Priority 3 (Nice to have)
1. Refactor metric architecture for consistency
2. Add comprehensive documentation
3. Consider trait-based approach for all metrics

## Files to Modify

**For NArgs**:
- src/metrics/nargs.rs (compute() and accumulation logic)
- src/spaces.rs (finalization logic)

**For LOC**:
- src/metrics/loc.rs (line counting logic)
- src/spaces.rs (finalization logic)

**For All Metrics**:
- src/spaces.rs (compute_minmax, finalize functions)
- src/metrics/*.rs (Stats Default implementations)

## Timeline Estimate

| Task | Time | Priority |
|------|------|----------|
| Fix NArgs | 2-3 hours | Critical |
| Fix LOC | 2-3 hours | Critical |
| Fix Cognitive | 1-2 hours | Important |
| Fix remaining metrics | 3-4 hours | Important |
| **Total** | **8-12 hours** | - |

## Conclusion

The 90 test failures are not scattered issues but symptoms of a single architectural problem: **metrics aren't being properly accumulated and finalized during the AST traversal**.

Fixing this requires understanding the compute pass vs. finalization pass, and ensuring that:
1. Metrics are properly accumulated for each scope
2. Finalization properly converts accumulated values to min/max/average
3. Values are reset appropriately between functions/scopes

Once this core issue is fixed, most or all 90 tests should pass.
