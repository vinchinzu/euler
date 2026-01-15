# Project Euler Problem 956
#
# PROBLEM DESCRIPTION:
# <p>
# The total number of prime factors of $n$, counted with multiplicity, is denoted $\Omega(n)$.<br>
# For example, $\Omega(12)=3$, counting the factor $2$ twice, and the factor $3$ once.</p>
# 
# <p>
# Define $D(n, m)$ to be the sum of all divisors $d$ of $n$ where $\Omega(d)$ is divisible by $m$. <br>
# For example, $D(24, 3)=1+8+12=21$.</p>
# 
# <p>
# The <b>superfactorial</b> of $n$, often written as $n\$$, is defined as the product of the first $n$ factorials:
# $$n\$=1!\times 2! \times\cdots\times n!$$
# The <b>superduperfactorial</b> of $n$, we write as $n\bigstar$, is defined as the product of the first $n$ superfactorials:
# $$n\bigstar=1\$ \times 2\$ \times\cdots\times n\$ $$
# </p>
# 
# <p>
# You are given $D(6\bigstar, 6)=6368195719791280$.</p>
# 
# <p>
# Find $D(1\,000\bigstar, 1\,000)$. 
# Give your answer modulo $999\,999\,001$.</p>
#

import math

def sieve(n):
    is_prime = [True]*(n+1)
    is_prime[0]=is_prime[1]=False
    for i in range(2, int(math.sqrt(n))+1):
        if is_prime[i]:
            for j in range(i*i, n+1, i):
                is_prime[j]=False
    return [i for i in range(2,n+1) if is_prime[i]]

def mod_inverse(a, m):
    return pow(a, -1, m)

def is_primitive_root(g, mod, primes_factors):
    phi = mod - 1
    for q in primes_factors:
        if pow(g, phi // q, mod) == 1:
            return False
    return True

mod = 999999001
phi = mod - 1
factors = [2,3,5,7,11,13,37]

# find primitive root
g = 2
while not is_primitive_root(g, mod, factors):
    g += 1

n = 1000
m = 1000
primes = sieve(n)
exponents = []
for pp in primes:
    e = 0
    for k in range(1, n+1):
        vk = 0
        pk = pp
        while pk <= k:
            vk += k // pk
            pk *= pp
        e += (n - k + 1) * vk
    exponents.append(e)

# now omega = g^{phi / m}
omega = pow(g, phi // m, mod)

inv_m = mod_inverse(m, mod)

sum_S = 0
for j in range(m):
    y = pow(omega, j, mod)
    prod = 1
    for idx in range(len(primes)):
        pp = primes[idx]
        e = exponents[idx]
        r = (pp * y) % mod
        if r == 1:
            geo = (e + 1) % mod
        else:
            exp_mod = (e + 1) % phi
            re_plus1 = pow(r, exp_mod, mod)
            num = (1 - re_plus1 + mod) % mod
            den = (1 - r + mod) % mod
            den_inv = mod_inverse(den, mod)
            geo = (num * den_inv) % mod
        prod = (prod * geo) % mod
    sum_S = (sum_S + prod) % mod

result = (sum_S * inv_m) % mod
print(result)
