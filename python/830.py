import sys

def compute_binom_mod(n, m, mod):
    if m == 0:
        return 1 % mod
    res = 1
    for i in range(1, m + 1):
        res *= (n - m + i)
        res //= i
    return res % mod

def build_bin_table(max_m, mod):
    bin_table = [[0] * (max_m + 1) for _ in range(max_m + 1)]
    for i in range(max_m + 1):
        bin_table[i][0] = 1
        for j in range(1, min(i, max_m) + 1):
            bin_table[i][j] = (bin_table[i - 1][j - 1] + bin_table[i - 1][j]) % mod
    return bin_table

def compute_s_mod_p3(p, n):
    mod = p ** 3
    phi = p ** 2 * (p - 1)
    e = pow(10, 18, phi)
    if e < 3:
        raise ValueError(f"e = {e} < 3 for p={p}, special handling needed")
    max_m = 3 * p - 1
    bin_table = build_bin_table(max_m, mod)
    total = 0
    for m in range(max_m + 1):
        sum_se = 0
        for j in range(m + 1):
            sign = 1 if (m - j) % 2 == 0 else -1
            je = 0 if j == 0 else pow(j, e, mod)
            term = (bin_table[m][j] * je % mod * sign) % mod
            sum_se = (sum_se + term) % mod
        bnm = compute_binom_mod(n, m, mod)
        tw = pow(2, n - m, mod)
        term = (sum_se * bnm % mod) * tw % mod
        total = (total + term) % mod
    return total

def crt(as_list, ms_list):
    assert len(as_list) == len(ms_list) == 3
    M = ms_list[0] * ms_list[1] * ms_list[2]
    x = 0
    for i in range(3):
        Mi = M // ms_list[i]
        inv = pow(Mi, -1, ms_list[i])
        x += as_list[i] * Mi * inv
    return x % M

n = 10**18
ps = [83, 89, 97]
ss = []
mods = []
for p in ps:
    s = compute_s_mod_p3(p, n)
    ss.append(s)
    mods.append(p**3)

answer = crt(ss, mods)
print(answer)
