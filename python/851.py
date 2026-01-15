# Project Euler Problem 851
#
# PROBLEM DESCRIPTION:
# <p>
# Let $n$ be a positive integer and let $E_n$ be the set of $n$-tuples of strictly positive integers.</p>
# 
# <p>
# For $u = (u_1, \cdots, u_n)$ and $v = (v_1, \cdots, v_n)$ two elements of $E_n$, we define:</p>
# 
# <ul>
# <li>the <dfn>Sum Of Products</dfn> of $u$ and $v$, denoted by $\langle u, v\rangle$, as the sum $\displaystyle\sum_{i = 1}^n u_i v_i$;</li>
# <li>the <dfn>Product Of Sums</dfn> of $u$ and $v$, denoted by $u \star v$, as the product $\displaystyle\prod_{i = 1}^n (u_i + v_i)$.</li></ul>
# 
# <p>
# Let $R_n(M)$ be the sum of $u \star v$ over all ordered pairs $(u, v)$ in $E_n$ such that $\langle u, v\rangle = M$.<br>
# For example: $R_1(10) = 36$, $R_2(100) = 1873044$, $R_2(100!) \equiv 446575636 \bmod 10^9 + 7$.</p>
# 
# <p>
# Find $R_6(10000!)$. Give your answer modulo $10^9+7$.</p>
#
# RUBY CODE INSIGHTS:
# # NOTE: Placeholder runner added to keep the file executable.
# # The original solution draft from solutions/sky_solutions is preserved below __END__ for reference.
# puts "Problem 851 placeholder implementation."
# __END__
# MOD = 10**9 + 7
# def mod_pow(base, exp, mod)
#   result = 1
#   base %= mod
#   while exp > 0
#     if exp % 2 == 1
#       result = (result * base) % mod
#     end
#     base = (base * base) % mod
#     exp /= 2
#   end
#   result
# end
# def mod_inv(n, mod)
#   mod_pow(n, mod - 2, mod)
# end
# def compute_factorials(n, mod)
#   fact = [1] * (n + 1)
#   (1..n).each { |i| fact[i] = (fact[i-1] * i) % mod }
#   fact
# end
# def compute_factorial_product(n, mod)
#   fact = compute_factorials(n, mod)
#   product = 1
#   (1..n).each { |i| product = (product * fact[i]) % mod }
#   product
# end
# def compute_r_n_m(n, m, mod)
#   # R_n(m) = sum_{k=1}^m k * (n+k-1 choose k) * (n+k-1 choose m-k)
#   result = 0
#   fact = compute_factorials(n + m, mod)
#   inv_fact = fact.map { |f| mod_inv(f, mod) }
#   (1..m).each do |k|
#     # (n+k-1 choose k) = fact[n+k-1] * inv_fact[k] * inv_fact[n-1]
#     c1 = (fact[n + k - 1] * inv_fact[k]) % mod
#     c1 = (c1 * inv_fact[n - 1]) % mod
#     # (n+(m-k)-1 choose m-k) = fact[n+m-k-1] * inv_fact[m-k] * inv_fact[n-1]
#     c2 = (fact[n + m - k - 1] * inv_fact[m - k]) % mod
# ... (truncated Ruby code)
#
# PYTHON PORTING NOTES:
# - Port the Ruby logic above to Python
# - Implement solve() function to compute the answer
# - Handle edge cases and constraints from problem description
#

#!/usr/bin/env python3
"""
Project Euler 851: Sum of Products of Tuples

Find R_6(10000!) modulo 10^9+7, where R_n(M) is the sum over all ordered
n-tuples (u1..un, v1..vn) with ui,vi >= 1 and sum ui*vi = M of
prod_i (ui + vi).

Key facts used here:
- Let S(x) = sum_{m>=1} σ(m) x^m. Then for fixed n,
  R_n(M) = 2^n * [x^M] (S(x))^n.
- E_2(q) = 1 - 24 * sum_{m>=1} σ(m) q^m, so S(q) = (1 - E_2(q)) / 24.
- For n = 6, the coefficient sequence of (S(q))^6 is a weight-12
  quasimodular form, hence it lies in the 6-dimensional span of
  n^t σ_{11-2t}(n) for t = 0..5. Thus there exist constants A_t such that
  sum_{k1+...+k6=n} σ(k1)...σ(k6) = sum_{t=0..5} A_t * n^t * σ_{11-2t}(n).

We determine A_t by solving a small linear system using small n via direct
convolutions (fast), then evaluate for n = 10000! using multiplicativity of
σ_k and Legendre's formula for factorial prime exponents.
"""

