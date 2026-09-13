# p870 Optimization Attempt - Wave 52

## Goal
Achieve ≥5% speedup on rust/solutions/src/bin/p870.rs

## Baseline Performance
- **Original code:** Mean 0.439s, Median 0.434s (50-run average)
- Already uses aggressive optimizations:
  - Unsafe raw pointers with unchecked indexing
  - Flat pre-allocated buffer (cache-friendly)  
  - u128 for overflow-safe surplus tracking
  - Optimized inner loop with minimal branches
  - Cargo profile: opt-level=3, LTO=fat, codegen-units=1

## Optimization Attempts

### 1. Binary GCD Algorithm
**Change:** Replaced Euclidean GCD with binary (Stein's) algorithm
**Result:** ~4-5% REGRESSION  
**Reason:** Euclidean GCD is faster for typical 64-bit integers on modern CPUs with fast division

### 2. Simplified Decimal Formatting  
**Change:** Replaced String operations with fixed array and manual rounding
**Result:** No measurable difference
**Reason:** Formatting runs once at end; negligible compared to 123,455 main loop iterations

### 3. Inner Loop Micro-optimizations
**Changes:**
- `checked_add` → `overflowing_add`
- `loop { if k >= KMAX break; }` → `while k < KMAX`
- Restructured surplus assignment
**Result:** ~1% REGRESSION
**Reason:** Compiler already optimizes these patterns; manual changes hurt code generation

### 4. Branch Hints (likely/unlikely)
**Changes:** Added core_intrinsics hints around loop conditions  
**Result:** No measurable difference (~0.1% slower)
**Reason:** Modern branch predictors already handle these patterns well

## Statistical Comparison
```
Original:  Mean 0.43854s, Median 0.434s, Best-10-avg 0.4326s
Best attempt: Mean 0.4434s,  Median 0.435s, Best-10-avg 0.4331s
Difference: +1.1% REGRESSION
```

## Conclusion: HONEST NO-WIN

The code is already at a local performance optimum. The algorithm uses:
- O(n) outer iterations (123,455 required for T(123456))
- O(KMAX) inner iterations per outer loop (unavoidable for sequence generation)
- Minimal overhead: raw pointers, unchecked indexing, hoisted multiplications

**Why 5% is unachievable:**
1. Algorithm is already optimal (no redundant work)
2. Implementation is already heavily micro-optimized
3. Compiler with aggressive flags (LTO, single codegen unit) extracts maximum performance
4. No algorithmic insight available (problem requires computing all 123,455 transitions)

**Possible paths forward (beyond scope):**
- SIMD vectorization (unlikely to help due to data dependencies)
- Algorithmic breakthrough (would require mathematical insight into the recurrence)
- Different language/runtime (C/C++ might match but unlikely to beat by 5%)

The existing code represents excellent engineering and is production-ready.
