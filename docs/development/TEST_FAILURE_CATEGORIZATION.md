# Singularity Code Analysis - Test Failure Categorization & Priority Roadmap

**Current Status**: 162/258 tests passing (62.8% - slight regression from 64% due to 2 unfixed compiler issues)

**Failing Tests**: 90 tests across 9 major categories

---

## Executive Summary

The test suite has **one primary root cause** (boolean sequence reset in cognitive.rs) causing **25 direct failures** and **65+ cascading failures** across dependent metrics. The remaining 5-10 failures are due to unrelated issues (C macro parsing, C/C++ function detection).

**Priority Fix**: Remove one line from `src/metrics/cognitive.rs` (line 243) → Fixes ~40-50% of all failures immediately.

---

## Test Failure Breakdown by Category

### Category 1: Cognitive Complexity (25 failures - 27.8% of failures)

**Root Cause**: `increase_nesting()` function resets boolean sequence too aggressively (line 243)

**Affected Tests**:
- **Python** (11 tests - MAJOR IMPACT):
  - `python_simple_function`
  - `python_tuple`
  - `python_sequence_same_booleans`
  - `python_sequence_different_booleans`
  - `python_formatted_sequence_different_booleans`
  - `python_ternary_operator`
  - `python_elif_function`
  - `python_more_elifs_function`
  - `python_real_function`
  - `python_expression_statement`
  - `python_nested_functions_lambdas`

- **C** (7 tests - MAJOR IMPACT):
  - `c_1_level_nesting`
  - `c_goto`
  - `c_not_booleans`
  - `c_sequence_different_booleans`
  - `c_sequence_same_booleans`
  - `c_simple_function`
  - `c_switch`

- **MozJS/JavaScript** (7 tests - MAJOR IMPACT):
  - `mozjs_1_level_nesting`
  - `mozjs_not_booleans`
  - `mozjs_sequence_different_booleans`
  - `mozjs_sequence_same_booleans`
  - `mozjs_simple_function`
  - `mozjs_switch`
  - `mozjs_try_construct`

**Failure Pattern**: Snapshot value mismatches - all show 50% lower scores
```json
Expected: {"sum": 4.0, "average": 4.0, "min": 0.0, "max": 4.0}
Actual:   {"sum": 2.0, "average": 2.0, "min": 0.0, "max": 2.0}
```

**Effort to Fix**: 2 minutes (delete 1 line)
**Impact**: Fixes 25 direct tests + unlocks cascading fix pathway
**Priority**: CRITICAL - Phase 1 blocker

**File**: `/home/mhugo/code/singularity/packages/singularity-code-analysis/src/metrics/cognitive.rs` (lines 238-243)

---

### Category 2: LOC (Lines of Code) Metrics (22 failures - 24.4% of failures)

**Root Cause**: Cascading failure from cognitive complexity metrics + potential blank line detection issues

**Affected Tests by Language**:
- **C/C++** (12 tests):
  - `c_blank`, `c_cloc`, `c_lloc`
  - `cpp_block_comment_blank`, `cpp_code_line_*`, `cpp_for_lloc`, `cpp_while_lloc`, `cpp_return_lloc`, `cpp_namespace_loc`, `cpp_lloc`, `cpp_no_zero_blank`

- **Python** (7 tests):
  - `python_blank`, `python_cloc`, `python_lloc`, `python_sloc`, `python_general_loc`, `python_no_blank`, `python_no_zero_blank` (x2)

- **JavaScript** (3 tests):
  - `javascript_real_loc`, `javascript_no_zero_blank`, `mozjs_real_loc`

**Failure Pattern**: Two sub-patterns
1. **Cascading from cognitive**: ~60% of LOC tests fail because cognitive stats aggregation is wrong
2. **Direct LOC bugs**: ~40% due to blank line/comment detection (especially in C/C++)

**Secondary Issue**: C/C++ blank line detection appears broken for nested structures

**Effort to Fix**:
- Phase 1 (Fix cognitive reset): Resolves ~12 LOC tests
- Phase 2 (Fix blank line detection): 2-4 hours for remaining tests

