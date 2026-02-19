# Rust Solutions Guide

## Project Structure

```
rust/
  Cargo.toml              # Workspace: euler_utils + solutions
  euler_utils/src/         # Shared library (primes, modular, binomial, crt, etc.)
  solutions/src/bin/pNNN.rs  # One binary per problem
```

Build and run: `cargo run --release --bin pNNN`

## Performance Rules (Learned from Porting 595 C Solutions)

### 1. Never Use Vec<Vec<T>> for Fixed-Size 2D Arrays

Use `Box<[[T; COLS]; ROWS]>` or flat `Vec<T>` with 1D indexing (`i * cols + j`).

```rust
// BAD: pointer indirection per row, cache-hostile
let mut mat: Vec<Vec<i64>> = vec![vec![0; N]; N];

// GOOD: contiguous memory, cache-friendly
let mut mat: Box<[[i64; N]; N]> = Box::new([[0i64; N]; N]);

// GOOD: flat with manual indexing
let mut mat = vec![0i64; rows * cols];
mat[i * cols + j] = val;
```

This was the #1 cause of regressions (P490: 6.3x fix, P189: 12x fix, P150: 3.2x fix).

### 2. Never Use HashMap When Keys Are Bounded Integers

Use `Vec<T>` with direct indexing + an active-index list for sparse iteration.

```rust
// BAD: hashing overhead dominates for small bounded keys
let mut counts: HashMap<usize, i64> = HashMap::new();

// GOOD: flat array + active tracking
let mut vals = vec![0i64; max_key + 1];
let mut active: Vec<usize> = Vec::new();
```

P189: 12x speedup from this change alone.

### 3. Use unsafe get_unchecked in Proven-Safe Hot Loops

When array indices are provably in-bounds (e.g., loop bounds guarantee it), eliminate bounds checks in tight inner loops. Always add a safety comment.

```rust
// In triple-nested loops or DP inner loops:
// SAFETY: i < N, j < N guaranteed by loop bounds; arrays have size N+1
unsafe {
    let v = *arr.get_unchecked(idx);
    *arr.get_unchecked_mut(idx) = new_val;
}
```

P201: 18x speedup. P154: 1.4x speedup.

### 4. Inline Modular Arithmetic in Hot Paths

Do NOT call `euler_utils::mod_pow` in tight loops. The function call + u128 widening in the generic version kills performance. Inline a local version using the narrowest types that work.

```rust
// BAD in hot loop: external call, u128 intermediate
total += mod_pow(base, exp, modulus);

// GOOD: local inline, u64 intermediates when modulus < 2^32
fn pow_mod_local(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    result
}
```

P522: 3.8x, P445: 4.2x, P479: 2.9x from this pattern.

### 5. Avoid String Operations in Hot Paths

Never use `.to_string()`, format!, or string comparison for numeric checks like palindromes. Use arithmetic.

```rust
// BAD: allocates String every iteration
fn is_palindrome(n: u64) -> bool { let s = n.to_string(); s == s.chars().rev().collect::<String>() }

// GOOD: pure arithmetic, zero allocation
fn is_palindrome(mut n: u64) -> bool {
    let original = n;
    let mut rev = 0u64;
    while n > 0 { rev = rev * 10 + n % 10; n /= 10; }
    rev == original
}
```

### 6. Use Deferred Modular Reduction in Matrix Multiply

Don't reduce mod M on every element. Accumulate in i128 and reduce every 16 iterations.

```rust
// In mat_mul inner loop:
let mut acc = [0i128; DIM];
for k in 0..DIM {
    let aik = a[i][k] as i128;
    for j in 0..DIM { acc[j] += aik * b[k][j] as i128; }
    if k % 16 == 15 { for j in 0..DIM { acc[j] %= MOD as i128; } }
}
```

### 7. Pre-allocate and Reuse Buffers

Don't allocate inside loops. Pre-allocate outside and use `std::mem::swap` for double-buffering.

```rust
let mut buf_a = vec![0i64; size];
let mut buf_b = vec![0i64; size];
for step in 0..steps {
    compute(&buf_a, &mut buf_b);
    std::mem::swap(&mut buf_a, &mut buf_b);
}
```

## Rayon Parallelism

