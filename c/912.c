/* Project Euler Problem 912 - Digit DP with no consecutive three 1s
 * F(N) = sum of n^2 for n <= N where s_n is odd.
 * Binary strings without three consecutive 1s, ranked and summed mod 10^9+7. */
#include <stdio.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;
#define MOD 1000000007LL

/* exact_dp[s][r] = exact count of valid completions with state s, r bits remaining */
/* s: trailing 1s (0,1,2), r: remaining bits (0..79) */
static ull exact_count[3][80];
static int exact_done[3][80];

static ull exact_dp(int s, int r) {
    if (exact_done[s][r]) return exact_count[s][r];
    exact_done[s][r] = 1;
    if (r == 0) {
        exact_count[s][r] = 1;
        return 1;
    }
    ull result = exact_dp(0, r - 1);
    if (s < 2) result += exact_dp(s + 1, r - 1);
    exact_count[s][r] = result;
    return result;
}

/* mod DP: returns (count, odd_count, sum_odd_pos, sum_odd_pos_sq) */
typedef struct { ll cnt, odd, sop, sopq; } DPResult;
static DPResult mod_memo[3][80];
static int mod_done[3][80];

static DPResult dp(int s, int r) {
    if (mod_done[s][r]) return mod_memo[s][r];
    mod_done[s][r] = 1;
    DPResult res;
    if (r == 0) {
        int is_odd = (s >= 1) ? 1 : 0;
        res.cnt = 1; res.odd = is_odd; res.sop = is_odd; res.sopq = is_odd;
        mod_memo[s][r] = res;
        return res;
    }
    DPResult L = dp(0, r - 1);
    DPResult R_val = {0, 0, 0, 0};
    if (s < 2) R_val = dp(s + 1, r - 1);

    res.cnt = (L.cnt + R_val.cnt) % MOD;
    res.odd = (L.odd + R_val.odd) % MOD;
    ll cl = L.cnt % MOD;
    res.sop = (L.sop + R_val.sop + cl % MOD * R_val.odd) % MOD;
    res.sopq = (L.sopq + R_val.sopq + 2 * cl % MOD * R_val.sop % MOD + cl * cl % MOD * R_val.odd) % MOD;
    res.sopq %= MOD;

    mod_memo[s][r] = res;
    return res;
}

int main(void) {
    ull N = 10000000000000000ULL; /* 10^16 */

    memset(exact_done, 0, sizeof(exact_done));
    memset(mod_done, 0, sizeof(mod_done));

    /* Precompute */
    for (int b = 1; b < 80; b++)
        exact_dp(1, b - 1);

    ull remaining_N = N;
    ull R_offset = 0;
    ll ans = 0;

    for (int b = 1; b < 80; b++) {
        ull total_b = exact_dp(1, b - 1);

        if (total_b <= remaining_N) {
            DPResult d = dp(1, b - 1);
            ll R_off_mod = (ll)(R_offset % MOD);
            ll contribution = (d.odd * R_off_mod % MOD * R_off_mod % MOD
                             + 2 * R_off_mod % MOD * d.sop % MOD
                             + d.sopq) % MOD;
            ans = (ans + contribution) % MOD;
            R_offset += total_b;
            remaining_N -= total_b;
        } else {
            /* Partial processing of b-bit numbers */
            int s = 1;
            ull local_offset = 0;

            for (int bit_pos = b - 2; bit_pos >= 0; bit_pos--) {
                ull cnt_0 = exact_dp(0, bit_pos);

                if (remaining_N <= cnt_0) {
                    s = 0;
                    continue;
                } else {
                    if (cnt_0 > 0) {
                        DPResult d = dp(0, bit_pos);
                        ll R_off_mod = (ll)((R_offset + local_offset) % MOD);
                        ll contribution = (d.odd * R_off_mod % MOD * R_off_mod % MOD
                                         + 2 * R_off_mod % MOD * d.sop % MOD
                                         + d.sopq) % MOD;
                        ans = (ans + contribution) % MOD;
                    }
                    remaining_N -= cnt_0;
                    local_offset += cnt_0;

                    if (s >= 2) break;
                    s = s + 1;
                }
            }

            if (remaining_N > 0) {
                int is_odd = (s >= 1) ? 1 : 0;
                if (is_odd) {
                    ll rank = (ll)((R_offset + local_offset + 1) % MOD);
                    ans = (ans + rank * rank % MOD) % MOD;
                }
                remaining_N -= 1;
            }
            break;
        }
    }

    ans = ((ans % MOD) + MOD) % MOD;
    printf("%lld\n", ans);
    return 0;
}