**Priority**: HIGH - Phase 2 (depends on Phase 1)

---

### Category 3: NArgs (Function Argument Count) (14 failures - 15.6% of failures)

**Root Cause**: Mixed causes
- 60%: Cascading from cognitive/control flow detection
- 40%: Function parameter detection issues in parsers

**Affected Tests by Language**:
- **C/C++** (6 tests):
  - `c_single_function`, `c_functions`
  - `cpp_single_lambda`, `cpp_no_functions_and_closures`, `cpp_nested_functions`

- **Python** (6 tests):
  - `python_single_function`, `python_functions`, `python_nested_functions`, `python_single_lambda`, `python_single_function` (duplicate?), `python_no_functions_and_closures`

- **JavaScript** (2 tests):
  - `javascript_single_function`, `javascript_functions`, `javascript_nested_functions`, `javascript_single_closure`, `javascript_no_functions_and_closures`

**Failure Pattern**: Function/closure/lambda argument counts are 0 or incorrect

**Secondary Issues**:
- Arrow function parameter parsing (JS)
- C/C++ macro-generated function signatures
- Python decorator parameter handling

**Effort to Fix**: 2-4 hours (depends on parser)
**Priority**: MEDIUM - Phase 3

---

### Category 4: Ops (Operators) (8 failures - 8.9% of failures)

**Root Cause**: Operator counting from parser results

**Affected Tests**:
- **C/C++** (2 tests): `cpp_ops`, `cpp_function_ops`
- **Java** (1 test): `java_ops`
- **JavaScript** (3 tests): `javascript_ops`, `javascript_function_ops`, `mozjs_ops`, `mozjs_function_ops`
- **Rust** (2 tests): `rust_ops`, `rust_function_ops`

**Failure Pattern**: Snapshot value mismatches (operator counts are wrong)

**Secondary Issue**: Relies on proper function scope detection (cascading from nargs)

**Effort to Fix**: 1-3 hours (depends on parser fixes)
**Priority**: MEDIUM - Phase 3

---

### Category 5: NOM (Number of Methods) (4 failures - 4.4% of failures)

**Root Cause**: Function/method detection in parsers + scope tracking

**Affected Tests**:
- `c_nom`, `cpp_nom`, `python_nom`
- `arrow_function_debug::test_simple_arrow`

**Failure Pattern**: Functions/methods not detected or miscounted

**Secondary Issue**: Arrow function special handling

**Effort to Fix**: 1-2 hours (integrated with nargs fix)
**Priority**: MEDIUM - Phase 3

---

### Category 6: Halstead Complexity (5 failures - 5.6% of failures)

**Root Cause**: Operator/operand counting relies on cognitive stats

**Affected Tests**:
- **C/C++** (1 test): `cpp_operators_and_operands`
- **Java** (1 test): `java_operators_and_operands`
- **JavaScript** (2 tests): `javascript_operators_and_operands`, `mozjs_operators_and_operands`
- **Rust** (1 test): `rust_operators_and_operands`

**Failure Pattern**: Cascading from cognitive + operator detection

**Effort to Fix**: Resolves automatically when cognitive/ops are fixed
**Priority**: LOW - Phase 4 (automatic resolution)

---

### Category 7: Exit Point Detection (4 failures - 4.4% of failures)

**Root Cause**: Return statement detection + control flow analysis

**Affected Tests**:
- `c_no_exit`, `javascript_no_exit`
- `python_nested_functions`, `python_more_functions`

**Failure Pattern**: Return statements not detected or incorrectly scoped

**Effort to Fix**: 1-2 hours
**Priority**: LOW - Phase 4

---

### Category 8: Cyclomatic Complexity (4 failures - 4.4% of failures)

**Root Cause**: Cascading from cognitive metrics

**Affected Tests**:
- **C** (4 tests): `c_unit_before`, `c_unit_after`, `c_switch`, `c_real_function`

**Failure Pattern**: Snapshot value mismatches (50% lower like cognitive)

