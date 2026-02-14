#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>

bool is_prime(int n) {
    if (n < 2) return false;
    if (n == 2) return true;
    if (n % 2 == 0) return false;
    for (int i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return false;
    }
    return true;
}

void sorted_digits(int n, char *out) {
    char buf[16];
    sprintf(buf, "%d", n);
    int len = strlen(buf);
    /* Simple bubble sort */
    for (int i = 0; i < len - 1; i++) {
        for (int j = i + 1; j < len; j++) {
            if (buf[j] < buf[i]) {
                char tmp = buf[i]; buf[i] = buf[j]; buf[j] = tmp;
            }
        }
    }
    strcpy(out, buf);
}

int main(void) {
    /* Collect 4-digit primes */
    int primes[2000];
    int nprimes = 0;
    for (int i = 1000; i < 10000; i++) {
        if (is_prime(i)) {
            primes[nprimes++] = i;
        }
    }

    /* Group by sorted digits and find arithmetic sequences */
    for (int i = 0; i < nprimes; i++) {
        for (int j = i + 1; j < nprimes; j++) {
            char si[16], sj[16];
            sorted_digits(primes[i], si);
            sorted_digits(primes[j], sj);
            if (strcmp(si, sj) != 0) continue;

            int diff = primes[j] - primes[i];
            int c = primes[j] + diff;
            if (c >= 10000) continue;

            if (!is_prime(c)) continue;

            char sc[16];
            sorted_digits(c, sc);
            if (strcmp(si, sc) != 0) continue;

            /* Skip the known example */
            if (primes[i] == 1487 && primes[j] == 4817 && c == 8147) continue;

            printf("%d%d%d\n", primes[i], primes[j], c);
            return 0;
        }
    }

    return 0;
}
