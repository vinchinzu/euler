"""
Project Euler Problem 944: Sum of Elevisors

S(n) is the sum of sev(E) for all subsets E of {1, 2, ..., n}.
sev(E) is the sum of all "elevisors" of E.
An element x in E is an elevisor if it divides another element of E.

Find S(10^14) mod 1234567891
"""

import math

def solve():
    n = 10**14
    mod = 1234567891
    mod_exp = mod - 1  # For Fermat's Little Theorem exponentiation
    s = int(math.sqrt(n))

    # Calculate the first part of the sum: from d = 2 to s
    sum1 = 0
    n_mod_exp = n % mod_exp
    for d in range(2, s + 1):
        n_div_d = n // d
        # Calculate C(floor(n/d)) = floor(n/d) * (floor(n/d) + 1) / 2
        c_val = n_div_d * (n_div_d + 1) // 2
        c_val %= mod

        # Calculate 2^(n-d) mod mod
        exp = (n_mod_exp - d % mod_exp + mod_exp) % mod_exp
        power_val = pow(2, exp, mod)

        term = (power_val * c_val) % mod
        sum1 = (sum1 + term) % mod

    # Calculate the second part of the sum: from d = s+1 to n
    # This part is transformed to a sum over k
    k_max = n // (s + 1)

    # Calculate the term C(k_max) * 2^(n - s)
    c_k_max = k_max * (k_max + 1) // 2
    c_k_max %= mod
    exp_s = (n - s) % mod_exp
    term1 = (c_k_max * pow(2, exp_s, mod)) % mod

    # Calculate the sum of k * 2^(n - floor(n/k)) for k from 1 to k_max
    sum_k = 0
    for k in range(1, k_max + 1):
        n_div_k = n // k
        # Exponent is (n - floor(n/k)) mod (mod - 1)
        exp = (n - n_div_k) % mod_exp
        power_val = pow(2, exp, mod)
        term_k = (k * power_val) % mod
        sum_k = (sum_k + term_k) % mod

    sum2 = (term1 - sum_k + mod) % mod

    # The final result is the sum of the two parts
    result = (sum1 + sum2) % mod
    return result

def main():
    result = solve()
    print(result)
    return result

if __name__ == "__main__":
    main()
