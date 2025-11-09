# Implementation Roadmap - Test Fix Strategy

**Current Status**: 165/251 tests passing (65.7%)
**Goal**: 251/251 tests passing (100%)
**Remaining Failures**: 90 tests across 8 categories

## Failure Summary by Category

### 1. Cognitive Complexity Tests (28 failures) ⚠️ CRITICAL
**Tests Affected**:
- `c_1_level_nesting`, `c_goto`, `c_not_booleans`, `c_sequence_different_booleans`, `c_sequence_same_booleans`, `c_simple_function`, `c_switch`
- `mozjs_1_level_nesting`, `mozjs_not_booleans`, `mozjs_sequence_different_booleans`, `mozjs_sequence_same_booleans`, `mozjs_simple_function`, `mozjs_switch`, `mozjs_try_construct`
- `python_elif_function`, `python_expression_statement`, `python_formatted_sequence_different_booleans`, `python_more_elifs_function`, `python_nested_functions_lambdas`, `python_real_function`, `python_sequence_different_booleans`, `python_sequence_same_booleans`, `python_simple_function`, `python_ternary_operator`, `python_tuple`

**Root Cause**: Boolean sequence reset prevents boolean operators from being counted (known tree-sitter limitation - BooleanOperator nodes not visited in AST)

**Issue**: Tests expect boolean operators to contribute to complexity, but:
1. BooleanOperator nodes are never visited during tree traversal
2. The reset() call prevents them from being counted anyway
3. Test snapshots assume they CAN be counted

**Solution Path**:
- **Option A (Recommended)**: Accept architectural limitation and update test snapshots to NOT expect boolean operator contributions
- **Option B**: Implement text-based boolean operator detection (fragile, not recommended)
- **Option C**: Deep investigation into tree-sitter Python grammar (high effort, uncertain payoff)

**Effort**: 30 min for Option A (just update snapshots), 4+ hours for Option B/C

**Priority**: HIGH - 28 tests, but may be architectural limitation

---

### 2. LOC (Lines of Code) Tests (20 failures)
**Tests Affected**:
- C: `c_blank`, `c_cloc`, `c_lloc`
- C++: `cpp_block_comment_blank`, `cpp_code_line_block_one_line_blank`, `cpp_code_line_end_block_blank`, `cpp_code_line_start_block_blank`, `cpp_for_lloc`, `cpp_lloc`, `cpp_namespace_loc`, `cpp_no_zero_blank`, `cpp_return_lloc`, `cpp_while_lloc`
- JavaScript: `javascript_no_zero_blank`, `javascript_real_loc`, `mozjs_real_loc`
- Python: `python_cloc`, `python_general_loc`, `python_no_blank`, `python_no_zero_blank`, `python_no_zero_blank_more_comments`, `python_real_loc`

**Root Cause**: Blank line and comment line detection logic broken

**Issue**: Example failure showing closures not counted:
```
-  "closures": 1.0,
+  "closures": 0.0,
```

**Solution Path**:
1. Debug LOC metric calculation logic in `src/metrics/loc.rs`
2. Check blank line detection algorithm
3. Verify code/comment line differentiation
4. Test with simple cases first

**Effort**: 2-3 hours to debug and fix all 20 tests

**Priority**: HIGH - 20 tests, likely systematic issue in one module

---

### 3. NArgs (Number of Arguments) Tests (14 failures)
**Tests Affected**:
- C: `c_functions`, `c_single_function`
- C++: `cpp_nested_functions`, `cpp_no_functions_and_closures`, `cpp_single_lambda`
- JavaScript: `javascript_functions`, `javascript_nested_functions`, `javascript_no_functions_and_closures`, `javascript_single_closure`, `javascript_single_function`
- Python: `python_functions`, `python_nested_functions`, `python_single_function`, `python_single_lambda`

**Root Cause**: Function parameter detection not working

**Issue**: Not counting function parameters correctly across all languages

**Solution Path**:
1. Examine `src/metrics/nargs.rs`
2. Check language-specific parameter parsing
3. Verify lambda/closure parameter handling
4. Debug with simple test cases

**Effort**: 2-3 hours to identify and fix root cause

**Priority**: HIGH - 14 tests, common issue across languages

---

### 4. NOM (Number of Methods) Tests (4 failures)
**Tests Affected**:
- `c_nom`, `cpp_nom`, `python_nom`, `arrow_function_debug::test_simple_arrow`

**Root Cause**: Closure/lambda function counting broken

**Issue**: Example from output:
```
-  "closures": 1.0,
+  "closures": 0.0,
```

**Solution Path**:
1. Check closure detection logic in `src/metrics/nom.rs`
2. Verify lambda expression handling
3. Test with closure-containing code

**Effort**: 1-2 hours to fix

**Priority**: MEDIUM - Only 4 tests, but likely same root cause as NArgs

---

### 5. Halstead Metric Tests (5 failures)
**Tests Affected**:
- `cpp_operators_and_operands`
- `java_operators_and_operands`
- `javascript_operators_and_operands`
- `mozjs_operators_and_operands`
- `rust_operators_and_operands`

**Root Cause**: Operator and operand detection not working

**Solution Path**:
1. Review `src/metrics/halstead.rs`
2. Check operator counting logic
3. Verify operand detection
4. Test with simple expressions

**Effort**: 1.5-2 hours to fix

**Priority**: MEDIUM - 5 tests

---