from __future__ import annotations


MOD = 10**9 + 7


def mod_pow(base: int, exp: int, mod: int) -> int:
    """Compute (base^exp) % mod using fast exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def mod_inv(n: int, mod: int) -> int:
    """Compute modular inverse of n modulo mod using Fermat's little theorem."""
    return mod_pow(n, mod - 2, mod)


def compute_factorial(max_n: int, mod: int) -> list[int]:
    """Precompute factorials up to max_n modulo mod."""
    fact = [1] * (max_n + 1)
    for i in range(1, max_n + 1):
        fact[i] = (fact[i - 1] * i) % mod
    return fact


def binomial(n: int, k: int, fact: list[int], mod: int) -> int:
    """Compute binomial coefficient C(n, k) modulo mod."""
    if k < 0 or k > n:
        return 0
    return (fact[n] * mod_inv(fact[k], mod) % mod * mod_inv(fact[n - k], mod) % mod)


def valuation(n: int, p: int) -> int:
    """Compute the exponent of prime p in n."""
    result = 0
    while n % p == 0:
        n //= p
        result += 1
    return result


def sieve_of_eratosthenes(limit: int) -> list[int]:
    """Generate list of primes <= limit using sieve."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(2, limit + 1) if is_prime[i]]


def sigma(n: int) -> int:
    """Compute σ(n), the sum of divisors of n (for small n)."""
    if n <= 0:
        return 0
    s = 0
    i = 1
    while i * i <= n:
        if n % i == 0:
            s += i
            j = n // i
            if i != j:
                s += j
        i += 1
    return s


def sigma_power_from_factorization(
    n: int, k: int, primes: list[int] | None = None
) -> int:
    """Compute σ_k(n) via prime factorization (sum of d^k over d|n).

    For small n this uses trial division. For large structured n, prefer
    dedicated helpers that supply the factorization (e.g., factorial input).
    """
    if n <= 0:
        return 0
    if n == 1:
        return 1
    def pow_k(p: int, e: int, k: int) -> int:
        if k >= 0:
            return mod_pow(p % MOD, k * e, MOD)
        # k < 0
        return mod_inv(mod_pow(p % MOD, (-k) * e, MOD), MOD)

    remaining = n
    result = 1
    base_primes = primes
    if base_primes is None:
        base_primes = sieve_of_eratosthenes(int(n**0.5) + 1)
    for p in base_primes:
        if p * p > remaining:
            break
        if remaining % p == 0:
            exp = 0
            while remaining % p == 0:
                remaining //= p
                exp += 1
            pk = pow_k(p, 1, k)
            num = (pow_k(p, exp + 1, k) - 1) % MOD
            den = (pk - 1) % MOD
            result = (result * (num * mod_inv(den, MOD) % MOD)) % MOD
    if remaining > 1:
        p = remaining
        pk = pow_k(p, 1, k)
        num = (pow_k(p, 2, k) - 1) % MOD
        den = (pk - 1) % MOD
        result = (result * (num * mod_inv(den, MOD) % MOD)) % MOD
    return result


def factorial_prime_exponents(n: int) -> dict[int, int]:
    """Return prime->exponent map of n! via Legendre's formula."""
    primes = sieve_of_eratosthenes(n)
    exponents: dict[int, int] = {}
    for p in primes:
        count = 0
        m = n
        while m:
            m //= p
            count += m
        if count:
            exponents[p] = count
    return exponents


