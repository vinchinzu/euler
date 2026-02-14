#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define LIMIT 50000000

static char is_prime_arr[8000];
static int primes[1100];
static int nprimes;

/* Sieve of Eratosthenes up to max_val */
void sieve(int max_val) {
    memset(is_prime_arr, 1, max_val + 1);
    is_prime_arr[0] = is_prime_arr[1] = 0;
    for (int i = 2; i * i <= max_val; i++)
        if (is_prime_arr[i])
            for (int j = i * i; j <= max_val; j += i)
                is_prime_arr[j] = 0;
    nprimes = 0;
    for (int i = 2; i <= max_val; i++)
        if (is_prime_arr[i])
            primes[nprimes++] = i;
}

/* Use a bit array to mark which numbers < LIMIT are expressible */
static unsigned char seen[LIMIT / 8 + 1];

void set_bit(int n) { seen[n >> 3] |= (1 << (n & 7)); }
int get_bit(int n) { return (seen[n >> 3] >> (n & 7)) & 1; }

int main(void) {
    int max_prime = (int)sqrt((double)LIMIT) + 1;
    sieve(max_prime);

    memset(seen, 0, sizeof(seen));

    for (int i = 0; i < nprimes; i++) {
        long long sq = (long long)primes[i] * primes[i];
        if (sq >= LIMIT) break;
        for (int j = 0; j < nprimes; j++) {
            long long cube = (long long)primes[j] * primes[j] * primes[j];
            if (sq + cube >= LIMIT) break;
            for (int k = 0; k < nprimes; k++) {
                long long fourth = (long long)primes[k] * primes[k] * primes[k] * primes[k];
                long long s = sq + cube + fourth;
                if (s >= LIMIT) break;
                set_bit((int)s);
            }
        }
    }

    int count = 0;
    for (int i = 0; i < LIMIT; i++)
        if (get_bit(i)) count++;

    printf("%d\n", count);
    return 0;
}
