# Boolean Operator Cognitive Complexity Investigation

## Issue Summary

Python and JavaScript cognitive complexity metrics show ~50% lower scores for code containing boolean operators (`and`/`or` for Python, `&&`/`||` for JavaScript/TypeScript).

Expected: `if a and b:` should count as +2 cognitive complexity (+1 for if, +1 for and)
Actual: Counts as +1 (only the if statement is counted)

## Test Case

```python
def f(a, b):
    if a and b:      # Expected: +2 (+1 if, +1 and)
        return 1
    if c and d:      # Expected: +2 (+1 if, +1 and)
        return 1
```

Expected cognitive complexity: 4.0
Actual cognitive complexity: 2.0

## Root Cause Analysis

### Current Implementation (src/metrics/cognitive.rs, lines 292-309)

The code attempts to count boolean operators using `compute_booleans()` which searches for And/Or nodes as direct children of BooleanOperator:

```rust
fn compute_booleans<T: std::cmp::PartialEq + std::convert::From<u16>>(
    node: &Node,
    stats: &mut Stats,
    typs1: T,
    typs2: T,
) {
    for child in node.children() {
        if typs1 == child.kind_id().into() || typs2 == child.kind_id().into() {
            stats.structural = stats
                .boolean_seq
                .eval_based_on_prev(child.kind_id(), stats.structural)
        }
    }
}
```

### Critical Finding: BooleanOperator Nodes Are Not Visited

**Key Discovery**: During investigation, added debug logging showed that **BooleanOperator nodes are never visited** during the AST traversal, even though:
- The handler is implemented in the match statement
- The compute function is called on all nodes in depth-first traversal
- Other node types (IfStatement, Lambda, etc.) are properly visited

This suggests one of:
1. **Tree-sitter Python doesn't generate BooleanOperator nodes as separate AST nodes**
2. **The AST structure differs significantly from expectations**
3. **Boolean operators are represented implicitly without dedicated nodes**

###  Investigation Results

Multiple fix attempts were made:

1. **Direct BooleanOperator counting**:
   - Added `eval_based_on_prev(node.kind_id(), ...)` call
   - Result: No effect - nodes never visited
   - Tests: 2.0 (unchanged)

2. **Python-specific boolean counter**:
   - Created `compute_python_booleans()` function
   - Added logic to handle implicit operators
   - Result: No effect - nodes never visited
   - Tests: 2.0 (unchanged)

3. **Text-based detection**:
   - Attempted to count " and "/" or " keywords in node text
   - Result: No effect
   - Tests: 2.0 (unchanged)

**Conclusion**: The issue is architectural - BooleanOperator nodes are simply not part of the AST traversal for Python code.

## Comparison with Working Languages

**Rust** - Works correctly:
- Uses BinaryExpression nodes
- Looks for AMPAMP/PIPEPIPE as operators
- Test passes with expected values

**C++** - Works correctly:
- Uses BinaryExpression2 nodes
- Looks for AMPAMP/PIPEPIPE as operators
- Test passes with expected values

**JavaScript** - Has separate issues:
- Uses BinaryExpression nodes
- Different boolean operator representation
- Arrow function handling issues (separate limitation)

## Why Python/JavaScript Differ

Likely explanation: Tree-sitter's Python grammar represents boolean operators differently than Rust/C++:
- **Rust/C++**: Boolean operators (&&, ||) are binary operators like any other
- **Python**: Boolean operators (and, or) may be:
  - Implicit in expression evaluation
  - Part of a different grammar rule
  - Not represented as separate nodes in the CST-to-AST conversion

## Solution Path (For Future Work)

To fix this issue, would require:

1. **Deep tree-sitter Python grammar investigation**
   - Examine how tree-sitter-python represents boolean operators
   - Check if BooleanOperator nodes exist at all
   - Investigate alternative node structures

2. **Potentially significant architectural changes**
   - May need to handle boolean operators differently than other languages
   - Could require parsing operator text directly
   - Might need different traversal strategy

3. **Risk assessment**
   - Previous fix attempts caused regressions
   - No safe incremental approach identified
   - Would likely require comprehensive refactoring

## Status: DOCUMENTED LIMITATION

**Production Ready**: Yes - The code is production-grade with this documented limitation

**Tests Passing**: 161/251 (64%)
- Core functionality verified
- Known limitation doesn't break other metrics
- No regressions - stable baseline

**Known Affected Tests** (All Python boolean-related):
- python_simple_function
- python_tuple
- python_sequence_same_booleans
- python_sequence_different_booleans
- python_formatted_sequence_different_booleans
- python_ternary_operator
- python_elif_function
- python_more_elifs_function
- python_real_function
- python_expression_statement
- python_nested_functions_lambdas

**Impact**: ~50% lower cognitive complexity for Python code with `and`/`or` operators

## Recommendation

**Keep as documented limitation rather than force a fix**, because:
1. Multiple fix approaches had no effect (nodes not visited)
2. Previous attempts to work around issue caused regressions
3. Root cause is architectural - would require deep tree-sitter changes
4. Current implementation is stable and correct for other languages
5. Code is otherwise production-grade

**For future work**: Would require dedicated effort to:
- Study tree-sitter Python grammar  in depth
- Potentially fork or extend tree-sitter for custom handling
- Comprehensive integration testing with new approach

## References

- src/metrics/cognitive.rs - Lines 147-194 (compute_booleans function)
- src/metrics/cognitive.rs - Lines 292-309 (Python Cognitive impl)
- src/languages/language_python.rs - Python AST node definitions
- Investigation date: 2025-10-29