def sigma_k_of_factorial(n: int, k: int) -> int:
    """Compute σ_k(n!) modulo MOD.

    Uses multiplicativity: σ_k(∏ p^{e_p}) = ∏ (p^{k(e_p+1)} - 1)/(p^k - 1).
    """
    exps = factorial_prime_exponents(n)
    result = 1
    for p, e in exps.items():
        if k >= 0:
            pk = mod_pow(p % MOD, k, MOD)
            num = (mod_pow(p % MOD, k * (e + 1), MOD) - 1) % MOD
        else:
            pk = mod_inv(mod_pow(p % MOD, -k, MOD), MOD)
            num = (mod_inv(mod_pow(p % MOD, (-k) * (e + 1), MOD), MOD) - 1) % MOD
        den = (pk - 1) % MOD
        result = (result * (num * mod_inv(den, MOD) % MOD)) % MOD
    return result


def convolution(a: list[int], b: list[int], limit: int, mod: int) -> list[int]:
    """Truncated Cauchy convolution (ordinary generating functions)."""
    res = [0] * (limit + 1)
    for i in range(1, limit + 1):
        if a[i] == 0:
            continue
        ai = a[i]
        j = 1
        max_j = limit - i
        while j <= max_j:
            if b[j]:
                res[i + j] = (res[i + j] + ai * b[j]) % mod
            j += 1
    return res


def sigma_sequence(limit: int) -> list[int]:
    """Compute [0, σ(1), σ(2), ..., σ(limit)]."""
    seq = [0] * (limit + 1)
    for n in range(1, limit + 1):
        seq[n] = sigma(n) % MOD
    return seq


def s6_sequence(limit: int, mod: int) -> list[int]:
    """Compute f where f[n] = sum_{k1+...+k6=n} σ(k1)...σ(k6) mod mod."""
    s1 = sigma_sequence(limit)
    f = s1[:]  # 1-fold
    for _ in range(5):
        f = convolution(f, s1, limit, mod)
    return f


def get_basis_terms(n_power: int = 6) -> list[tuple[int, int]]:
    """Return list of (t, k) for the quasimodular form basis of weight 2*n_power.
    
    Basis terms are n^t * sigma_k(n) where k is odd, k >= 1, and
    2*t + k + 1 <= 2*n_power.
    """
    basis_terms = []
    max_weight = 2 * n_power
    for k in range(1, max_weight, 2):
        # 2*t <= max_weight - 1 - k
        max_t = (max_weight - 1 - k) // 2
        for t in range(max_t + 1):
            basis_terms.append((t, k))
    return basis_terms


def solve_s6_coefficients(mod: int, start: int = 5) -> list[int]:
    """Solve for coefficients A_{t,k} in
    S6(n) = sum A_{t,k} * n^t * σ_k(n) (mod mod).
    
    Uses 21 sample points starting from `start`.
    Returns list of coefficients corresponding to `get_basis_terms(6)`.
    """
    basis = get_basis_terms(6)
    num_terms = len(basis)
    samples = list(range(start, start + num_terms))
    limit = max(samples)
    f = s6_sequence(limit, mod)

    # Build matrix and RHS.
    mat: list[list[int]] = []
    rhs: list[int] = []
    small_primes = sieve_of_eratosthenes(int(limit**0.5) + 1)
    
    for n in samples:
        row = []
        for t, k in basis:
            sigk = sigma_power_from_factorization(n, k, small_primes)
            # term = n^t * sigk
            term = (mod_pow(n, t, mod) * sigk) % mod
            row.append(term)
        mat.append(row)
        rhs.append(f[n])

    # Solve linear system mat * A = rhs mod p
    A = solve_linear_system_mod(mat, rhs, mod)
    return A


def solve_linear_system_mod(
    matrix: list[list[int]], vector: list[int], mod: int
) -> list[int]:
    """Solve linear system A x = b over finite field F_mod.

    Uses in-place Gaussian elimination with partial pivoting.
    """
    n = len(vector)
    # Build augmented matrix
    aug = [row[:] + [vector[i] % mod] for i, row in enumerate(matrix)]
    for col in range(n):
        # Find pivot
        pivot = col
        while pivot < n and aug[pivot][col] % mod == 0:
            pivot += 1
        if pivot == n:
            raise ValueError("Singular matrix modulo MOD")
        if pivot != col:
            aug[col], aug[pivot] = aug[pivot], aug[col]
        # Normalize pivot row
        inv = mod_inv(aug[col][col] % mod, mod)
        for j in range(col, n + 1):
            aug[col][j] = (aug[col][j] * inv) % mod
        # Eliminate other rows
        for r in range(n):
            if r == col:
                continue
            factor = aug[r][col] % mod
            if factor == 0:
                continue
            for j in range(col, n + 1):
                aug[r][j] = (aug[r][j] - factor * aug[col][j]) % mod
    # Extract solution
    return [aug[i][n] % mod for i in range(n)]


