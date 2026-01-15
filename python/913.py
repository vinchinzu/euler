"""
Project Euler Problem 913
"""
import math

def gcd(a, b):
    while b:
        a, b = b, a % b
    return a

def get_prime_factors_small(n):
    """
    Returns a dictionary of prime factors for small n (n <= 10^8).
    """
    factors = {}
    d = 2
    temp = n
    while d * d <= temp:
        while temp % d == 0:
            factors[d] = factors.get(d, 0) + 1
            temp //= d
        d += 1
    if temp > 1:
        factors[temp] = factors.get(temp, 0) + 1
    return factors

def get_prime_factors_of_L(n, m):
    """
    Factors L = n^4 * m^4 - 1 for the large problem,
    or L = n * m - 1 for the small problem if needed.
    Wait, for small problem L = nm - 1.
    For large problem L = (nm)^4 - 1.
    
    This function factors X^4 - 1 where X = nm.
    L = (X-1)(X+1)(X^2+1).
    Since X <= 10000, X^2+1 <= 10^8 + 1.
    We can factor each part using trial division.
    """
    X = n * m
    # Factors of X - 1
    factors = get_prime_factors_small(X - 1)
    
    # Factors of X + 1
    f2 = get_prime_factors_small(X + 1)
    for p, count in f2.items():
        factors[p] = factors.get(p, 0) + count

    # Factors of X^2 + 1
    f3 = get_prime_factors_small(X * X + 1)
    for p, count in f3.items():
        factors[p] = factors.get(p, 0) + count

    return factors

def pow_mod(a, b, m):
    res = 1
    a %= m
    while b > 0:
        if b % 2 == 1:
            res = (res * a) % m
        a = (a * a) % m
        b //= 2
    return res

def get_order_mod_pk(a, p, k):
    """
    Computes order of a modulo p^k.
    Precondition: gcd(a, p) = 1.
    """
    # First compute order mod p
    # order divides p-1
    # Factor p-1
    if p == 2:
        # For p=2, order is usually power of 2
        # But group structure of (Z/2^k Z)* is not cyclic for k>=3.
        # However, we just need the smallest d such that a^d = 1 mod 2^k.
        # Just check powers of 2?
        # a must be odd.
        # Order divides phi(2^k) = 2^(k-1).
        # So order is a power of 2.
        order = 1
        curr = a % (2**k)
        if curr == 1:
            return 1
        # Try 2, 4, 8...
        for i in range(1, k+1): # max power is k-1? 2^(k-1)
            order *= 2
            curr = (curr * curr) % (2**k)
            if curr == 1:
                return order
        return order

    # For odd p
    p_minus_1 = p - 1
    factors = get_prime_factors_small(p_minus_1)
    order = p_minus_1
    for q, count in factors.items():
        # Try to divide order by q as much as possible
        while order % q == 0:
            temp_order = order // q
            if pow_mod(a, temp_order, p) == 1:
                order = temp_order
            else:
                break

    # Now lift to p^k
    # If a^order = 1 + c*p^v, then order mod p^(v+1) is order*p...
    # We can just iterate.
    current_mod = p
    current_val = pow_mod(a, order, current_mod) # This is 1 mod p

    # We need order mod p^k.
    # We know order mod p is `order`.
    # Let order_j be order mod p^j.
    # order_{j+1} is either order_j or order_j * p.
    # Actually, generally, if ord_p(a) = d, then ord_{p^k}(a) = d * p^v for some v.
    # We can just multiply by p until it works.

    total_mod = p**k
    curr_order = order
    while True:
        if pow_mod(a, curr_order, total_mod) == 1:
            return curr_order
        curr_order *= p

