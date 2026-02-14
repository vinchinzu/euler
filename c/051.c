#include <stdio.h>
#include <stdbool.h>
#include <string.h>

#define LIMIT 200000

static bool sieve[LIMIT];

void init_sieve(void) {
    for (int i = 0; i < LIMIT; i++) sieve[i] = true;
    sieve[0] = sieve[1] = false;
    for (int i = 2; (long long)i * i < LIMIT; i++) {
        if (sieve[i]) {
            for (int j = i * i; j < LIMIT; j += i) {
                sieve[j] = false;
            }
        }
    }
}

bool is_prime(int n) {
    if (n < 2) return false;
    if (n < LIMIT) return sieve[n];
    if (n % 2 == 0) return false;
    for (int i = 3; (long long)i * i <= n; i += 2) {
        if (n % i == 0) return false;
    }
    return true;
}

int main(void) {
    init_sieve();

    /* Collect primes */
    int primes[20000];
    int nprimes = 0;
    for (int i = 2; i < LIMIT; i++) {
        if (sieve[i]) primes[nprimes++] = i;
    }

    for (int pi = 0; pi < nprimes; pi++) {
        int p = primes[pi];
        char sp[16];
        sprintf(sp, "%d", p);
        int num_digits = (int)strlen(sp);
        if (num_digits < 2) continue;

        /* Count digit frequencies */
        int digit_count[10] = {0};
        int digit_positions[10][16];
        int digit_pos_count[10] = {0};

        for (int i = 0; i < num_digits; i++) {
            int d = sp[i] - '0';
            digit_positions[d][digit_pos_count[d]] = i;
            digit_pos_count[d]++;
            digit_count[d]++;
        }

        /* Try replacing digits that appear >= 3 times */
        for (int d = 0; d <= 9; d++) {
            if (digit_count[d] < 3) continue;

            int *positions = digit_positions[d];
            int npos = digit_pos_count[d];
            int prime_family_count = 0;
            int min_prime = 0;

            for (char replacement = '0'; replacement <= '9'; replacement++) {
                /* Skip leading zero */
                if (positions[0] == 0 && replacement == '0' && num_digits > 1) continue;

                char candidate[16];
                strcpy(candidate, sp);
                for (int k = 0; k < npos; k++) {
                    candidate[positions[k]] = replacement;
                }

                int candidate_num = 0;
                for (int k = 0; candidate[k]; k++) {
                    candidate_num = candidate_num * 10 + (candidate[k] - '0');
                }

                if (is_prime(candidate_num)) {
                    prime_family_count++;
                    if (min_prime == 0 || candidate_num < min_prime) {
                        min_prime = candidate_num;
                    }
                }
            }

            if (prime_family_count == 8) {
                printf("%d\n", min_prime);
                return 0;
            }
        }
    }

    return 0;
}
