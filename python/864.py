"""
Project Euler Problem 864: Square + 1 = Squarefree

C(n) is the number of squarefree integers of the form x² + 1 such that 1 ≤ x ≤ n.
A squarefree integer is not divisible by the square of any prime.

For example, C(10) = 9 and C(1000) = 895.

Find C(123567101113).

Solution Logic:
We want to compute sum_{x=1 to N} mu^2(x^2+1).
mu^2(k) = sum_{d^2 | k} mu(d).
So sum_{x=1 to N} sum_{d^2 | x^2+1} mu(d)
= sum_{d} mu(d) * #{x <= N : x^2+1 = 0 mod d^2}.

Let S_d(N) = #{x <= N : x^2+1 = 0 mod d^2}.
Total = sum_{d} mu(d) S_d(N).
Only d composed of primes p=2 or p=1 mod 4 can divide x^2+1.
Actually p=2 is impossible because x^2+1 = 1 or 2 mod 4, so d^2 (which is 0 mod 4) cannot divide x^2+1.
So d must be odd and composed of primes p = 1 mod 4.

We split the sum at threshold D.
Part A: sum_{d <= D} mu(d) S_d(N).
Part B: sum_{d > D} mu(d) S_d(N).

For Part B, we iterate k such that x^2+1 = k d^2.
Since d > D, k = (x^2+1)/d^2 < (N^2+1)/D^2.
Let K_LIMIT = (N^2+1)/D^2.
We iterate valid k (composed of 2, primes 1 mod 4) up to K_LIMIT.
For each k, we solve x^2 - k d^2 = -1 (Pell-like equation).
This gives us pairs (x, d).
If d > D and x <= N, we add mu(d) to the total.
"""

import math
import sys
import time

