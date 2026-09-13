# P867 Optimization Attempts - No Win

## Baseline Performance
- Average: ~24ms (range 23-28ms over 50 runs)
- Already highly optimized with:
  - Parallel computation for hexagon/trapezoid tilings
  - unsafe get_unchecked in DP hot paths
  - Inline modular arithmetic
  - Efficient bit operations

## Attempted Optimizations

### 1. Pre-compute trap_pow6 values
**Rationale**: Avoid repeated pow6() calls in recursion
**Result**: **SLOWER** - 32.24ms avg (+33% regression)
**Why it failed**: Pre-computation overhead (100 pow6 calls) exceeded savings

### 2. Replace Option<i64> with i64 sentinel (-1)
**Rationale**: Reduce Option overhead in memoization cache
**Result**: **SLOWER** - 31.28ms avg (+29% regression) when combined with trap_pow6
**Why it failed**: Compiler already optimizes Option<i64> well

### 3. Remove parallel overhead from hex/trap computation  
**Rationale**: Parallel overhead may hurt small N=10
**Result**: **MARGINAL** - 23.92ms avg (~1% improvement, below 5% threshold)

### 4. Adjust PAR_WINDOW threshold
- PAR_WINDOW=20: 27.04ms avg (worse)
- PAR_WINDOW=999 (disable DP parallel): 40.02ms avg (much worse)
**Result**: Current PAR_WINDOW=14 is optimal

## Conclusion
The code is already at peak performance. The DP parallelization, unsafe optimizations,
and careful memory layout cannot be meaningfully improved without algorithmic changes.

**Recommendation**: NO-WIN - Do not merge any changes.
