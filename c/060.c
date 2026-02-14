#include <stdio.h>
#include <stdbool.h>
#include <math.h>

bool is_prime(long long n) {
    if (n < 2) return false;
    if (n == 2 || n == 3) return true;
    if (n % 2 == 0 || n % 3 == 0) return false;
    for (long long i = 5; i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) return false;
    }
    return true;
}

long long concat_nums(int a, int b) {
    long long result = a;
    int tmp = b;
    if (tmp == 0) return result * 10;
    while (tmp > 0) {
        result *= 10;
        tmp /= 10;
    }
    return result + b;
}

bool is_concat_prime(int a, int b) {
    return is_prime(concat_nums(a, b)) && is_prime(concat_nums(b, a));
}

int main(void) {
    /* Generate primes up to 10000 */
    int primes[1300];
    int nprimes = 0;
    for (int i = 2; i < 10000; i++) {
        if (is_prime(i)) {
            primes[nprimes++] = i;
        }
    }

    /* Search for 5 primes where all pairs concatenate to primes */
    for (int a = 0; a < nprimes; a++) {
        for (int b = a + 1; b < nprimes; b++) {
            if (!is_concat_prime(primes[a], primes[b])) continue;
            for (int c = b + 1; c < nprimes; c++) {
                if (!is_concat_prime(primes[a], primes[c])) continue;
                if (!is_concat_prime(primes[b], primes[c])) continue;
                for (int d = c + 1; d < nprimes; d++) {
                    if (!is_concat_prime(primes[a], primes[d])) continue;
                    if (!is_concat_prime(primes[b], primes[d])) continue;
                    if (!is_concat_prime(primes[c], primes[d])) continue;
                    for (int e = d + 1; e < nprimes; e++) {
                        if (!is_concat_prime(primes[a], primes[e])) continue;
                        if (!is_concat_prime(primes[b], primes[e])) continue;
                        if (!is_concat_prime(primes[c], primes[e])) continue;
                        if (!is_concat_prime(primes[d], primes[e])) continue;

                        int sum = primes[a] + primes[b] + primes[c] + primes[d] + primes[e];
                        printf("%d\n", sum);
                        return 0;
                    }
                }
            }
        }
    }

    return 0;
}
