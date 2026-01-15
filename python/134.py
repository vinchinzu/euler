"""Project Euler Problem 134: Prime pair connection.

For every pair of consecutive primes (p1, p2) with 5 <= p1 <= 1,000,000:
1. Determine k, the number of digits in p1. Let M = 10^k.
2. Find the smallest positive integer S such that S ends with digits of p1 (S === p1 mod M)
   and S is divisible by p2 (S === 0 mod p2).
3. Sum these values of S.
"""

from sympy import primerange


def extended_gcd(a: int, b: int) -> tuple[int, int, int]:
    """Extended Euclidean Algorithm.
    
    Returns [gcd, x, y] such that a*x + b*y = gcd(a,b)
    """
    if a == 0:
        return (b, 0, 1)
    gcd_val, x1, y1 = extended_gcd(b % a, a)
    x = y1 - (b // a) * x1
    y = x1
    return (gcd_val, x, y)


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse.
    
    Returns x such that (a * x) % m == 1
    """
    gcd_val, x, _y = extended_gcd(a, m)
    # p2 can be larger than M, but gcd(p2, M) must be 1.
    # M = 10^k = 2^k * 5^k. So p2 cannot be 2 or 5.
    # Since p1 >= 5, p2 will be > 5, so p2 is not 2 or 5.
    # Thus, gcd(p2, M) will be 1.
    if gcd_val != 1:
        raise ValueError(f"Modular inverse does not exist if gcd(a, m) != 1, gcd was {gcd_val}")
    return (x % m + m) % m


def main() -> int:
    """Main function."""
    limit_p1 = 1_000_000
    # We need p2 for p1 up to limit_p1.
    # If p1 is the largest prime <= 1,000,000 (which is 999983),
    # p2 is the next prime (1000003).
    # So, generating primes up to limit_p1 + a small margin (e.g., 200, or just find next prime after limit_p1)
    # Prime.each(N) goes up to N inclusive.
    # Max p1 = 999983. Next prime is 1000003. So Prime.each(1000003) is sufficient.
    # A slightly larger upper bound for safety is fine.
    primes = list(primerange(2, limit_p1 + 200))  # Using a buffer like in the draft

    total_s_sum = 0

    for i in range(len(primes) - 1):
        p1 = primes[i]

        if p1 < 5:  # Start from p1 = 5
            continue
        if p1 > limit_p1:  # Process p1 up to the limit
            break

        p2 = primes[i + 1]

        # Determine k, the number of digits in p1
        k = len(str(p1))
        M = 10 ** k

        # We need S such that:
        # S ≡ p1 (mod M)  =>  S = p1 + M * t for some integer t
        # S ≡ 0 (mod p2)   =>  p1 + M * t ≡ 0 (mod p2)
        #                    =>  M * t ≡ -p1 (mod p2)
        #                    =>  t ≡ (-p1) * M^(-1) (mod p2)

        # Compute modular inverse of M modulo p2
        M_inv = mod_inverse(M, p2)

        # Compute t
        t = ((-p1) % p2) * M_inv % p2

        # Compute S
        S = p1 + M * t

        total_s_sum += S

    return total_s_sum


if __name__ == "__main__":
    print(main())
