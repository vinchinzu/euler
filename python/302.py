"""Project Euler Problem 302: Strong Achilles Numbers

An Achilles number is a number that is powerful but not a perfect power.
A powerful number is a positive integer where for every prime p dividing it, p^2 also divides it.
A Strong Achilles number is one where both the number and its totient are Achilles numbers.

Find the count of Strong Achilles numbers up to 10^18.

Algorithm (from Java):
- Generate candidates recursively by maintaining factorizations of n and phi(n)
- Modify dictionaries in-place, then restore them after recursion
- If phi(n) has a prime with exponent 1, try to fix it
- Otherwise, check if both n and phi(n) are Achilles numbers
"""

def sieve_primes(limit):
    """Generate primes up to limit using sieve."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            for j in range(i*i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(2, limit + 1) if is_prime[i]]

def smallest_prime_factor(limit):
    """Compute smallest prime factor for all numbers up to limit."""
    spf = list(range(limit + 1))
    for i in range(2, int(limit**0.5) + 1):
        if spf[i] == i:
            for j in range(i*i, limit + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf

def gcd(a, b):
    """Compute greatest common divisor."""
    while b:
        a, b = b, a % b
    return a

def gcd_list(values):
    """Compute GCD of a list of values."""
    result = 0
    for v in values:
        result = gcd(result, v)
    return result

def is_prime_check(n):
    """Check if n is prime."""
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    for i in range(3, int(n**0.5) + 1, 2):
        if n % i == 0:
            return False
    return True

def solve():
    N = 10**18
    L = int(N**(1/3)) + 1

    primes = sieve_primes(L)
    spf = smallest_prime_factor(L)
    achilles_set = set()

    def helper(n, factors, phi, max_p):
        """Recursively generate Strong Achilles numbers."""
        # Find largest prime in phi with exponent 1
        bad_p = 0
        for p, exp in phi.items():
            if exp == 1 and p > bad_p:
                bad_p = p

        if bad_p == 0:
            # No prime with exponent 1 in phi
            # Check if both n and phi(n) are Achilles
            if factors:
                factor_exps = list(factors.values())
                phi_exps = list(phi.values())
                if gcd_list(factor_exps) == 1 and gcd_list(phi_exps) == 1:
                    achilles_set.add(n)

            # Try adding primes already in phi
            for p in list(phi.keys()):
                if p < max_p:
                    add_prime(n, p, 2, factors, phi, p)

            # Try adding new primes (with exponent >= 3)
            for p in primes:
                if p >= max_p or n * p**3 >= N:
                    break
                if p not in phi:
                    add_prime(n, p, 3, factors, phi, p)

        elif n * bad_p**2 < N:
            # Try to fix bad_p by adding it
            add_prime(n, bad_p, 2, factors, phi, max_p)

            # Add primes q where bad_p | (q-1)
            p = bad_p + 1
            while p < max_p and n * p**2 < N:
                if p % bad_p == 1 and is_prime_check(p):
                    add_prime(n, p, 2, factors, phi, max_p)
                p += bad_p

    def add_prime(n, p, min_e, factors, phi, max_p):
        """Add prime p with various exponents. Modifies dicts in place."""
        if p in factors:
            return

        # Save previous phi[p]
        prev_e = phi.get(p)

        # Update phi for adding p^(min_e-1)
        phi[p] = phi.get(p, 0) + min_e - 1

        # Factor p-1 and add to phi
        phi_p_factors = []
        temp = p - 1
        while temp > 1:
            if temp < len(spf):
                pf = spf[temp]
            else:
                # Find smallest factor manually
                pf = temp
                for pr in primes:
                    if pr * pr > temp:
                        break
                    if temp % pr == 0:
                        pf = pr
                        break
            phi_p_factors.append(pf)
            phi[pf] = phi.get(pf, 0) + 1
            temp //= pf

        # Try different exponents
        e = min_e
        power_p = p ** e
        while n * power_p < N:
            factors[p] = e
            helper(n * power_p, factors, phi, max_p)

            # Update phi for next exponent
            phi[p] = phi.get(p, 0) + 1
            e += 1
            power_p *= p

        # Restore factors
        if p in factors:
            del factors[p]

        # Restore phi[p]
        if prev_e is not None:
            phi[p] = prev_e
        else:
            if p in phi:
                del phi[p]

        # Restore phi for factors of p-1
        for pf in phi_p_factors:
            phi[pf] -= 1
            if phi[pf] == 0:
                del phi[pf]

    helper(1, {}, {}, 10**9)
    return len(achilles_set)

if __name__ == "__main__":
    print(solve())
