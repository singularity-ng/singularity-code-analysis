# Test Failures Analysis - singularity-code-analysis

**Status**: 159/251 tests passing (63.3%)
**Goal**: 251/251 tests passing (100%)
**Failures**: 92 tests

## Executive Summary

The test suite has regressed due to a recent commit that removed `stats.boolean_seq.reset()` from the `increase_nesting()` function. While the removal was intended to fix boolean operator counting in Python/JavaScript, it has created systemic issues across the cognitive complexity metrics and related tests.

The core problem is an **architectural limitation**: Tree-sitter's Python grammar does not generate BooleanOperator nodes during AST traversal, making it impossible to count boolean operators within the current tree-walking framework.

## Test Failure Categories

### 1. Cognitive Complexity Tests (30 failures)
**Status**: All expecting boolean operators to be counted (impossible with current tree-sitter)
**Impact**: ~50% of expected values unrealistic
**Files**: `src/metrics/cognitive.rs` (lines 700-1000+)

**Examples**:
- `python_simple_function`: Expects 4.0 (if+and for 2 statements), getting 0.0
- `c_simple_function`: Expects 4.0 (if+and for 2 statements), getting 0.0
- `java_compound_conditions`: Expects values that assume boolean operators are counted

**Root Cause**: Commit 9510b22 ("remove boolean reset bug") removed the `stats.boolean_seq.reset()` call that was preventing boolean operator counting. However, this doesn't help because:
1. BooleanOperator nodes are never visited in Python code (tree-sitter limitation)
2. Without the reset, the boolean sequence carries over between statements incorrectly
3. Removing the reset changed behavior but didn't enable the impossible boolean operator counting

**Solution Options**:
1. **Accept limitation** (Recommended): Update test snapshots to reflect current behavior (no boolean operator counting)
2. **Restore the reset**: Add back `stats.boolean_seq.reset()` to prevent cross-statement contamination
3. **Deep fix**: Implement alternative boolean operator detection (requires text parsing, not AST-based)

### 2. LOC (Lines of Code) Metrics (25+ failures)
**Status**: Blank line detection issues
**Files**: `src/metrics/loc.rs`

**Affected Tests**:
- `c_blank`, `c_cloc`, `c_lloc`
- `cpp_*` (multiple variants)
- `javascript_*` (multiple variants)
- `python_*` (multiple variants)

**Issues**:
- Blank line counting not working correctly
- Code line (lloc) detection broken
- Comment line (cloc) detection broken

### 3. NOM (Number of Methods) Metrics (14+ failures)
**Status**: Nested function/closure counting issues
**Files**: `src/metrics/nom.rs`

**Examples**:
- `cpp_nom`: Getting 0.0, expecting nested functions counted
- `c_nom`: Missing closure detection
- `python_nom`: Lambda function detection broken
- `javascript_*`: Arrow function handling issues

**Root Cause**: Likely related to the same reset issue - lambda/closure context being lost

### 4. NArgs (Number of Arguments) Metrics (12+ failures)
**Status**: Function parameter detection broken
**Files**: `src/metrics/nargs.rs`

**Affected Tests**:
- All language variants failing to detect function parameters correctly
- Both simple functions and nested functions affected

### 5. Halstead Metrics (6+ failures)
**Status**: Operators/operands detection broken
**Files**: `src/metrics/halstead.rs`

**Examples**:
- `cpp_operators_and_operands`
- `java_operators_and_operands`
- `rust_operators_and_operands`

### 6. Exit Points & Cyclomatic (6+ failures)
**Status**: Return statement detection issues
**Files**: `src/metrics/exit.rs`, `src/metrics/cyclomatic.rs`

**Examples**:
- `python_more_functions`: Not counting exit points correctly
- `c_no_exit`: False positives or negatives

### 7. Other Issues (2+ failures)
**Status**: Various
**Files**: `src/metrics/ops.rs`, `src/spaces.rs`, `src/c_langs_macros.rs`

