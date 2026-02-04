"""Project Euler Problem 405: A rectangular tiling.

f(n) counts 4-way meeting points in tiling T(n).
Recurrence: f(n) = 5*f(n-1) - 2*f(n-2) - 8*f(n-3) + 6
with f(1)=0, f(2)=2, f(3)=16.

We need f(10^(10^18)) mod 17^7. Use matrix exponentiation with
period reduction for the enormous exponent.
"""
from math import gcd

def mat_mul(A, B, mod):
    n = len(A)
    m = len(B[0])
    k = len(B)
    C = [[0]*m for _ in range(n)]
    for i in range(n):
        for j in range(m):
            s = 0
            for l in range(k):
                s += A[i][l] * B[l][j]
            C[i][j] = s % mod
    return C

def mat_pow(M, p, mod):
    n = len(M)
    result = [[1 if i==j else 0 for j in range(n)] for i in range(n)]
    base = [row[:] for row in M]
    while p > 0:
        if p & 1:
            result = mat_mul(result, base, mod)
        base = mat_mul(base, base, mod)
        p >>= 1
    return result

def solve():
    MOD = 17**7  # 410338673

    # Matrix for recurrence f(n) = 5*f(n-1) - 2*f(n-2) - 8*f(n-3) + 6
    # State: [f(n), f(n-1), f(n-2), 1]
    M = [[5, -2, -8, 6], [1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 1]]

    # Period of sequence mod 17 is 8
    # Period mod 17^7 divides 8 * 17^6 = 193100552
    period = 8 * 17**6  # 193100552

    # Compute 10^(10^18) mod period
    # period = 8 * 17^6. CRT: compute mod 8 and mod 17^6 separately.

    # 10^(10^18) mod 8: 10 ≡ 2 (mod 8), 2^1=2, 2^2=4, 2^3=0 mod 8
    # For k >= 3: 10^k ≡ 0 (mod 8). 10^18 >= 3, so 10^(10^18) ≡ 0 (mod 8).
    r8 = 0

    # 10^(10^18) mod 17^6: use Euler's theorem
    # phi(17^6) = 17^5 * 16
    phi_17_6 = 17**5 * 16  # 22717712
    # Need 10^18 mod phi(17^6)
    exp_mod = pow(10, 18, phi_17_6)
    r17_6 = pow(10, exp_mod, 17**6)

    # CRT: x ≡ 0 (mod 8), x ≡ r17_6 (mod 17^6)
    # x = 8 * k where 8*k ≡ r17_6 (mod 17^6)
    # k ≡ r17_6 * modinv(8, 17^6) (mod 17^6)
    inv8 = pow(8, -1, 17**6)
    k = (r17_6 * inv8) % (17**6)
    n_mod_period = (8 * k) % period

    if n_mod_period < 3:
        n_mod_period += period

    steps = n_mod_period - 3
    Mn = mat_pow(M, steps, MOD)
    state = [16, 2, 0, 1]
    result = sum(Mn[0][j] * state[j] for j in range(4)) % MOD
    return result

if __name__ == "__main__":
    print(solve())
