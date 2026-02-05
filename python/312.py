"""Project Euler Problem 312: Cyclic paths on Sierpinski graphs

Find C(C(C(10000))) mod 13^8, where C(n) is the number of Hamiltonian cycles
in the Sierpinski graph of order n.

The formula is: C(n) = 2^(3^(n-2)) * 3^((3^(n-2)-3)/2)

For nested applications, use Euler's theorem to reduce exponents.

Algorithm (from Java):
- Define Ck(n, mod) = C(C(...C(n)...)) mod with k applications
- Use Euler's theorem: a^b ≡ a^(b mod φ(m)) (mod m) when gcd(a,m)=1
- Recursively reduce exponents using φ(mod)
"""

def euler_phi(n):
    """Compute Euler's totient function φ(n)."""
    result = n
    p = 2
    while p * p <= n:
        if n % p == 0:
            while n % p == 0:
                n //= p
            result -= result // p
        p += 1
    if n > 1:
        result -= result // n
    return result

def pow_mod(base, exp, mod):
    """Compute base^exp mod mod efficiently."""
    if mod == 1:
        return 0
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        exp >>= 1
        base = (base * base) % mod
    return result

def solve():
    N = 10000
    K = 3
    M = 13 ** 8

    def Ck(mod, k):
        """Compute C(C(...C(N)...)) mod mod with k applications of C."""
        # For Euler's theorem reduction
        mod1 = 2 * euler_phi(mod)
        mod2 = euler_phi(mod1)

        # Determine n value
        if k == 1:
            n = N
        else:
            n = Ck(mod2, k - 1)

        # Compute C(n) mod mod
        # C(n) = 2^(3^(n-2)) * 3^((3^(n-2)-3)/2)

        # First compute 3^(n-2) mod mod1
        exp1 = pow_mod(3, n - 2, mod1)

        # Compute 2^exp1 mod mod
        term1 = pow_mod(2, exp1, mod)

        # Compute 3^((exp1-3)/2) mod mod
        # Note: exp1-3 is always even since 3^k ≡ 3 or 1 or 9 (mod 2) and 3^k ≡ 1 (mod 2) for k>=1
        exp2 = (exp1 - 3) // 2
        term2 = pow_mod(3, exp2, mod)

        return (term1 * term2) % mod

    return Ck(M, K)

if __name__ == "__main__":
    print(solve())
