# Wave 52: Problem 323 Optimization Analysis

## Objective
Achieve ≥5% wall-clock speedup for `rust/solutions/src/bin/p323.rs`

## Result
**NO-WIN**: No consistent, reproducible speedup achieved.

## Analysis

### Baseline Performance
- Current implementation: ~0.7-0.9ms per run (highly variable due to short runtime)
- C reference: ~0.8ms per run
- Algorithm: Dynamic programming with binomial coefficients, computing expected steps

### Optimization Attempts

1. **Algebraic Simplification**
   - Changed `(1.0 + sum) / (1.0 - p_stay)` to `(1.0 + sum * inv) * multiplier`
   - Result: No improvement, sometimes 2-3% slower

2. **unsafe get_unchecked**
   - Eliminated bounds checking in hot loops
   - Result: No consistent improvement, high variance

3. **Type Changes**
   - Tried u32 instead of i64 for binomial coefficients
   - Result: Slightly slower

4. **Pre-computation**
   - Pre-computed inverse powers of 2
   - Result: No measurable difference

### Why No Improvement?

1. **Already Highly Optimized Cargo.toml**
   ```toml
   [profile.release]
   opt-level = 3
   lto = "fat"
   codegen-units = 1
   ```
   These aggressive flags mean LLVM is already doing sophisticated optimizations.

2. **Very Short Runtime**
   - Problem only iterates to BITS=32
   - Runtime ~0.7ms dominated by startup/I/O overhead
   - Measurement noise exceeds potential gains

3. **Simple Algorithm**
   - Already O(BITS²) which is optimal for this DP approach
   - Only 528 iterations total (binomial + expected value loops)
   - No obvious algorithmic improvement

### Benchmark Variance
Multiple 10k-run tests showed high variability:
- Baseline: 7.9-9.7s (18-22% variance)
- Optimized: 7.8-9.9s (similar variance)
- Difference within measurement noise

## Conclusion
The original implementation of p323 is already well-optimized. The combination of:
- Simple, cache-friendly algorithm
- Aggressive compiler optimizations
- Very short runtime

means that manual micro-optimizations provide no measurable benefit and sometimes harm performance by interfering with compiler optimizations.

**Recommendation**: Keep original implementation unchanged.
