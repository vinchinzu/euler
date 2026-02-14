/* Project Euler 185: Number Mind.
   Hill-climbing / random restart approach to find the 16-digit number
   matching the given guess constraints. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define DIGITS 16
#define NUM_GUESSES 22

static const char *guess_strs[NUM_GUESSES] = {
    "5616185650518293", "3847439647293047", "5855462940810587",
    "9742855507068353", "4296849643607543", "3174248439465858",
    "4513559094146117", "7890971548908067", "8157356344118483",
    "2615250744386899", "8690095851526254", "6375711915077050",
    "6913859173121360", "6442889055042768", "2321386104303845",
    "2326509471271448", "5251583379644322", "1748270476758276",
    "4895722652190306", "3041631117224635", "1841236454324589",
    "2659862637316867"
};

static const int targets[NUM_GUESSES] = {
    2, 1, 3, 3, 3, 1, 2, 3, 1, 2, 3, 1, 1, 2, 0, 2, 2, 3, 1, 3, 3, 2
};

static int guesses[NUM_GUESSES][DIGITS];
/* match_table[pos][digit][gi] = 1 if guess gi has digit at position pos */
static int match_table[DIGITS][10][NUM_GUESSES];

/* Simple LCG random number generator */
static unsigned long long rng_state;

static unsigned int rng_next(void) {
    rng_state = rng_state * 6364136223846793005ULL + 1442695040888963407ULL;
    return (unsigned int)(rng_state >> 32);
}

static int rng_int(int n) {
    return (int)(rng_next() % (unsigned int)n);
}

int main(void) {
    /* Parse guesses */
    for (int gi = 0; gi < NUM_GUESSES; gi++) {
        for (int p = 0; p < DIGITS; p++) {
            guesses[gi][p] = guess_strs[gi][p] - '0';
        }
    }

    /* Build match table */
    memset(match_table, 0, sizeof(match_table));
    for (int pos = 0; pos < DIGITS; pos++) {
        for (int gi = 0; gi < NUM_GUESSES; gi++) {
            match_table[pos][guesses[gi][pos]][gi] = 1;
        }
    }

    rng_state = 2024;
    int sequence[DIGITS];
    int matches[NUM_GUESSES];
    int contrib[NUM_GUESSES];

    int best_sequence[DIGITS];
    int best_cost = 999999;
    int current_cost;

    int max_attempts = 200;
    int max_iterations = 6000;

    for (int attempt = 0; attempt < max_attempts; attempt++) {
        /* Generate random sequence */
        for (int i = 0; i < DIGITS; i++) sequence[i] = rng_int(10);

        memset(matches, 0, sizeof(matches));
        for (int pos = 0; pos < DIGITS; pos++) {
            int d = sequence[pos];
            for (int gi = 0; gi < NUM_GUESSES; gi++) {
                matches[gi] += match_table[pos][d][gi];
            }
        }

        current_cost = 0;
        for (int gi = 0; gi < NUM_GUESSES; gi++) {
            contrib[gi] = abs(matches[gi] - targets[gi]);
            current_cost += contrib[gi];
        }

        if (current_cost < best_cost) {
            best_cost = current_cost;
            memcpy(best_sequence, sequence, sizeof(sequence));
        }

        for (int iter = 0; iter < max_iterations && current_cost > 0; iter++) {
            int improved = 0;

            /* Shuffle positions */
            int positions[DIGITS];
            for (int i = 0; i < DIGITS; i++) positions[i] = i;
            for (int i = DIGITS - 1; i > 0; i--) {
                int j = rng_int(i + 1);
                int tmp = positions[i]; positions[i] = positions[j]; positions[j] = tmp;
            }

            for (int pi = 0; pi < DIGITS && current_cost > 0; pi++) {
                int pos = positions[pi];
                int orig_digit = sequence[pos];

                int best_digit = orig_digit;
                int best_delta = 0;

                for (int td = 0; td < 10; td++) {
                    if (td == orig_digit) continue;
                    int delta_cost = 0;
                    for (int gi = 0; gi < NUM_GUESSES; gi++) {
                        int dm = match_table[pos][td][gi] - match_table[pos][orig_digit][gi];
                        if (dm == 0) continue;
                        int new_match = matches[gi] + dm;
                        int new_contrib = abs(new_match - targets[gi]);
                        delta_cost += new_contrib - contrib[gi];
                    }
                    if (delta_cost < best_delta) {
                        best_delta = delta_cost;
                        best_digit = td;
                    }
                }

                if (best_digit == orig_digit) continue;

                /* Apply */
                for (int gi = 0; gi < NUM_GUESSES; gi++) {
                    matches[gi] += match_table[pos][best_digit][gi] - match_table[pos][orig_digit][gi];
                    contrib[gi] = abs(matches[gi] - targets[gi]);
                }
                sequence[pos] = best_digit;
                current_cost += best_delta;
                improved = 1;

                if (current_cost < best_cost) {
                    best_cost = current_cost;
                    memcpy(best_sequence, sequence, sizeof(sequence));
                }
            }

            if (current_cost == 0) break;

            if (!improved) {
                /* Random perturbation */
                for (int r = 0; r < 2; r++) {
                    int pos = rng_int(DIGITS);
                    int nd = rng_int(10);
                    if (nd == sequence[pos]) continue;
                    int od = sequence[pos];
                    for (int gi = 0; gi < NUM_GUESSES; gi++) {
                        matches[gi] += match_table[pos][nd][gi] - match_table[pos][od][gi];
                        contrib[gi] = abs(matches[gi] - targets[gi]);
                    }
                    sequence[pos] = nd;
                }
                current_cost = 0;
                for (int gi = 0; gi < NUM_GUESSES; gi++) current_cost += contrib[gi];
                if (current_cost < best_cost) {
                    best_cost = current_cost;
                    memcpy(best_sequence, sequence, sizeof(sequence));
                }
            }
        }

        if (current_cost == 0) {
            memcpy(best_sequence, sequence, sizeof(sequence));
            break;
        }
    }

    for (int i = 0; i < DIGITS; i++) printf("%d", best_sequence[i]);
    printf("\n");
    return 0;
}