def count_cycles(n, m, is_large=False):
    """
    Counts cycles for transformation of n x m matrix.
    If is_large is True, arguments are n, m but we solve for n^4, m^4.
    Otherwise we solve for n, m.
    """
    if is_large:
        # Solving for N=n^4, M=m^4
        # L = N*M - 1 = n^4 m^4 - 1
        # Multiplier A = N = n^4
        factors_L = get_prime_factors_of_L(n, m) # Factors of (nm)^4 - 1
        multiplier = pow(n, 4)
    else:
        # Solving for N=n, M=m
        # L = n*m - 1
        # Multiplier A = n
        factors_L = get_prime_factors_small(n * m - 1)
        multiplier = n

    # Precompute order of multiplier for each maximal prime power in L
    # p^e || L
    prime_power_orders = {} # p -> (e, order_mod_p_e, phi_p_e)
    primes = sorted(factors_L.keys())

    for p in primes:
        e = factors_L[p]
        pk = p**e
        ord_val = get_order_mod_pk(multiplier, p, e)
        phi_val = (p**(e-1)) * (p-1)
        prime_power_orders[p] = (e, ord_val, phi_val)

    # Now DFS to sum phi(d)/ord_d(multiplier)
    # We build d prime by prime
    # For each prime p with exponent e in L (so p^e || L):
    # divisor d can have p^j where 0 <= j <= e.
    # phi(d) is multiplicative.
    # ord_d(mult) = lcm(ord_{p^j}(mult)) over p^j || d.
    # Note ord_{p^j} is needed.
    # prime_power_orders stores ord for p^e.
    # We need ord for p^j for j < e.
    # It is efficient to compute them on demand or precompute all.
    # Since e is small (usually 1, maybe up to 13 for 2^13 < 10000?),
    # we can precompute for all j.

    # Better precompute structure: p -> list of (phi(p^j), ord(p^j)) for j=0..e
    precomputed = []
    for p in primes:
        e = factors_L[p]
        entry = []
        # j=0 => d=1, phi=1, ord=1
        entry.append((1, 1))

        current_p_pow = 1
        current_phi = 1
        # We need ord mod p^j.
        # We can compute it from scratch or derive from higher order?
        # Safer to compute from scratch or incrementally.

        # Incremental computation
        # ord mod p
        ord_p = get_order_mod_pk(multiplier, p, 1)
        curr_ord = ord_p
        current_p_pow = p

        # For j=1
        entry.append((p-1, curr_ord))

        for j in range(2, e+1):
            current_p_pow *= p
            # Check if curr_ord works for new power
            if pow_mod(multiplier, curr_ord, current_p_pow) != 1:
                curr_ord *= p
            entry.append((p**(j-1) * (p-1), curr_ord))

        precomputed.append(entry)

    # DFS
    # State: (index in primes, current_lcm_order, current_phi_product)

    total_sum = 0

    stack = [(0, 1, 1)] # index, lcm_order, phi_prod

    while stack:
        idx, lcm_ord, phi_prod = stack.pop()

        if idx == len(primes):
            total_sum += phi_prod // lcm_ord
            continue

        # Try all exponents for primes[idx]
        # entries is list of (phi, ord) for exponents 0..e
        entries = precomputed[idx]
        for phi_part, ord_part in entries:
            # new_lcm = lcm(lcm_ord, ord_part)
            # lcm(a, b) = (a*b)//gcd(a, b)
            g = gcd(lcm_ord, ord_part)
            new_lcm = (lcm_ord * ord_part) // g

            stack.append((idx + 1, new_lcm, phi_prod * phi_part))

    # The sum counts cycles in {0, ..., L-1}.
    # This includes the cycle {0}.
    # The cycles we want are in {1, ..., L-1}.
    # So we have (total_sum - 1) cycles in {1, ..., nm-2}.
    # And we add fixed points 0 and nm-1 (indices).
    # Wait, the formula S = nm - (cycles in 1..nm-2) - 2 ?
    # From my thought process:
    # Cycles_in_range = total_sum - 1.
    # Total_Cycles_Permutation = Cycles_in_range + 2.
    # S = Size - Total_Cycles_Permutation
    # S = Size - (total_sum - 1 + 2) = Size - total_sum - 1.
    
    return total_sum

def S(n, m, is_large=False):
    if is_large:
        size = (n*m)**4
    else:
        size = n*m

    if size <= 2: return 0 # Trivial cases
    
    # corner case: if size-1 == 1, loop doesn't work well?
    # nm=2 -> size-1=1. L=1. Divisors d=1.
    # sum = phi(1)/ord(1) = 1.
    # S = 2 - 1 - 1 = 0. Correct.

    cycle_term = count_cycles(n, m, is_large)
    return size - cycle_term - 1

def solve_small():
    total = 0
    for n in range(2, 101):
        for m in range(n, 101):
            val = S(n, m, is_large=False)
            total += val
    return total

def solve_large():
    total = 0
    for n in range(2, 101):
        for m in range(n, 101):
            val = S(n, m, is_large=True)
            total += val
    return total

def solve():
    return solve_large()

if __name__ == "__main__":
    import sys
    if len(sys.argv) > 1 and sys.argv[1] == "small":
        print(solve_small())
    else:
        print(solve())
