# P857 Optimization Attempt Summary

## Baseline Performance
- Mean: ~98-100 ms
- Median: ~98 ms  
- Variance: 2-3 ms stdev (relatively low variance)

## Optimizations Attempted

### 1. U128 Deferred Modular Reduction
**Result**: -5.36% (REGRESSION)
- Accumulated terms in u128 before taking mod
- CLAUDE.md warned about this: u128 overhead dominates for MOD < 2^32
- Confirmed: u128 casting overhead outweighs reduced mod operations

### 2. Array Rotation with `copy_within`
**Result**: -0.44% (NO IMPROVEMENT)
- Replaced manual array shifts with optimized `copy_within`
- Compiler was already optimizing the manual shifts well

### 3. Pre-computed n_prod Values
**Result**: ~0-2% (MINIMAL/INCONSISTENT)
- Broke dependency chain by pre-computing all n_prod values
- Slight improvement in some runs, regression in others
- Within measurement noise

### 4. Unsafe `get_unchecked`
**Result**: -1.61% with 50-run benchmark (SLIGHT REGRESSION)
- Initial 15-run test showed +2.34%, but not reproducible
- Comprehensive 50-run test showed slight regression
- Suggests compiler was already eliminating bounds checks

### 5. Combined Optimizations
**Result**: WORSE than individual attempts
- Combining multiple micro-optimizations increased complexity
- Likely inhibited compiler optimizations

## Analysis

The original code is already well-optimized:
1. **Simple, tight loop**: 10M iterations of straightforward arithmetic
2. **Compiler-friendly**: Regular access patterns, predictable branches
3. **No obvious bottlenecks**: No allocations, no complex data structures
4. **Modern LLVM**: Already doing bounds-check elimination, loop optimization

The computation is fundamentally:
- 5 modular multiplications per iteration (for n_prod updates)
- 5 modular multiplications for value computation
- 4 array element moves
- ~100 million modular operations total

With modular arithmetic dominating (unavoidable for correctness) and a cache-friendly access pattern, there's limited room for optimization without changing the algorithm itself.

## Conclusion

**HONEST NO-WIN**: The original implementation is already near-optimal for this algorithm. Attempted micro-optimizations either:
- Added overhead (u128 casting)
- Were already done by the compiler (bounds check elimination)  
- Had no measurable effect within variance

To achieve ≥5% improvement would likely require:
- Algorithmic changes (different recurrence formulation)
- SIMD vectorization (difficult for this sequential recurrence)
- Assembly-level optimization (not maintainable)