**Examples**:
- `cpp_function_ops`: Operator detection in function scope
- `c_scope_resolution_operator`: C++ scope resolution handling
- `test_fn_macros`: C macro parameter detection

## Recommended Fix Strategy

### Phase 1: Restore Stability (Short-term)
1. **Restore the reset** in `increase_nesting()`:
   ```rust
   fn increase_nesting(stats: &mut Stats, nesting: &mut usize, depth: usize, lambda: usize) {
       stats.nesting = *nesting + depth + lambda;
       increment(stats);
       *nesting += 1;
       stats.boolean_seq.reset();  // <-- Add this back
   }
   ```
2. **Update cognitive test snapshots** to reflect original expected values
3. **Run full test suite** to confirm regression is resolved

### Phase 2: Fix Specific Metrics (Medium-term)
1. **LOC metrics** (25+ tests): Debug blank line detection logic
2. **NOM metrics** (14+ tests): Fix closure/lambda detection
3. **NArgs metrics** (12+ tests): Parameter detection issues
4. **Halstead metrics** (6+ tests): Operator/operand counting

### Phase 3: Address Architectural Limitation (Long-term)
For boolean operator counting in Python/JavaScript:
1. **Option A** (Recommended): Document as limitation, update snapshots
2. **Option B**: Implement text-based detection for boolean operators
3. **Option C**: Investigate tree-sitter grammar to see if nodes exist but aren't visited

## Investigation Notes

### Boolean Operator Investigation (Completed)
- **Finding**: BooleanOperator nodes are not visited in Python AST traversal
- **Evidence**: Added debug logging showed zero visits despite traversal of other node types
- **Approaches tried**: 3 different methods all failed due to nodes not being visited
- **Conclusion**: This is an architectural limitation of tree-sitter Python grammar

### Commit History
- **9510b22**: Removed `stats.boolean_seq.reset()` to "fix boolean operator counting"
  - Intended to help with boolean operators
  - Actually broke multiple other metrics
  - Removed because it was "destroying boolean operator context"
- **Previous commit**: Had the reset in `increase_nesting()`
  - Tests were passing with original snapshots
  - Reset was preventing boolean operators from being counted (impossible anyway)

## Key Files Involved

```
src/metrics/
├── cognitive.rs          - Cognitive complexity (30 tests)
├── loc.rs               - Lines of code (25+ tests)
├── nom.rs               - Number of methods (14+ tests)
├── nargs.rs             - Number of arguments (12+ tests)
├── halstead.rs          - Halstead metrics (6+ tests)
├── exit.rs              - Exit points (3+ tests)
├── cyclomatic.rs        - Cyclomatic complexity (3+ tests)
└── [others]

src/
├── spaces.rs            - Tree walking and metric calculation
├── c_langs_macros.rs    - C/C++ macro handling
└── [others]
```

## Action Items

1. **Immediate** (30 min):
   - [ ] Restore `stats.boolean_seq.reset()` in `increase_nesting()`
   - [ ] Run full test suite
   - [ ] Document the commit that caused regression

2. **Short-term** (2 hours):
   - [ ] Update cognitive test snapshots if needed
   - [ ] Fix LOC metrics blank line detection
   - [ ] Fix NOM metrics closure detection

3. **Medium-term** (4+ hours):
   - [ ] Fix NArgs parameter detection
   - [ ] Fix Halstead operator counting
   - [ ] Fix exit point detection

4. **Long-term**:
   - [ ] Resolve boolean operator limitation
   - [ ] Comprehensive testing framework
   - [ ] Performance benchmarking

## Related Issues

- See `BOOLEAN_OPERATOR_INVESTIGATION.md` for detailed investigation
- See git commit `9510b22` for the regression
- See previous commits for working baseline

## Status Timeline

- **2025-10-29 04:40**: Investigation report created
- **2025-10-29 05:10**: Regression commit (9510b22) applied
- **Current**: 159/251 tests passing (regression confirmed)
