/* Project Euler Problem 928 - Cribbage Scoring
 * Enumerate all multisets of cards, compute hand score and cribbage score.
 * Count hands where hand_score == cribbage_score.
 */
#include <stdio.h>
#include <string.h>

typedef long long ll;

#define NRANKS 13
#define MAX_COUNT 4
#define TARGET_SUM 15

int rank_value(int rank) {
    /* rank 0=Ace(1), ..., 9=10, 10=J(10), 11=Q(10), 12=K(10) */
    if (rank == 0) return 1;
    if (rank >= 9) return 10;
    return rank + 1;
}

ll binomial(int n, int k) {
    if (k < 0 || k > n) return 0;
    if (k == 0 || k == n) return 1;
    if (k > n - k) k = n - k;
    ll res = 1;
    for (int i = 0; i < k; i++) {
        res = res * (n - i) / (i + 1);
    }
    return res;
}

ll suit_combinations(int count) {
    return binomial(4, count);
}

int counts[NRANKS];

ll calculate_pairs_score(void) {
    ll score = 0;
    for (int i = 0; i < NRANKS; i++) {
        if (counts[i] >= 2)
            score += (counts[i] * (counts[i] - 1) / 2) * 2;
    }
    return score;
}

ll calculate_runs_score(void) {
    ll score = 0;
    int i = 0;
    while (i < NRANKS - 2) {
        int run_length = 0;
        int j = i;
        while (j < NRANKS && counts[j] > 0) {
            run_length++;
            j++;
        }
        if (run_length >= 3) {
            ll run_product = 1;
            for (int k = i; k < i + run_length; k++)
                run_product *= counts[k];
            score += run_length * run_product;
            i += run_length;
        } else {
            i++;
        }
    }
    return score;
}

ll calculate_fifteens_score(void) {
    /* Generating function approach */
    ll gf[TARGET_SUM + 1];
    ll new_gf[TARGET_SUM + 1];
    memset(gf, 0, sizeof(gf));
    gf[0] = 1;

    for (int idx = 0; idx < NRANKS; idx++) {
        int value = rank_value(idx);
        int count = counts[idx];
        memset(new_gf, 0, sizeof(new_gf));

        for (int k = 0; k <= count; k++) {
            ll coeff = suit_combinations(k);
            int power = k * value;
            if (power > TARGET_SUM) break;
            for (int s = 0; s + power <= TARGET_SUM; s++) {
                new_gf[s + power] += gf[s] * coeff;
            }
        }
        memcpy(gf, new_gf, sizeof(gf));
    }

    return gf[TARGET_SUM] * 2;
}

ll calculate_hand_score(void) {
    ll total = 0;
    for (int i = 0; i < NRANKS; i++)
        total += (ll)counts[i] * rank_value(i);
    return total;
}

ll calculate_num_hands(void) {
    ll product = 1;
    for (int i = 0; i < NRANKS; i++)
        product *= suit_combinations(counts[i]);
    return product;
}

int main(void) {
    ll total_count = 0;

    /* Enumerate all multisets: counts[0]..counts[12] each 0..4 */
    /* 5^13 = 1,220,703,125 iterations - too many! */
    /* Optimize: skip empty hand, prune */
    /* Actually 5^13 ~ 1.2 billion, feasible in C in ~30s */

    /* Use recursive enumeration with pruning */
    int idx = 0;
    memset(counts, 0, sizeof(counts));

    /* Iterative enumeration */
    while (1) {
        /* Check if current multiset is valid (non-empty) */
        int total = 0;
        for (int i = 0; i < NRANKS; i++) total += counts[i];

        if (total > 0) {
            ll hand_score = calculate_hand_score();
            ll pairs = calculate_pairs_score();
            ll runs = calculate_runs_score();
            ll fifteens = calculate_fifteens_score();
            ll cribbage_score = pairs + runs + fifteens;

            if (hand_score == cribbage_score) {
                total_count += calculate_num_hands();
            }
        }

        /* Increment multiset */
        int carry = 1;
        for (int i = NRANKS - 1; i >= 0 && carry; i--) {
            counts[i]++;
            if (counts[i] > MAX_COUNT) {
                counts[i] = 0;
            } else {
                carry = 0;
            }
        }
        if (carry) break; /* All done */
    }

    printf("%lld\n", total_count);
    return 0;
}