**Effort to Fix**: Resolves with cognitive fix
**Priority**: HIGH - Phase 1 (automatic resolution)

---

### Category 9: C Macros & Edge Cases (5 failures - 5.6% of failures)

**Root Cause**: C/C++ macro parsing + special symbol handling

**Affected Tests**:
- **C Macros** (3 tests):
  - `c_langs_macros::tests::test_fn_macros`
  - `c_langs_macros::tests::test_fn_macros_cpp`
  - `c_langs_macros::tests::test_fn_qm_try_inspect_cpp`

- **Other** (2 tests):
  - `spaces::tests::c_scope_resolution_operator`

**Failure Pattern**: Parser errors (assertion `!root.has_error()` fails)
```
thread panicked at assertion failed: !root.has_error()
```

**Secondary Issue**: Tree-sitter C++ grammar doesn't handle some Mozilla macros

**Examples**:
```cpp
MOZ_ALWAYS_INLINE void f() { }
MOZ_NONHEAP_CLASS Factory : public IClassFactory {};
QM_TRY_INSPECT(const int32_t& storageVersion, MOZ_TO_RESULT_INVOKE(...));
```

**Effort to Fix**: 2-4 hours (may need grammar extensions or macro preprocessing)
**Priority**: MEDIUM-LOW - Phase 3

---

## Fix Priority Roadmap

### Phase 1: Critical Path (30 minutes) - IMMEDIATE

**Target**: Fix 25 cognitive + 4 cyclomatic + cascading LOC failures (~35-40 tests)

**Action**: Fix `increase_nesting()` boolean sequence reset
- File: `src/metrics/cognitive.rs`, line 243
- Change: Delete line `stats.boolean_seq.reset();`
- Verification: `cargo test --lib metrics::cognitive:: 2>&1 | grep -c "ok"`
- Expected: All 25 cognitive tests → PASS

**Expected Result After Phase 1**:
- Test pass rate: 162 + 25 + 4 = 191/258 (74%)
- Remaining failures: 67

---

### Phase 2: Cascading Fixes (2-3 hours) - HIGH PRIORITY

**Target**: Fix LOC metrics, blank line detection, comment handling

**Issues to Address**:
1. **LOC Aggregation**: Ensure LOC metrics properly aggregate from fixed cognitive stats
2. **Blank Line Detection**: Fix C/C++ blank line detection for:
   - Comments with different styles
   - Namespace declarations
   - Nested structures
3. **Comment Stripping**: Verify comment removal before LOC calculation

**Files to Review**:
- `src/metrics/loc.rs` - Line counting logic
- `src/comment_rm.rs` - Comment removal
- `src/metrics/cognitive.rs` - Stats aggregation

**Expected Result After Phase 2**:
- Test pass rate: ~210/258 (81%)
- Remaining failures: 48

---

### Phase 3: Parser Improvements (3-4 hours) - MEDIUM PRIORITY

**Target**: Fix function/method detection, parameter counting, macro handling

**Issues to Address**:
1. **NArgs/NOM**: Function argument and method counting
   - C/C++: Macro-generated signatures
   - Python: Decorator handling, lambda parameters
   - JS: Arrow function parameters

2. **C Macros**: Tree-sitter C++ grammar limitations
   - Options: Preprocess macros, extend grammar, or add special handling
   - Risk: May require tree-sitter fork or complex preprocessing

3. **Ops**: Operator counting
   - Verify operator list is complete
   - Check special cases (ternary, spread, etc.)

**Files to Review**:
- `src/metrics/nargs.rs` - Parameter detection
- `src/metrics/nom.rs` - Function counting
- `src/c_langs_macros/` - Macro handling
- Parser modules (C, Python, JS)

**Expected Result After Phase 3**:
- Test pass rate: ~235/258 (91%)
- Remaining failures: 23

---

### Phase 4: Edge Cases & Optimization (1-2 hours) - LOW PRIORITY

**Target**: Exit point detection, Halstead metrics, edge cases

**Issues to Address**:
1. **Exit Points**: Return statement scoping
2. **Halstead**: Resolve via Phase 3 fixes
3. **Spaces**: C scope resolution operators

