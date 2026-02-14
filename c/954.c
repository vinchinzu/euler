/*
 * Project Euler Problem 954
 * Count numbers up to 10^13 that are not divisible by 7 and remain
 * not divisible by 7 after swapping any two digits.
 *
 * Digit DP: for each length L (1..13), count L-digit numbers (no leading zeros)
 * where mod 7 != 0 and no swap of two digits makes it divisible by 7.
 *
 * A swap of digits at positions i,j changes the number by
 * (d_i - d_j) * (10^{L-1-i} - 10^{L-1-j}).
 * The number n + delta is divisible by 7 iff n ≡ -(d_i-d_j)*diff (mod 7).
 * So "is_bad" when there exist i<j such that for the diff value,
 * (d_i - d_j) * diff ≡ -n (mod 7).
 */
#include <stdio.h>
#include <string.h>

static int mod_inverse_table[7]; /* inverse of x mod 7 for x=1..6 */

static void init_inv(void) {
    for (int x = 1; x < 7; x++)
        for (int y = 1; y < 7; y++)
            if ((x * y) % 7 == 1) { mod_inverse_table[x] = y; break; }
}

static int pow10_mod7[14]; /* 10^i mod 7 */

static void init_pow10(void) {
    pow10_mod7[0] = 1;
    for (int i = 1; i <= 13; i++)
        pow10_mod7[i] = (pow10_mod7[i - 1] * 10) % 7;
}

/* diff_table[L][i][j] = (10^{L-1-j} * (10^{p_i-p_j} - 1)) mod 7
   where p_i = L-1-i, p_j = L-1-j */
static int diff_table[14][14][14];

static void init_diff(void) {
    for (int L = 1; L <= 13; L++) {
        for (int i = 0; i < L; i++) {
            for (int j = i + 1; j < L; j++) {
                int p_i = L - 1 - i;
                int p_j = L - 1 - j;
                int val = (pow10_mod7[p_j] * ((pow10_mod7[p_i - p_j] - 1 + 7) % 7)) % 7;
                diff_table[L][i][j] = val;
            }
        }
    }
}

/*
 * Digit DP with memoization.
 * State: (pos, mod, is_bad, tuple of digits so far)
 *
 * The key insight: we need to track the digits themselves because
 * the "is_bad" check involves comparing each new digit against all previous.
 * But for L=13, tracking full digit tuple is too expensive.
 *
 * Alternative: Once is_bad is set, we don't need digits anymore.
 * When is_bad is NOT set, we need to check each new digit against all previous.
 *
 * For the not-bad path, we need to track digit residues.
 * Actually, we only need (d_k mod 7) for each position k, which gives 7^pos states.
 * For L=13: 7^13 ~ 96 billion - too large.
 *
 * Better approach: precompute for each (L, pos, mod, is_bad):
 * - if is_bad: count = 6 * 10^(remaining-1) * ... just count freely
 *   Actually if is_bad, any digit for remaining positions, just ensure final mod != 0.
 *   Count of numbers with specific prefix mod and remaining r digits: (10^r / 7) * 6 ≈
 *
 * Hmm, the Python solution does full recursion with memoization including digit tuples.
 * For L up to 13, and digits tuple up to 13 digits, the state space per L is huge.
 *
 * Let me re-examine: the Python stores (pos, tight, leading, mod, is_bad, digits)
 * and tight is always False (since we're counting L-digit numbers for each L, not up to a bound).
 * Also leading is only relevant at pos=0.
 *
 * The key: once is_bad becomes true, we don't need digits anymore.
 * State becomes (pos, mod, is_bad=true) and we can compute freely:
 * For remaining r positions, count = (number of r-digit extensions with final mod != 0).
 * That's (10^r - count_with_mod_m) where mod_m + current_mod = 0 mod 7.
 *
 * For the NOT-bad path, we DO need the full digit history.
 * But the check only uses (d_k - d_new) mod 7 and diff_table values.
 * So we only need the digits mod 7, not the actual digits.
 * State: (pos, mod, digit_residues_mod7) for the not-bad path.
 * digit_residues can be encoded as a sorted histogram of residues mod 7.
 * Actually we need positions, not just counts, because diff_table[L][k][pos] depends on k and pos.
 *
 * Given the Python solution works with full digits tuple and L up to 13, it must be feasible
 * with caching. Let me just do a direct recursive approach with hash-map caching in C.
 *
 * Actually, looking more carefully at the Python: it caches on
 * (pos, tight, leading, mod, is_bad, digits).
 * Since tight=False always and leading only matters at pos 0, the real key is
 * (pos, mod, is_bad, digits).
 * For is_bad=true, digits is irrelevant, so those states are just (pos, mod): 13*7 = 91 states,
 * each counting 10^(remaining) completions.
 *
 * For is_bad=false, we need digits. Since digits is a tuple of actual digits 0-9,
 * and we have up to 13 of them... that's potentially 10^13 states which is too many.
 *
 * BUT: the actual number of reachable (digits, is_bad=false) states is much smaller because
 * many digit sequences will trigger is_bad early. Let's just implement it recursively with
 * memoization similar to the Python but more efficiently.
 *
 * WAIT: Re-reading the check more carefully - the Python stores the full digit tuple AND
 * checks forbidden values using diff_table and current mod. The digits tuple affects which
 * future digits are forbidden.
 *
 * Key optimization: we only need the digit values mod 7 at each position, since the check is:
 *   (d - digits[k]) mod 7 == forbidden_value
 * where forbidden_value depends on diff_table[L][k][pos] and current mod.
 * But the position matters because diff_table depends on (k, pos).
 *
 * Let me think differently. For each position pos and digit d, the check against all previous
 * positions k is: does (d - d_k) * diff[k][pos] ≡ -mod (mod 7)?
 * This is: d_k ≡ d + mod * inv(diff[k][pos]) (mod 7) if diff[k][pos] != 0.
 * If diff[k][pos] == 0 and mod == 0, then bad.
 *
 * The important thing: we need to know, for each previous position k, what d_k mod 7 was.
 * So the state includes the sequence of (d_0 mod 7, d_1 mod 7, ..., d_{pos-1} mod 7).
 * That's 7^pos possibilities per position. For pos up to 12, 7^12 ~ 13 billion - too much.
 *
 * Alternative: We can group positions by their diff_table value.
 * For a given L and current pos, diff_table[L][k][pos] for k=0..pos-1 takes values in {0,1,...,6}.
 * What matters is: for each residue class r (0..6), how many previous positions k
 * have d_k ≡ r (mod 7) AND diff_table[L][k][pos] = some specific value.
 *
 * Actually, let's just follow the Python approach since it works for L up to 13.
 * The Python likely works because the recursive calls with is_bad=false prune heavily.
 *
 * For a direct C implementation, I'll use recursion with a hash map.
 * But a simpler approach: since is_bad transitions to true and stays true, and
 * once true we only need (pos, mod), let's compute the "bad" contribution analytically
 * and focus the DP on the "not bad" path.
 *
 * For the not-bad DP, I'll store digit residues at each position.
 * State: position-specific digit residues. But that's expensive.
 *
 * Simplest correct approach: just do the recursion for each L, keeping the digit tuple
 * in the recursion, with a hash map for memoization.
 *
 * Actually, let me reconsider the state space. The Python memoizes on
 * (pos, tight, leading, mod, is_bad, digits). Here tight=False, leading only at start.
 * So the key is (pos, mod, is_bad, digits_tuple).
 * For is_bad=True, the remaining count can be precomputed.
 * For is_bad=False, digits_tuple has up to 13 digits 0-9.
 *
 * The number of distinct is_bad=False digit tuples is at most 10^13 in theory,
 * but in practice with the pruning of is_bad transitions, it should be manageable.
 *
 * Actually, the Python solution runs fine for L up to 13, so let me just do a
 * straightforward recursive implementation in C without complex memoization.
 * For each L, the recursion tree has at most 10^L leaves, but heavy pruning from
 * the is_bad check means it's much smaller. With L=13, even 10^8 operations should be fast.
 *
 * Let me just do a recursive DFS for each L, tracking digits.
 */

