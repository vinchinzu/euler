/* Project Euler Problem 130: Composites with prime repunit property.
 *
 * Sum the first 25 composite n (coprime to 10) where A(n) | (n-1).
 * A(n) = least k such that repunit R(k) divisible by n.
 */
#include <stdio.h>
#include <stdbool.h>
#include <math.h>

#define TARGET_COUNT 25

static int repunit_period(int n) {
    int remainder = 1 % n;
    int length = 1;
    while (remainder != 0) {
        remainder = (remainder * 10 + 1) % n;
        length++;
    }
    return length;
}

static bool is_prime(int n) {
    if (n < 2) return false;
    if (n < 4) return true;
    if (n % 2 == 0 || n % 3 == 0) return false;
    for (int i = 5; (long long)i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) return false;
    }
    return true;
}

static bool is_composite_coprime10(int n) {
    if (n < 4) return false;
    if (n % 2 == 0 || n % 5 == 0) return false;
    return !is_prime(n);
}

int main(void) {
    int found = 0;
    long long total = 0;

    for (int n = 2; found < TARGET_COUNT; n++) {
        if (n % 2 == 0 || n % 5 == 0) continue;
        if (!is_composite_coprime10(n)) continue;

        int period = repunit_period(n);
        if ((n - 1) % period == 0) {
            total += n;
            found++;
        }
    }

    printf("%lld\n", total);
    return 0;
}
