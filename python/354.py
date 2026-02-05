"""Project Euler Problem 354 - Honeycomb distance distribution.

Given an infinite honeycomb of unit hexagons, B(l) is the number of honeycomb
centers at distance l from a particular center. Find count of l <= 5*10^11
such that B(l) = 450.

Uses Eisenstein integers and prime factorization modulo 3.
"""

def solve():
    N = 5 * 10**11
    K = 450
    L1 = (N / (3**0.5)) ** 2  # (N/sqrt(3))^2
    L2 = int((L1 / (7**4) / (13**4)) ** 0.5)

    # Sieve smallest prime factor
    spf = list(range(L2 + 1))
    for i in range(2, int(L2**0.5) + 1):
        if spf[i] == i:
            for j in range(i*i, L2 + 1, i):
                if spf[j] == j:
                    spf[j] = i

    # Check if n is composed only of primes ≡ 2 (mod 3)
    def is_2mod3_only(n):
        if n == 1:
            return True
        temp = n
        while temp > 1:
            p = spf[temp]
            if p % 3 != 2:
                return False
            temp //= p
        return True

    # Precompute count of numbers composed only of primes ≡ 2 (mod 3)
    num_2mod3s = [0] * (L2 + 1)
    for n in range(1, L2 + 1):
        num_2mod3s[n] = num_2mod3s[n - 1] + (1 if is_2mod3_only(n) else 0)

    # Check if n is prime
    def is_prime(n):
        if n < 2:
            return False
        if n == 2:
            return True
        if n % 2 == 0:
            return False
        if n <= L2:
            return spf[n] == n
        # For n > L2, do trial division
        for i in range(3, int(n**0.5) + 1, 2):
            if n % i == 0:
                return False
        return True

    ans = 0

    def find_nums_for_template(index, prod_primes, min_prime, limit, template):
        """Recursively find numbers matching the template."""
        nonlocal ans

        if index == len(template):
            # Add factors of 3 and primes ≡ 2 (mod 3)
            remaining = limit
            while remaining > 1:
                ans += num_2mod3s[int(remaining**0.5)]
                remaining /= 3
        else:
            e = template[index]
            p = min_prime if min_prime > 1 else 1
            if p % 3 == 0:
                p += 1
            elif p % 3 == 2:
                p += 2

            while p ** e <= limit:
                if prod_primes % p != 0 and is_prime(p):
                    # Determine next min prime
                    if index + 1 < len(template) and template[index] == template[index + 1]:
                        next_min = p + 3
                    else:
                        next_min = 1

                    find_nums_for_template(
                        index + 1,
                        prod_primes * p,
                        next_min,
                        limit / (p ** e),
                        template
                    )

                p += 3
                while p % 3 != 1:
                    p += 1

    def find_nums_for_all_templates(n, max_d, template):
        """Find all templates by factorizing K/6."""
        if n == 1:
            find_nums_for_template(0, 1, 1, L1, template)
        else:
            for d in range(2, max_d + 1):
                if n % d == 0:
                    template.append(d - 1)
                    find_nums_for_all_templates(n // d, d, template)
                    template.pop()

    find_nums_for_all_templates(K // 6, K // 6, [])
    return ans

if __name__ == "__main__":
    print(solve())