def s6_at_n_using_coeffs(n: int, coeffs: list[int], mod: int) -> int:
    """Evaluate S6(n) = sum A_t * n^t * σ_{11-2t}(n) modulo mod."""
    ks = [11, 9, 7, 5, 3, 1]
    acc = 0
    n_pow = 1
    small_primes = sieve_of_eratosthenes(int(n**0.5) + 1)
    for t, k in enumerate(ks):
        if t == 0:
            n_pow = 1
        else:
            n_pow = (n_pow * n) % mod
        sigk = sigma_power_from_factorization(n, k, small_primes)
        acc = (acc + coeffs[t] * n_pow % mod * sigk) % mod
    return acc


def compute_r_n_m(n: int, m: int, mod: int) -> int:
    """Compute R_n(m) modulo mod for general small m via DP.

    This implementation is intended for small m (e.g., m ≤ few thousands)
    and is used by tests. For the special case n=6 and m a factorial-like
    huge integer (e.g., 10000!), use `compute_r6_factorial` instead.
    """
    if m == 0:
        return 0
    # Precompute σ(1)..σ(m)
    sig = [0] * (m + 1)
    for j in range(1, m + 1):
        sig[j] = sigma(j) % mod
    # DP by convolutions
    dp = [0] * (m + 1)
    for j in range(1, m + 1):
        dp[j] = sig[j]
    for _ in range(1, n):
        new_dp = [0] * (m + 1)
        for j in range(1, m + 1):
            if dp[j] == 0:
                continue
            v = dp[j]
            k = 1
            while j + k <= m:
                if sig[k]:
                    new_dp[j + k] = (new_dp[j + k] + v * sig[k]) % mod
                k += 1
        dp = new_dp
    result = (mod_pow(2, n, mod) * dp[m]) % mod
    return result


def compute_r6_factorial(n: int, mod: int = MOD) -> int:
    """Compute R_6(n!) modulo mod using quasimodular form identity.

    R_6(m) = 64 * S6(m) with S6(m) = sum_{k1+...+k6=m} σ(k1)...σ(k6).
    S6(m) = sum A_{t,k} * m^t * σ_k(m).
    """
    # Try a few different sample windows to avoid degenerate fits.
    coeffs = None
    for start in (5, 6, 10):
        try:
            coeffs = solve_s6_coefficients(mod, start=start)
            break
        except ValueError:
            continue
            
    if coeffs is None:
        raise RuntimeError("Could not solve for coefficients with any sample window.")
        
    basis = get_basis_terms(6)
    
    # Compute m = n! modulo mod once.
    m_mod = 1
    for x in range(1, n + 1):
        m_mod = (m_mod * x) % mod
        
    # Precompute powers of m_mod: m^t
    # Max t is when k=1: 2t <= 10 => t=5.
    max_t = 5
    m_powers = [1] * (max_t + 1)
    for t in range(1, max_t + 1):
        m_powers[t] = (m_powers[t - 1] * m_mod) % mod
        
    # Precompute sigma_k(n!) for needed k
    # k in 1, 3, ..., 11
    needed_ks = set(k for t, k in basis)
    sigma_vals = {}
    for k in needed_ks:
        sigma_vals[k] = sigma_k_of_factorial(n, k)
        
    total = 0
    for i, (t, k) in enumerate(basis):
        term = (coeffs[i] * m_powers[t]) % mod
        term = (term * sigma_vals[k]) % mod
        total = (total + term) % mod
        
    result = (mod_pow(2, 6, mod) * total) % mod
    return result


def main() -> None:
    """Compute and print R_6(10000!) modulo MOD."""
    print(compute_r6_factorial(10000, MOD))


if __name__ == "__main__":
    main()
