/*
 * Project Euler Problem 946
 * Continued fraction of beta = (2*alpha+3)/(3*alpha+2)
 * where alpha has CF [2;1,1,2,1,1,1,2,...] with primes-many 1's between 2's
 * Find sum of first 10^8 coefficients of CF of beta.
 */
#include <stdio.h>
#include <stdbool.h>

typedef __int128 int128;

static long long floor_div(int128 a, int128 b) {
    if (b == 0) return 0;
    int128 res = a / b;
    if ((a ^ b) < 0 && (a % b != 0)) {
        res--;
    }
    return (long long)res;
}

static bool is_prime(int n) {
    if (n < 2) return false;
    if (n % 2 == 0) return n == 2;
    for (int i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return false;
    }
    return true;
}

int main(void) {
    int128 A = 2, B = 3, C = 3, D = 2;

    long long sum_beta = 0;
    long long count_beta = 0;
    long long target = 100000000;

    int state = 0;
    int current_prime = 2;
    int ones_left = 0;

    while (count_beta < target) {
        bool match = false;
        long long q = 0;

        int128 denom1 = C + D;
        int128 denom_inf = C;

        if (denom1 != 0 && denom_inf != 0) {
            long long q1 = floor_div(A + B, denom1);
            long long q_inf = floor_div(A, denom_inf);
            if (q1 == q_inf) {
                match = true;
                q = q1;
            }
        }

        if (match) {
            sum_beta += q;
            count_beta++;

            int128 next_A = C;
            int128 next_B = D;
            int128 next_C = A - (int128)q * C;
            int128 next_D = B - (int128)q * D;

            A = next_A; B = next_B; C = next_C; D = next_D;
        } else {
            int a = 0;
            if (state == 0) {
                a = 2;
                state = 1;
                ones_left = current_prime;
            } else if (state == 1) {
                if (ones_left > 0) {
                    a = 1;
                    ones_left--;
                } else {
                    state = 2;
                    a = 2;
                }
            } else if (state == 2) {
                int next_p = current_prime + 1;
                while (!is_prime(next_p)) next_p++;
                current_prime = next_p;

                state = 1;
                ones_left = current_prime;
                a = 1;
                ones_left--;
            }

            int128 next_A = A * a + B;
            int128 next_B = A;
            int128 next_C = C * a + D;
            int128 next_D = C;

            A = next_A; B = next_B; C = next_C; D = next_D;
        }
    }

    printf("%lld\n", sum_beta);
    return 0;
}
