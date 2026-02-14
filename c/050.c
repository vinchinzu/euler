#include <stdio.h>
#include <stdbool.h>

#define LIMIT 1000000

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

int main(void) {
    init_sieve();

    /* Collect primes */
    int primes[80000];
    int nprimes = 0;
    for (int i = 2; i < LIMIT; i++) {
        if (sieve[i]) {
            primes[nprimes++] = i;
        }
    }

    /* Prefix sums */
    long long prefix[80001];
    prefix[0] = 0;
    for (int i = 0; i < nprimes; i++) {
        prefix[i + 1] = prefix[i] + primes[i];
    }

    int max_length = 0;
    int best_prime = 0;

    for (int start = 0; start < nprimes; start++) {
        if (start + max_length >= nprimes) break;
        for (int finish = start + max_length; finish < nprimes; finish++) {
            long long sum = prefix[finish + 1] - prefix[start];
            if (sum >= LIMIT) break;
            if (sieve[(int)sum]) {
                int current_length = finish - start + 1;
                if (current_length > max_length) {
                    max_length = current_length;
                    best_prime = (int)sum;
                }
            }
        }
    }

    printf("%d\n", best_prime);
    return 0;
}
