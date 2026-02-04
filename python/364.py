"""Project Euler Problem 364: Comfortable Distance.

T(N) counts distinct seating sequences for N seats following priority rules.
Formula (Max Alekseyev, OEIS A192008):
  T(n) = sum over v=0,1,2 and valid m of:
    (m+k+1)! * C(m+k,m) * 2^k * (k+v)! * (m+k)! * (1+(v==1))
  where k = (n-1-v-2m)/3.

Compute T(1000000) mod 100000007.
"""

def solve():
    MOD = 100000007
    N = 1000000

    # Precompute factorials mod MOD
    fact = [1] * (N + 2)
    for i in range(1, N + 2):
        fact[i] = fact[i-1] * i % MOD

    inv_fact = [1] * (N + 2)
    inv_fact[N + 1] = pow(fact[N + 1], MOD - 2, MOD)
    for i in range(N, -1, -1):
        inv_fact[i] = inv_fact[i+1] * (i+1) % MOD

    def C_mod(n, k):
        if k < 0 or k > n:
            return 0
        return fact[n] * inv_fact[k] % MOD * inv_fact[n-k] % MOD

    r = 0
    for v in range(3):
        rem = N - 1 - v
        if rem < 0:
            continue

        if rem % 3 == 0:
            m_start = 0
        elif rem % 3 == 1:
            m_start = 2
        else:
            m_start = 1

        m = m_start
        while 2 * m <= rem:
            k = (rem - 2 * m) // 3
            mk = m + k

            term = fact[mk + 1]
            term = term * C_mod(mk, m) % MOD
            term = term * pow(2, k, MOD) % MOD
            term = term * fact[k + v] % MOD
            term = term * fact[mk] % MOD
            if v == 1:
                term = term * 2 % MOD

            r = (r + term) % MOD
            m += 3

    return r

if __name__ == "__main__":
    print(solve())
