#!/usr/bin/env python3
"""Project Euler Problem 685: Inverse Digit Sum II.

f(n,m) = m-th smallest number with digit sum n.
Find S(10000) = sum_{n=1}^{10000} f(n^3, n^4) mod 10^9+7.

Key insight: numbers with digit sum S have at minimum L=ceil(S/9) digits.
For m-th number, most digits are 9 with at most 8 positions deviating
(deficit approach). Uses combinatorial counting with fast position-skipping
via binary search for large digit counts.
"""

import subprocess, tempfile, os

C_CODE = r"""
#include <stdio.h>
typedef long long ll;
typedef __int128 lll;
static const ll MOD = 1000000007;

ll power_mod(ll base, ll exp, ll mod) {
    ll result = 1; base %= mod; if (base < 0) base += mod;
    while (exp > 0) { if (exp & 1) result = (lll)result * base % mod; base = (lll)base * base % mod; exp >>= 1; }
    return result;
}
ll mod_inv(ll a, ll mod) { return power_mod(a, mod - 2, mod); }

ll comb_exact(ll n, int r, ll cap) {
    if (r < 0) return 0;
    if (r == 0) return 1;
    if (n < r) return 0;
    if (r > n - r && n - r >= 0 && n - r <= 18) r = (int)(n - r);
    ll result = 1;
    for (int i = 0; i < r; i++) {
        if (n - i <= 0) return 0;
        if (result > cap / (n - i) + 1) return -1;
        result *= (n - i); result /= (i + 1);
        if (result > cap) return -1;
    }
    return result;
}

ll count_digits(ll P, ll K_ll, ll cap) {
    if (P == 0) return (K_ll == 0) ? 1 : 0;
    if (K_ll < 0 || K_ll > 9 * P) return 0;
    ll deficit = 9 * P - K_ll;
    ll ek = (K_ll < deficit) ? K_ll : deficit;
    if (ek <= 9) return comb_exact(P - 1 + ek, (int)ek, cap);
    ll total = 0;
    int ek_int = (int)ek;
    for (int j = 0; j <= ek_int / 10 && j <= P; j++) {
        ll c1 = comb_exact(P, j, cap); if (c1 == -1) { if (j%2==0) return -1; continue; }
        ll c2 = comb_exact(P - 1 + ek_int - 10*j, ek_int - 10*j, cap); if (c2 == -1) { if (j%2==0) return -1; continue; }
        if (c1 > cap / (c2 + 1)) { if (j%2==0) return -1; continue; }
        if (j % 2 == 0) { total += c1 * c2; if (total > cap) return -1; }
        else total -= c1 * c2;
    }
    return total;
}

ll find_number_small(int d, ll rem_sum, int rem_pos, ll m) {
    ll val = d % MOD;
    ll cur_sum = rem_sum;
    for (int pos = 0; pos < rem_pos; pos++) {
        val = val * 10 % MOD;
        int max_dig = (cur_sum > 9) ? 9 : (int)cur_sum;
        for (int dig = 0; dig <= max_dig; dig++) {
            ll new_sum = cur_sum - dig;
            int new_pos = rem_pos - pos - 1;
            if (new_sum > 9LL * new_pos) continue;
            ll cnt = count_digits(new_pos, new_sum, (ll)2e18);
            if (cnt == -1 || m < cnt) {
                val = (val + dig) % MOD;
                cur_sum = new_sum;
                break;
            }
            m -= cnt;
        }
    }
    return val;
}

ll find_deficit(ll D, int d, ll P, int deficit, ll m) {
    if (deficit == 0) {
        ll val = (lll)(d + 1) % MOD * power_mod(10, P, MOD) % MOD;
        return (val - 1 + MOD) % MOD;
    }
    int num_slots = 0;
    ll slot_pos[10]; int slot_def[10];
    ll cur_P = P; int cur_K = deficit; ll cur_m = m; ll offset = 0;
    while (cur_K > 0 && cur_P > 0) {
        ll total_ge1 = comb_exact(cur_P - 2 + cur_K, cur_K - 1, (ll)2e18);
        if (total_ge1 != -1 && cur_m >= total_ge1) {
            cur_m -= total_ge1;
            if (cur_K == 1) {
                ll skip = cur_m;
                if (skip > cur_P - 2) skip = cur_P - 2;
                cur_m -= skip;
                offset += 1 + skip; cur_P -= 1 + skip;
            } else if (cur_K == 2) {
                ll lo = 0, hi = cur_P - 1 - cur_K;
                if (hi < 0) hi = 0;
                while (lo < hi) {
                    ll mid = lo + (hi - lo + 1) / 2;
                    lll s = (lll)(mid+1) * (2*(cur_P-1) - mid) / 2;
                    if (s <= (lll)cur_m) lo = mid; else hi = mid - 1;
                }
                lll s = (lll)(lo+1) * (2*(cur_P-1) - lo) / 2;
                cur_m -= (ll)s;
                offset += 1 + lo + 1; cur_P -= 1 + lo + 1;
            } else {
                offset++; cur_P--;
            }
        } else {
            for (int j = cur_K; j >= 1; j--) {
                ll cnt2 = comb_exact(cur_P - 2 + (cur_K - j), cur_K - j, (ll)2e18);
                if (cnt2 == -1 || cur_m < cnt2) {
                    slot_pos[num_slots] = offset; slot_def[num_slots] = j;
                    num_slots++; offset++; cur_P--; cur_K -= j; break;
                }
                cur_m -= cnt2;
            }
        }
    }
    ll val = (lll)(d + 1) % MOD * power_mod(10, D - 1, MOD) % MOD;
    val = (val - 1 + MOD) % MOD;
    for (int i = 0; i < num_slots; i++) {
        ll place = D - 2 - slot_pos[i];
        if (place < 0) continue;
        ll sub = (lll)slot_def[i] % MOD * power_mod(10, place, MOD) % MOD;
        val = (val - sub + MOD) % MOD;
    }
    return val;
}

ll f_func(ll S, ll m) {
    if (S <= 0) return 0;
    ll L; int r;
    if (S % 9 == 0) { L = S / 9; r = 9; } else { L = S / 9 + 1; r = (int)(S % 9); }
    m--;
    for (ll D = L; D <= L + 5; D++) {
        int d_start = (D == L) ? r : 1;
        for (int d = d_start; d <= 9; d++) {
            ll P = D - 1;
            ll rem_sum = S - d;
            if (rem_sum < 0 || rem_sum > 9 * P) continue;
            ll deficit = 9 * P - rem_sum;
            ll cnt;
            if (deficit <= 8) cnt = comb_exact(P - 1 + deficit, (int)deficit, (ll)2e18);
            else cnt = count_digits(P, rem_sum, (ll)2e18);
            if (cnt == -1 || m < cnt) {
                if (deficit <= 8 && P > 50) return find_deficit(D, d, P, (int)deficit, m);
                else if (P <= 50) return find_number_small(d, rem_sum, (int)P, m);
                else return -1;
            }
            m -= cnt;
        }
    }
    return -1;
}

int main() {
    ll total = 0;
    for (int n = 1; n <= 10000; n++) {
        ll S = (ll)n * n * n;
        ll m = (ll)n * n * n * n;
        total = (total + f_func(S, m)) % MOD;
    }
    printf("%lld\n", total);
    return 0;
}
"""

def solve():
    src = tempfile.NamedTemporaryFile(suffix='.c', delete=False, mode='w')
    src.write(C_CODE)
    src.close()
    exe = src.name.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', exe, src.name], check=True, capture_output=True)
        os.unlink(src.name)
        proc = subprocess.run([exe], capture_output=True, text=True, timeout=60)
        return int(proc.stdout.strip())
    finally:
        if os.path.exists(exe): os.unlink(exe)
        if os.path.exists(src.name): os.unlink(src.name)

if __name__ == "__main__":
    print(solve())
