/*
 * Project Euler 771 - Increasing Sequences
 *
 * Find the number of sequences of at least 5 strictly increasing integers x_i
 * such that |(x_i)^2 - x_{i-1}*x_{i+1}| <= 2 for all i.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

#define NN 1000000000000000000LL
#define M 1000000007LL

int* phi;
int64_t seq_count;
int64_t seq[200];
int64_t ans_total = 0;

void pre_phi(int64_t limit) {
    phi = (int*)malloc((limit + 1) * sizeof(int));
    for (int64_t i = 0; i <= limit; i++)
        phi[i] = (int)i;

    for (int64_t i = 2; i <= limit; i++) {
        if (phi[i] == i) {
            for (int64_t j = i; j <= limit; j += i)
                phi[j] -= phi[j] / (int)i;
        }
    }
}

int64_t tr(int64_t n) {
    int64_t a = n % M;
    int64_t b = (n + 1) % M;
    return ((__int128)a * b % M) * 500000004LL % M;
}

int64_t pow_int(int64_t base, int exponent) {
    int64_t result = 1;
    for (int i = 0; i < exponent; i++) {
        if (result > NN / base)
            return NN + 1;
        result *= base;
    }
    return result;
}

void process_seq_arr() {
    for (int start = 0; start < seq_count; start++) {
        for (int end = start + 5; end <= seq_count; end++) {
            if (seq[end - 1] <= NN)
                ans_total++;
        }
    }
}

void process_recursive_seq(int64_t x0, int64_t x1, int64_t a, int64_t b) {
    seq_count = 0;
    seq[seq_count++] = x0;
    seq[seq_count++] = x1;
    while (1) {
        int64_t prev2 = seq[seq_count - 2];
        int64_t prev1 = seq[seq_count - 1];
        double next_d = (double)a * prev2 + (double)b * prev1;
        if (next_d > (double)NN || next_d < 0)
            break;
        int64_t next = a * prev2 + b * prev1;
        if (next <= 0 || next > NN)
            break;
        seq[seq_count++] = next;
        if (seq_count >= 199)
            break;
    }
    process_seq_arr();
}

void process_seq_6(int64_t a, int64_t b, int64_t c, int64_t d, int64_t e, int64_t f) {
    seq_count = 0;
    seq[seq_count++] = a;
    seq[seq_count++] = b;
    seq[seq_count++] = c;
    seq[seq_count++] = d;
    seq[seq_count++] = e;
    seq[seq_count++] = f;
    process_seq_arr();
}

void process_seq_5(int64_t a, int64_t b, int64_t c, int64_t d, int64_t e) {
    seq_count = 0;
    seq[seq_count++] = a;
    seq[seq_count++] = b;
    seq[seq_count++] = c;
    seq[seq_count++] = d;
    seq[seq_count++] = e;
    process_seq_arr();
}

int main() {
    int64_t phi_limit = (int64_t)pow((double)NN, 0.25) + 1;
    pre_phi(phi_limit);

    ans_total = 0;
    int64_t ans = tr(NN - 4);

    process_seq_6(1, 2, 3, 4, 6, 9);
    process_seq_6(1, 2, 3, 5, 9, 16);
    process_seq_5(1, 2, 4, 7, 12);
    process_seq_5(1, 2, 4, 9, 20);
    process_seq_5(1, 2, 6, 17, 48);
    process_seq_5(1, 2, 6, 19, 60);

    process_recursive_seq(1, 2, 1, 1);
    process_recursive_seq(1, 2, 1, 2);
    process_recursive_seq(1, 2, -1, 3);
    process_recursive_seq(1, 3, 1, 2);
    process_recursive_seq(1, 3, -1, 4);

    for (int64_t x1 = 3; pow_int(x1 - 1, 4) <= NN; x1++) {
        process_recursive_seq(1, x1, -1, x1);
        process_recursive_seq(1, x1, 1, x1);
    }

    for (int64_t x1 = 2; 27 * x1 <= NN; x1 *= 3)
        ans_total++;

    for (int e = 4; pow_int(2, e) <= NN; e++) {
        for (int64_t x1 = 2; pow_int(x1, e) <= NN; x1++) {
            int64_t powe = pow_int(x1, e);
            ans_total += (int64_t)((NN / powe) % M) * phi[x1];
        }
    }

    ans = (ans + ans_total % M) % M;
    printf("%lld\n", ans);

    free(phi);
    return 0;
}