def solve(N=123567101113, D_LIMIT=10**8):
    
    print(f"N = {N}")
    print(f"Threshold D = {D_LIMIT}")
    
    K_LIMIT = (N**2 + 1) // (D_LIMIT**2) + 1
    sieve_limit = max(D_LIMIT, K_LIMIT)
    
    t0 = time.time()
    print(f"Sieving up to {sieve_limit}...")
    # For square-free check in Part B, we need primes up to sqrt(N/sqrt(1)) ~ 3.5e5?
    # No, max d is N. We need to check if d is squarefree.
    # d can be large. 
    # But d comes from Pell solution. d^2 = (x^2+1)/k.
    # d has only primes 1 mod 4.
    # We can check square-free by trial division up to some bound, 
    # but for very large d, factorizing is hard?
    # Actually d <= N. sqrt(d) <= sqrt(N) ~ 3.5e5.
    # So we only need primes up to 3.5e5 to fully factorize d.
    # Wait, d can be up to N ~ 1e11. sqrt(d) ~ 3.5e5.
    # So checking if d is square-free is easy with primes up to 3.5e5.
    
    # So we really need primes up to D_LIMIT for Part A DFS.
    
    t0 = time.time()
    print(f"Sieving up to {sieve_limit}...")
    primes = sieve(sieve_limit)
    primes_1mod4 = [p for p in primes if p % 4 == 1]
    print(f"Sieve done in {time.time()-t0:.2f}s. {len(primes)} primes, {len(primes_1mod4)} are 1 mod 4.")

    # --- Part A: Small d ---
    print("Starting Part A...")
    t0 = time.time()
    part_a_res = 0
    
    # DFS to generate square-free d <= D composed of primes_1mod4
    # We also maintain solutions mod d^2 to count efficiently.
    # Or just compute solutions on the fly?
    # Since we need to compute S_d(N), and S_d(N) uses solutions mod d^2.
    # We can carry (d, solutions) in DFS.
    
    # Stack: (index in primes_1mod4, current_d, current_sols)
    # solutions is list of x in [0, d^2) such that x^2 = -1 mod d^2.
    # For d=1, sols=[0]. (Wait, x^2=-1 mod 1 is 0=0. 0^2+1=1=0 mod 1. Correct.)
    # Actually for d=1, d^2=1. Any x works? No.
    # condition d^2 | x^2+1. If d=1, 1 | x^2+1 always.
    # So S_1(N) = N. mu(1) = 1.
    # My solver assumes we iterate d.
    
    # We need to be careful with d=1.
    # solutions_mod_m_sq logic:
    # If factors=[], m=1, sols=[0].
    # count_solutions: m_sq=1. sols=[0].
    # if a=0, a=1. 1 <= n. total += (n-1)//1 + 1 = n. Correct.
    
    # We can proceed with DFS.
    # But storing solutions list can be expensive if list is large.
    # Max solutions = 2^omega(d). 
    # For d < 10^8, omega(d) is small (at most 8-9, since 2*3*5*... > 10^8. For 5*13*17*... it's similar).
    # 2^9 = 512. Small enough.
    
    stack = [(0, 1, [0], 1)] # (idx, d, sols, mu)
    
    # Precompute mod_sqrt_minus1 for small primes? 
    # Maybe calculate on demand.
    
    count_a = 0
    
    while stack:
        idx, d, sols, mu = stack.pop()
        
        # Process current d
        # d is square-free product of primes. mu is set.
        # Calculate contribution
        
        d_sq = d * d
        cnt = count_solutions(N, d_sq, sols)
        if cnt > 0:
            part_a_res += mu * cnt
        count_a += 1
        
        # Recurse
        for i in range(idx, len(primes_1mod4)):
            p = primes_1mod4[i]
            new_d = d * p
            if new_d > D_LIMIT:
                break
            
            # Update solutions
            # We need to combine sols mod d^2 with roots mod p^2
            p_sq = p * p
            r = mod_sqrt_minus1(p, 2) # Root of x^2=-1 mod p^2
            roots_p = [r, p_sq - r]
            
            new_sols = []
            # CRT
            # x = s mod d_sq
            # x = r mod p_sq
            # Moduli d_sq and p_sq are coprime.
            # We need inverse of d_sq mod p_sq
            inv = pow(d_sq, -1, p_sq)
            
            for s in sols:
                for rp in roots_p:
                    # x = s + d_sq * ((rp - s) * inv % p_sq)
                    k = ((rp - s) * inv) % p_sq
                    x = s + d_sq * k
                    new_sols.append(x) # x is mod (d_sq * p_sq)
            
            stack.append((i + 1, new_d, new_sols, -mu))

    print(f"Part A done in {time.time()-t0:.2f}s. Terms: {count_a}. Sum: {part_a_res}")

    # --- Part B: Large d ---
    print("Starting Part B...")
    t0 = time.time()
    part_b_res = 0
    
    K_LIMIT = (N**2 + 1) // (D_LIMIT**2) + 1
    print(f"K limit: {K_LIMIT}")
    
    # Generate k values
    # k must be composed of 2 and primes_1mod4.
    # k must not be divisible by 4.
    # k can have square factors? Yes.
    # k must have at least one solution to x^2 = -1 mod k? No.
    # x^2 - k y^2 = -1.
    # If k has prime factor q = 3 mod 4, then x^2 = -1 mod q, impossible.
    # So k MUST be composed of 2 and primes_1mod4.
    # We generate such k via DFS.
    # We treat 2 separately. k can be 2^0 or 2^1. (Cannot be 2^2 or higher).
    
    # Primes for generating k
    # We need primes 1 mod 4 up to K_LIMIT.
    primes_k = [p for p in primes_1mod4 if p <= K_LIMIT]
    
    # Stack for k generation: (index, current_k)
    # Start with k=1 and k=2
    k_stack = [(0, 1), (0, 2)]
    
    checked_k = 0
    found_sols = 0
    
    # Precompute primes for checking square-free y
    # Max y is roughly N/sqrt(1) = N.
    # sqrt(y) <= sqrt(N) ~ 351521.
    # We need primes up to 351521.
    check_limit = int(math.isqrt(N)) + 100
    check_primes = sieve(check_limit)
    
    while k_stack:
        idx, k = k_stack.pop()
        
        # Check if Pell x^2 - k y^2 = -1 has solutions
        # This happens if length of period of sqrt(k) is odd.
        # If k is prime p=1 mod 4, always yes.
        # If k=2, yes.
        # If k composite, depends.
        
        # Also we need to iterate ALL solutions (x, y)
        # (x_n, y_n) coming from powers of fundamental unit.
        
        # Solve Pell for -1
        # Returns generator or list of (x, y) with x <= N
        sols = solve_pell_minus1(k, N)
        
        for x, y in sols:
            if y > D_LIMIT:
                # Check if y is square-free and valid primes
                # Actually y MUST have valid primes because x^2+1 = k y^2
                # and k has valid primes, x^2+1 has valid primes.
                # So we only check square-free.
                sq, mu_y = get_mu_if_squarefree(y, check_primes)
                if sq:
                    part_b_res += mu_y
                    found_sols += 1
        
        checked_k += 1
        if checked_k % 100000 == 0:
            print(f"Checked {checked_k} k-values...", flush=True)
            
        # Generate next k's
        for i in range(idx, len(primes_k)):
            p = primes_k[i]
            next_k = k * p
            if next_k > K_LIMIT:
                break
            k_stack.append((i, next_k)) # Allow repeated prime factors for k
            
    print(f"Part B done in {time.time()-t0:.2f}s. Found sols: {found_sols}. Sum: {part_b_res}")
    
    total = part_a_res + part_b_res
    print(f"Total C(N) = {total}")
    return total

def sieve(limit):
    if limit < 2: return []
    is_prime = bytearray([1]) * (limit + 1)
    is_prime[0] = 0
    is_prime[1] = 0
    for i in range(2, int(math.isqrt(limit)) + 1):
        if is_prime[i]:
            is_prime[i*i : limit+1 : i] = bytearray([0]) * len(range(i*i, limit+1, i))
    return [i for i, p in enumerate(is_prime) if p]

