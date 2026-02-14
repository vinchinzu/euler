/*
 * Project Euler 035 - Circular Primes
 * How many circular primes are there below one million?
 */
#include <stdio.h>
#include <stdbool.h>

#define LIMIT 1000000

bool sieve[LIMIT];

void build_sieve(void) {
    for (int i = 0; i < LIMIT; i++) sieve[i] = true;
    sieve[0] = sieve[1] = false;
    for (long long i = 2; i < LIMIT; i++) {
        if (sieve[i]) {
            for (long long j = i * i; j < LIMIT; j += i) {
                sieve[j] = false;
            }
        }
    }
}

bool has_disqualifying_digits(int num) {
    if (num < 10) return false;
    int temp = num;
    while (temp > 0) {
        int d = temp % 10;
        if (d == 0 || d == 2 || d == 4 || d == 5 || d == 6 || d == 8) {
            return true;
        }
        temp /= 10;
    }
    return false;
}

/* Count digits in num */
int num_digits(int num) {
    if (num == 0) return 1;
    int count = 0;
    while (num > 0) {
        count++;
        num /= 10;
    }
    return count;
}

/* Compute 10^n */
int power10(int n) {
    int result = 1;
    for (int i = 0; i < n; i++) result *= 10;
    return result;
}

bool is_circular_prime(int num) {
    if (has_disqualifying_digits(num)) return false;
    if (num < 10) return true; /* single digit primes already checked via sieve */

    int len = num_digits(num);
    int p10 = power10(len - 1);
    int rotated = num;

    for (int i = 0; i < len; i++) {
        /* Rotate: move first digit to end */
        int first = rotated / p10;
        rotated = (rotated % p10) * 10 + first;
        if (rotated >= LIMIT || !sieve[rotated]) return false;
    }
    return true;
}

int main(void) {
    build_sieve();

    int count = 0;
    for (int i = 2; i < LIMIT; i++) {
        if (sieve[i] && is_circular_prime(i)) {
            count++;
        }
    }

    printf("%d\n", count);
    return 0;
}
