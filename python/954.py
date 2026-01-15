import sys

def mod_inverse(a, m):
    m0 = m
    y = 0
    x = 1
    if m == 1:
        return 0
    while a > 1:
        q = a // m
        t = m
        m = a % m
        a = t
        t = y
        y = x - q * y
        x = t
    if x < 0:
        x += m0
    return x

def precompute_diff(max_L):
    diff = [[ [0 for _ in range(max_L)] for _ in range(max_L)] for _ in range(max_L+1)]
    pow10 = [1]
    for i in range(max_L):
        pow10.append((pow10[-1] * 10) % 7)
    for L in range(1, max_L+1):
        for i in range(L):
            for j in range(i+1, L):
                p_i = L - 1 - i
                p_j = L - 1 - j
                diff_val = (pow10[p_j] * ((pow10[p_i - p_j] - 1) % 7)) % 7
                diff[L][i][j] = diff_val
    return diff

def count_for_L(L, diff):
    memo = {}
    def dp(pos, tight, leading, mod, is_bad, digits):
        if pos == L:
            if not leading and mod != 0 and not is_bad:
                return 1
            return 0
        key = (pos, tight, leading, mod, is_bad, digits)
        if key in memo:
            return memo[key]
        res = 0
        up = 9 if not tight else 9  # since tight=False for all
        for d in range(up + 1):
            new_tight = tight and (d == up)
            new_leading = leading and (d == 0)
            if pos == 0 and d == 0:
                continue
            new_mod = (mod * 10 + d) % 7
            new_is_bad = is_bad
            if not is_bad:
                for k in range(pos):
                    diff_val = diff[L][k][pos]
                    if diff_val == 0:
                        if new_mod == 0:
                            new_is_bad = True
                            break
                    else:
                        inv_d = mod_inverse(diff_val, 7)
                        forbidden = ((-new_mod % 7) * inv_d) % 7
                        diff_dk = (d - digits[k]) % 7
                        if diff_dk == forbidden:
                            new_is_bad = True
                            break
            new_digits = digits + (d,)
            res += dp(pos + 1, new_tight, new_leading, new_mod, new_is_bad, new_digits)
        memo[key] = res
        return res
    return dp(0, False, True, 0, False, ())

def main():
    max_L = 13
    diff = precompute_diff(max_L)
    total = 0
    for L in range(1, 14):
        total += count_for_L(L, diff)
    print(total)

if __name__ == "__main__":
    main()