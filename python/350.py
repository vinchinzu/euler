"""Project Euler Problem 350 - Constrained sequences.

Find f(G, L, N) mod 101^4, where G=10^6, L=10^12, N=10^18.

f counts lists of size N with gcd >= G and lcm <= L.

For each r = L/(G*d) for d=1..L/G, the contribution is based on
the prime factorization of d. For each prime power p^e in d,
the factor is (e+1)^N - 2*e^N + (e-1)^N.
The count of valid gcds for ratio r is floor(L/r) - G + 1.
"""

def solve():
    G = 10**6
    L = 10**12
    N = 10**18
    M = 101**4  # 104060401

    R = L // G  # 10^6

    # Sieve smallest prime factor
    spf = list(range(R + 1))
    for i in range(2, int(R**0.5) + 1):
        if spf[i] == i:
            for j in range(i*i, R + 1, i):
                if spf[j] == j:
                    spf[j] = i

    # Precompute powers of e mod M for e up to max possible exponent
    # Max exponent in factorization of r <= 10^6 is about 19 (2^19 > 10^6)
    max_e = 20
    powN = [0] * (max_e + 2)
    for e in range(max_e + 2):
        powN[e] = pow(e, N, M)

    ans = 0
    for r in range(1, R + 1):
        # Factor r using spf
        temp = r
        res = 1
        while temp > 1:
            p = spf[temp]
            e = 0
            while temp % p == 0:
                temp //= p
                e += 1
            factor = (powN[e + 1] - 2 * powN[e] + (powN[e - 1] if e > 0 else 0)) % M
            res = res * factor % M

        count = (L // r - G + 1) % M
        ans = (ans + count * res) % M

    return ans % M

if __name__ == "__main__":
    print(solve())