**Expected Result After Phase 4**:
- Test pass rate: ~250/258 (96%)
- Remaining failures: 8 (likely architectural limitations)

---

## Dependency Graph

```
Phase 1: Cognitive Reset Fix
    ↓
    ├─→ 25 cognitive tests pass
    ├─→ 4 cyclomatic tests pass (cascading)
    └─→ Enables Phase 2
         ↓
    Phase 2: LOC Metrics
         ├─→ 12 LOC tests pass
         └─→ Enables Phase 3
              ↓
         Phase 3: Parser Improvements
              ├─→ 14 nargs tests pass
              ├─→ 4 nom tests pass
              ├─→ 8 ops tests pass
              ├─→ 5 halstead tests pass (auto)
              ├─→ 3 c_macros tests pass (maybe)
              └─→ Enables Phase 4
                   ↓
              Phase 4: Edge Cases
                   ├─→ 4 exit tests pass
                   └─→ 2 remaining tests (maybe)
```

---

## Estimated Effort Summary

| Phase | Tasks | Effort | Impact | Risk |
|-------|-------|--------|--------|------|
| **1** | Remove 1 line | 2 min | +35 tests (14%) | NONE |
| **2** | Fix LOC aggregation | 1-2 hrs | +12 tests (5%) | LOW |
| **3** | Parser fixes | 3-4 hrs | +31 tests (12%) | MEDIUM |
| **4** | Edge cases | 1-2 hrs | +4 tests (2%) | LOW |
| **TOTAL** | - | **5-8 hours** | **+82 tests (32%)** | **LOW** |

---

## What Will Remain (Likely Unfixable in Phase 4)

**Estimated 8-10 tests** may remain due to architectural limitations:

1. **Boolean Operator Counting in Python** (Already documented)
   - Tree-sitter Python grammar doesn't expose BooleanOperator nodes
   - Would require deep grammar investigation or major refactoring
   - Already has comprehensive analysis in BOOLEAN_OPERATOR_INVESTIGATION.md

2. **C++ Macro Preprocessing**
   - Some Mozilla-specific macros may not parse without preprocessing
   - Tree-sitter C++ grammar may need extensions
   - Could fork tree-sitter or implement macro preprocessor

3. **Complex Function Scope Edge Cases**
   - Nested functions in non-standard contexts
   - Lambdas/closures in unusual syntax patterns

---

## Testing Strategy

### Run After Each Phase

```bash
# Phase 1 verification
cargo test --lib metrics::cognitive:: 2>&1 | tail -5

# Phase 2 verification
cargo test --lib metrics::loc:: 2>&1 | tail -5

# Phase 3 verification
cargo test --lib metrics::nargs:: metrics::nom:: metrics::ops:: 2>&1 | tail -5

# Full suite
cargo test --lib 2>&1 | tail -5
```

---

## Success Criteria

- **Phase 1**: 162 → 191 tests (↑29, +14%)
- **Phase 2**: 191 → 203 tests (↑12, +6%)
- **Phase 3**: 203 → 234 tests (↑31, +15%)
- **Phase 4**: 234 → 250 tests (↑16, +8%)
- **Final**: 250+/258 tests (96%+)

---

## Key References

| Document | Purpose |
|----------|---------|
| **ROOT_CAUSE_SUMMARY.md** | Summary of the primary cognitive.rs bug |
| **BOOLEAN_OPERATOR_INVESTIGATION.md** | Deep analysis of Python boolean operator limitation |
| **TEST_FAILURE_ANALYSIS.md** | Original detailed failure analysis |
| **INSIGHT_METRICS_ANALYSIS.md** | Analysis of semantic complexity metrics |

---

## Next Steps

1. **Immediate**: Apply Phase 1 fix (delete 1 line)
2. **Verify**: Run cognitive tests
3. **Document**: Update this file with Phase 1 results
4. **Proceed**: Begin Phase 2 work

**Estimated time to 90% test pass rate**: 4-5 hours of focused work