static long long count_free[14][7]; /* count_free[r][m] = number of r-digit suffixes with sum ≡ m mod 7 */

static void init_count_free(void) {
    /* count_free[0][0] = 1, rest 0 */
    memset(count_free, 0, sizeof(count_free));
    count_free[0][0] = 1;
    for (int r = 1; r <= 13; r++) {
        for (int m = 0; m < 7; m++) {
            for (int d = 0; d <= 9; d++) {
                int prev_m = (m - (d * pow10_mod7[r-1]) % 7 + 7) % 7;
                count_free[r][m] += count_free[r-1][prev_m];
            }
        }
    }
}

static int cur_L;
static int digits[14];
static long long total_count;

static void dfs(int pos, int mod7, int is_bad) {
    if (pos == cur_L) {
        if (mod7 != 0 && !is_bad) {
            total_count++;
        }
        return;
    }

    int start_d = (pos == 0) ? 1 : 0;

    for (int d = start_d; d <= 9; d++) {
        int new_mod = (mod7 + d * pow10_mod7[cur_L - 1 - pos]) % 7;
        int new_bad = is_bad;

        if (!is_bad) {
            for (int k = 0; k < pos; k++) {
                int diff_val = diff_table[cur_L][k][pos];
                if (diff_val == 0) {
                    if (new_mod == 0) {
                        new_bad = 1;
                        break;
                    }
                } else {
                    int inv_d = mod_inverse_table[diff_val];
                    int forbidden = (((7 - new_mod % 7) % 7) * inv_d) % 7;
                    int diff_dk = ((d - digits[k]) % 7 + 7) % 7;
                    if (diff_dk == forbidden) {
                        new_bad = 1;
                        break;
                    }
                }
            }
        }

        if (new_bad) {
            /* Once bad, count all completions with final mod != 0 */
            int remaining = cur_L - pos - 1;
            /* We need total 10^remaining minus those ending with mod == 0 */
            /* Current new_mod is the mod after placing digit d at position pos */
            /* Need to add remaining digits and avoid final mod == 0 */
            /* target_remaining_mod = (7 - new_mod) % 7 -- this is what remaining digits should NOT sum to */
            int avoid_mod = (7 - new_mod % 7) % 7;
            long long total_rem = 1;
            for (int r = 0; r < remaining; r++) total_rem *= 10;
            long long bad_rem = count_free[remaining][avoid_mod];
            total_count += total_rem - bad_rem;
        } else {
            digits[pos] = d;
            dfs(pos + 1, new_mod, 0);
        }
    }
}

int main(void) {
    init_inv();
    init_pow10();
    init_diff();
    init_count_free();

    long long grand_total = 0;

    for (int L = 1; L <= 13; L++) {
        cur_L = L;
        total_count = 0;
        memset(digits, 0, sizeof(digits));
        dfs(0, 0, 0);
        grand_total += total_count;
    }

    printf("%lld\n", grand_total);
    return 0;
}