def mod_sqrt_minus1(p, k=2):
    # Find x such that x^2 = -1 mod p^k
    # Assume p = 1 mod 4
    if p == 2: return 1 # Should not happen for k>=2 in our usage
    
    # Tonelli-Shanks or similar for mod p
    # For p = 1 mod 4, -1 is a QR.
    # simple search for base g
    # r = g^((p-1)/4) mod p
    r = pow(2, (p - 1) // 4, p)
    if (r * r) % p != p - 1:
        for g in range(3, p):
            r = pow(g, (p - 1) // 4, p)
            if (r * r) % p == p - 1:
                break
                
    # Hensel Lift
    pk = p
    target_exp = k
    cur_exp = 1
    while cur_exp < target_exp:
        # r^2 = -1 mod p^j
        # New r' = r - f(r)*inv(f'(r))
        # f(x) = x^2 + 1
        # f'(x) = 2x
        # r' = r - (r^2+1) * inv(2r) mod p^(2j)
        
        step_mod = pk * pk # lifting from p^j to p^2j (quadratic convergence)
        if step_mod > p**target_exp:
            step_mod = p**target_exp
            
        term = (r * r + 1) // pk
        inv = pow(2 * r, -1, step_mod // pk * pk) # inverse mod p^j or similar?
        # Actually inverse mod p is enough for the first term of expansion?
        # Standard Hensel: r_{j+1} = r_j - (r_j^2+1) * inv(2r_j) mod p^{j+1}
        # With quadratic lift:
        inv = pow(2 * r, -1, step_mod)
        r = (r - (r * r + 1) * inv) % step_mod
        
        pk = step_mod
        cur_exp *= 2
        
    return min(r, pk - r)

def count_solutions(n, m_sq, sols):
    # Count x in [1, n] such that x in sols (mod m_sq)
    total = 0
    for a in sols:
        # solutions are in [0, m_sq-1]
        # if a=0, we want multiples of m_sq: m_sq, 2m_sq...
        if a == 0:
            if n >= m_sq:
                total += n // m_sq
        else:
            if a <= n:
                total += (n - a) // m_sq + 1
    return total

def solve_pell_minus1(k, max_x):
    # Solve x^2 - k y^2 = -1
    # Returns list of (x, y) with x <= max_x
    
    # Continued fraction for sqrt(k)
    m = 0
    d = 1
    a = int(math.isqrt(k))
    a0 = a
    if a * a == k:
        return [] # Perfect square, no solutions for -1 (0 - k y^2 = -1 impossible)
        
    # convergents
    num1, num2 = 1, 0
    den1, den2 = 0, 1
    
    # Period checking
    # We need to find period length L.
    # If L is even, no solution.
    # If L is odd, fundamental solution is convergent (p_{L-1}, q_{L-1})
    
    # Actually we can just generate convergents and check x^2 - k y^2
    # The first one to equal -1 is the fundamental solution.
    # If we encounter +1 first, then no solution for -1 exists.
    
    history = {}
    idx = 0
    
    while True:
        # Update convergents
        # p_n = a_n p_{n-1} + p_{n-2}
        num = a * num1 + num2
        den = a * den1 + den2
        
        # Check Pell equation
        # x = num, y = den
        val = num*num - k*den*den
        if val == -1:
            # Found fundamental solution
            fund_x, fund_y = num, den
            break
        if val == 1 and idx > 0:
            # Found +1 before -1, so no solution for -1
            return []
            
        num2, num1 = num1, num
        den2, den1 = den1, den
        
        # Next term in expansion
        m = d * a - m
        d = (k - m * m) // d
        a = (a0 + m) // d
        
        state = (m, d, a)
        if state in history:
             # Period detected and no -1 found yet (should have been caught by val==1 check usually)
             return []
        history[state] = idx
        idx += 1

    # We have fundamental solution (x1, y1)
    # Generate all solutions x <= max_x
    # (x + y sqrt(k)) = (x1 + y1 sqrt(k))^(2m-1)
    # alpha = x1 + y1 sqrt(k)
    # alpha^1 is sol
    # alpha^3, alpha^5...
    # Multiply by alpha^2 to get next
    
    sol_list = []
    
    cx, cy = fund_x, fund_y
    
    # Multiplier alpha^2
    # alpha^2 = (x1 + y1 sqrt(k))^2 = (x1^2 + k y1^2) + 2 x1 y1 sqrt(k)
    # x1^2 + k y1^2 = x1^2 + (x1^2 + 1) = 2 x1^2 + 1
    
    mul_x = 2 * fund_x * fund_x + 1
    mul_y = 2 * fund_x * fund_y
    
    while cx <= max_x:
        sol_list.append((cx, cy))
        
        # Next
        nx = cx * mul_x + cy * mul_y * k
        ny = cx * mul_y + cy * mul_x
        cx, cy = nx, ny
        
    return sol_list

def get_mu_if_squarefree(n, primes):
    # Check if n is square-free and compute mu(n)
    # n can be large (up to 10^11)
    # primes contains all primes up to sqrt(n) (actually up to 3.5e5)
    
    mu = 1
    temp = n
    limit = int(math.isqrt(n))
    
    for p in primes:
        if p * p > temp:
            break
        if temp % p == 0:
            temp //= p
            mu = -mu
            if temp % p == 0:
                return False, 0
    
    if temp > 1:
        mu = -mu
        
    return True, mu

if __name__ == "__main__":
    solve()