### 6. Exit Points & Cyclomatic Tests (8 failures)
**Tests Affected**:
- Cyclomatic: `c_real_function`, `c_switch`, `c_unit_after`, `c_unit_before`
- Exit: `c_no_exit`, `javascript_no_exit`, `python_more_functions`, `python_nested_functions`

**Root Cause**: Return statement and exit point detection broken

**Solution Path**:
1. Check `src/metrics/exit.rs` and `src/metrics/cyclomatic.rs`
2. Verify return statement detection
3. Test with function examples
4. Check for language-specific handling

**Effort**: 1.5-2 hours to fix both metrics

**Priority**: MEDIUM - 8 tests combined

---

### 7. OPS Operator Tests (8 failures)
**Tests Affected**:
- `cpp_function_ops`, `cpp_ops`
- `java_ops`
- `javascript_function_ops`, `javascript_ops`
- `mozjs_function_ops`, `mozjs_ops`
- `rust_function_ops`, `rust_ops`

**Root Cause**: Operator counting in function scope broken

**Solution Path**:
1. Review `src/metrics/ops.rs`
2. Check scope-specific operator counting
3. Verify function vs file-level counting
4. Language-specific operator definitions

**Effort**: 1.5-2 hours to debug

**Priority**: MEDIUM - 8 tests

---

### 8. Macro Tests (3 failures)
**Tests Affected**:
- `test_fn_macros`, `test_fn_macros_cpp`, `test_fn_qm_try_inspect_cpp`

**Root Cause**: C/C++ macro parameter detection broken

**Solution Path**:
1. Review `src/c_langs_macros.rs`
2. Check macro parameter extraction
3. Verify function-like macro handling
4. Test with simple macro examples

**Effort**: 1-1.5 hours to fix

**Priority**: LOW - 3 tests, only C/C++

---

### 9. Spaces Test (1 failure)
**Tests Affected**:
- `c_scope_resolution_operator`

**Root Cause**: C++ scope resolution operator handling

**Solution Path**:
1. Check scope resolution operator parsing
2. Test with C++ namespace code

**Effort**: 0.5 hour to fix

**Priority**: LOW - Only 1 test

---

## Recommended Implementation Order

### Phase 1: High-Impact Fixes (Estimated: 6-8 hours)

1. **Start**: Cognitive Complexity (28 tests)
   - Analyze test expectations vs current behavior
   - Decide on boolean operator handling strategy
   - Update snapshots or implement workaround
   - **Time**: 30-60 min

2. **LOC Metrics** (20 tests)
   - Debug blank line detection in `loc.rs`
   - Fix code/comment line differentiation
   - Fix with comprehensive testing
   - **Time**: 2-3 hours

3. **NArgs Metrics** (14 tests)
   - Debug parameter detection in `nargs.rs`
   - Test across all language variants
   - Fix root cause
   - **Time**: 2-3 hours

### Phase 2: Medium-Impact Fixes (Estimated: 3-4 hours)

4. **NOM Metrics** (4 tests)
   - Fix closure/lambda counting
   - **Time**: 1-2 hours

5. **Halstead Metrics** (5 tests)
   - Fix operator/operand counting
   - **Time**: 1.5-2 hours

6. **Exit/Cyclomatic** (8 tests)
   - Fix return statement and exit point detection
   - **Time**: 1.5-2 hours

### Phase 3: Low-Impact Fixes (Estimated: 1-2 hours)

7. **OPS Metrics** (8 tests)
   - Fix operator counting in function scope
   - **Time**: 1.5-2 hours

8. **Macros** (3 tests)
   - Fix C/C++ macro parameter handling
   - **Time**: 1-1.5 hours

9. **Spaces** (1 test)
   - Fix C++ scope resolution
   - **Time**: 0.5 hour

---

## Critical Success Factors

### ✅ Must Have:
1. Keep all currently passing tests (165) passing
2. Fix at least one major category completely before moving to next
3. Comprehensive testing after each fix
4. Document any architectural limitations discovered

### ⚠️ High Risk Areas:
1. **Cognitive Complexity** - May have unfixable tree-sitter limitation
2. **LOC Metrics** - Affects 20 tests, could be systemic issue
3. **NArgs Metrics** - Affects 14 tests, likely simple fix but impacts many

### 📊 Success Metrics:
- Reach 200/251 (79.7%) = High progress
- Reach 225/251 (89.6%) = Approaching completion
- Reach 251/251 (100%) = Full success

---

## Key Files to Review

```
src/metrics/
├── cognitive.rs       - Complex boolean operator logic
├── loc.rs            - Line counting (20 failing tests)
├── nargs.rs          - Parameter detection (14 failing tests)
├── nom.rs            - Method/closure counting (4 failing tests)
├── halstead.rs       - Operator/operand counting (5 failing tests)
├── exit.rs           - Exit point detection (4 failing tests)
├── cyclomatic.rs     - Cyclomatic complexity (4 failing tests)
└── ops.rs            - Operator counting (8 failing tests)

src/
├── c_langs_macros.rs - C/C++ macros (3 failing tests)
└── spaces.rs         - Scope resolution (1 failing test)
```

---

## Next Steps

1. **Immediately**: Review Cognitive tests to understand boolean operator issue
2. **Then**: Focus on LOC metrics as it has highest test count
3. **Parallel**: Debug NArgs as it affects multiple languages
4. **Finally**: Address remaining lower-impact metrics

---

## Commitment Timeline

- **Target**: 100% pass rate
- **Realistic Estimate**: 10-12 hours of focused debugging
- **Risk**: Cognitive complexity may have unfixable limitations
- **Contingency**: If Cognitive cannot be fixed, focus on other 62 tests

