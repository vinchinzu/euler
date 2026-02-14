/* Project Euler Problem 111: Primes with Runs */
#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>

/* Miller-Rabin primality test for numbers up to ~10^10 */
typedef unsigned long long ull;
typedef __int128 i128;

static ull mulmod(ull a, ull b, ull m) {
    return (i128)a * b % m;
}

static ull powmod(ull base, ull exp, ull mod) {
    ull result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = mulmod(result, base, mod);
        base = mulmod(base, base, mod);
        exp >>= 1;
    }
    return result;
}

static bool miller_rabin(ull n) {
    if (n < 2) return false;
    if (n < 4) return true;
    if (n % 2 == 0) return false;

    ull d = n - 1;
    int r = 0;
    while (d % 2 == 0) { d /= 2; r++; }

    /* For n < 3.2 * 10^14, testing with {2, 3, 5, 7, 11, 13, 17} suffices */
    ull witnesses[] = {2, 3, 5, 7, 11, 13, 17};
    int nw = 7;

    for (int w = 0; w < nw; w++) {
        ull a = witnesses[w];
        if (a >= n) continue;
        ull x = powmod(a, d, n);
        if (x == 1 || x == n - 1) continue;
        bool composite = true;
        for (int i = 0; i < r - 1; i++) {
            x = mulmod(x, x, n);
            if (x == n - 1) { composite = false; break; }
        }
        if (composite) return false;
    }
    return true;
}

/* For n=10 digits: enumerate patterns where digit d appears k_repeats times */
/* Positions: 0..9. Place d in k_repeats positions, other digits != d in remaining. */

#define N 10

/* Combinations C(10, k) positions for the d-digit */
static int combo_buf[252][N]; /* C(10,5) = 252 max */
static int combo_count;

static void gen_combos(int n, int k, int start, int *current, int depth) {
    if (depth == k) {
        for (int i = 0; i < k; i++)
            combo_buf[combo_count][i] = current[i];
        combo_count++;
        return;
    }
    for (int i = start; i < n; i++) {
        current[depth] = i;
        gen_combos(n, k, i + 1, current, depth + 1);
    }
}

int main(void) {
    long long total_s_sum = 0;

    for (int d = 0; d <= 9; d++) {
        long long s_n_d = 0;

        for (int k_repeats = N; k_repeats >= 1; k_repeats--) {
            int num_other = N - k_repeats;
            long long current_sum = 0;
            int current_count = 0;

            /* Generate all ways to choose k_repeats positions for digit d */
            combo_count = 0;
            int current[N];
            gen_combos(N, k_repeats, 0, current, 0);

            for (int ci = 0; ci < combo_count; ci++) {
                /* Determine other positions */
                int d_pos[N], other_pos[N];
                bool is_d[N];
                memset(is_d, 0, sizeof(is_d));
                for (int i = 0; i < k_repeats; i++) {
                    d_pos[i] = combo_buf[ci][i];
                    is_d[combo_buf[ci][i]] = true;
                }
                int other_count = 0;
                for (int i = 0; i < N; i++)
                    if (!is_d[i]) other_pos[other_count++] = i;

                if (num_other == 0) {
                    /* All digits are d */
                    if (d == 0) continue; /* 0000000000 is not valid */
                    ull num = 0;
                    for (int i = 0; i < N; i++)
                        num = num * 10 + d;
                    if (miller_rabin(num)) {
                        current_sum += (long long)num;
                        current_count++;
                    }
                } else {
                    /* Enumerate all sequences of 'num_other' digits != d */
                    /* Other digits: 0-9 except d => 9 choices per position */
                    int cand[9], cand_count = 0;
                    for (int x = 0; x <= 9; x++)
                        if (x != d) cand[cand_count++] = x;

                    /* Total: 9^num_other combinations */
                    int total_other = 1;
                    for (int i = 0; i < num_other; i++) total_other *= 9;

                    for (int t = 0; t < total_other; t++) {
                        int digits[N];
                        for (int i = 0; i < k_repeats; i++)
                            digits[d_pos[i]] = d;

                        int tmp = t;
                        for (int i = 0; i < num_other; i++) {
                            digits[other_pos[i]] = cand[tmp % 9];
                            tmp /= 9;
                        }

                        /* No leading zero */
                        if (digits[0] == 0) continue;

                        ull num = 0;
                        for (int i = 0; i < N; i++)
                            num = num * 10 + digits[i];

                        if (miller_rabin(num)) {
                            current_sum += (long long)num;
                            current_count++;
                        }
                    }
                }
            }

            if (current_count > 0) {
                s_n_d = current_sum;
                break;
            }
        }

        total_s_sum += s_n_d;
    }

    printf("%lld\n", total_s_sum);
    return 0;
}