`rayon` is available as a workspace dependency. Use it for solutions taking >5s.

### When It Works Well
- Per-iteration work is substantial (>1ms each)
- Work units are independent (no shared mutable state)
- Load is balanced across iterations

### When It Doesn't Work
- Per-iteration work is tiny (array lookups, simple arithmetic) -- overhead exceeds benefit
- P417 was reverted because the summation loop was only ~3s sequential with lightweight iterations

### Patterns

**Simple parallel sum:**
```rust
use rayon::prelude::*;
let total: i64 = (1..=n).into_par_iter().map(|i| compute(i)).sum();
```

**Flatten uneven work for load balancing:**
```rust
// BAD: a=2 does 1000x more work than a=1000
let ans: u64 = (2..=a_max).into_par_iter().map(|a| process(a)).sum();

// GOOD: split each a into fixed-size t-range chunks
let mut work_units: Vec<(u64, u64, u64)> = Vec::new();
for a in (2..=a_max).step_by(2) {
    for chunk in (t_lo..=t_hi).step_by(chunk_size) {
        work_units.push((a, chunk, min(chunk + chunk_size, t_hi)));
    }
}
let ans: u64 = work_units.par_iter().map(|&(a, t0, t1)| process(a, t0, t1)).sum();
```

**Chunked bitmap sieve (cache-friendly):**
```rust
// Use 5M chunks so per-chunk bitmaps fit in L2/L3 cache
let chunk_size = 5_000_000;
let n_chunks = (n + chunk_size - 1) / chunk_size;
let count: usize = (0..n_chunks).into_par_iter().map(|ci| {
    let base = ci * chunk_size;
    let mut bitmap = vec![0u8; chunk_size]; // fits in cache
    // ... sieve this chunk ...
    bitmap.iter().filter(|&&b| b == target).count()
}).sum();
```

**Move inner functions to module level** when rayon closures need to reference them (closures can't capture nested `fn` items that reference local variables).

## euler_utils

Available modules and key functions:

- **primes**: `sieve(n)`, `sieve_smallest_factor(n)`, `primes_up_to(n)`, `is_prime(n)`, `miller_rabin(n)`
- **modular**: `mod_pow(base, exp, m)`, `mod_mul(a, b, m)`, `mod_inv(a, m)`, `extended_gcd(a, b)`, `ModInt`
- **number**: `gcd(a, b)`, `lcm(a, b)`, `euler_phi(n)`, `divisors(n)`, `divisor_count(n)`, `divisor_sum(n)`, `factor(n)`
- **fraction**: `Rational64`, `BigRational`, `frac(num, den)`
- **binomial**: `BinomialMod::new(max_n, modulus)`, `.choose(n, r)`
- **crt**: `crt(remainders, moduli) -> Option<(i64, i64)>`
- **matrix**: `ModMatrix::<N>::identity(m)`, `.from_data(arr, m)`, `.mul(&other)`, `.pow(exp)`, `.mul_vec(&v)`

Note: For hot loops, inline local versions of `mod_pow`/`mod_inv` instead of calling the library (see rule 4).

### Matrix Exponentiation

Use `ModMatrix` for linear recurrences and similar problems:
```rust
use euler_utils::ModMatrix;

// Fibonacci: [[1,1],[1,0]]^n
let m = ModMatrix::<2>::from_data([[1, 1], [1, 0]], MOD);
let fib_n = m.pow(n).data[0][1];

// Apply to a column vector
let result = m.pow(n).mul_vec(&initial_state);
```

Features:
- Const generic dimension (stack-allocated, cache-friendly)
- Deferred modular reduction in multiply (batched every 4 or N iterations depending on modulus size)
- Binary exponentiation: O(N^3 log(exp))

## Common Pitfalls

1. **Type casting chains** (`as u32 as u64 as usize`): Pick one integer type and stick with it. Mixed types cause implicit widening/narrowing in hot paths.
2. **Closure vs loop `return`**: When converting rayon `.map(|x| { ... return val; })` back to a for loop, replace `return val` with `total += val; continue;` -- `return` inside a block returns from the enclosing function.
3. **u128 in mod_pow**: The generic `mod_pow(u64, u64, u64)` uses u128 intermediates. When modulus < 2^32, pure u64 arithmetic is 3-4x faster.
