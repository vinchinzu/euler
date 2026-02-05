"""Project Euler Problem 470: Super Ramvok."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define N 20

int ilog2(int n) {
    int r = 0;
    while (n > 1) { n >>= 1; r++; }
    return r;
}

int popcount(int x) {
    int c = 0;
    while (x) { c += x & 1; x >>= 1; }
    return c;
}

double R_func(int subset, double c) {
    if (c == 0)
        return 1 + ilog2(subset);

    int cnt = popcount(subset);
    double* vals = (double*)malloc(cnt * sizeof(double));
    int index = 0;
    for (int i = 0; i < N; i++)
        if ((subset & (1 << i)) > 0)
            vals[index++] = i + 1;

    double bestExpectedEarning = 0;
    for (int t = 1; ; t++) {
        double mean = 0;
        for (int i = 0; i < cnt; i++)
            mean += vals[i];
        mean /= cnt;

        double expectedEarning = mean - c * t;
        if (expectedEarning < bestExpectedEarning) {
            free(vals);
            return bestExpectedEarning;
        }
        bestExpectedEarning = expectedEarning;

        for (int i = 0; i < cnt; i++)
            if (vals[i] < mean)
                vals[i] = mean;
    }
}

double* tridiagonalSystem(double* A, double* B, double* C, double* D, int n) {
    double* c_prime = (double*)malloc(n * sizeof(double));
    double* d_prime = (double*)malloc(n * sizeof(double));
    double* x = (double*)malloc(n * sizeof(double));

    c_prime[0] = C[0] / B[0];
    d_prime[0] = D[0] / B[0];

    for (int i = 1; i < n; i++) {
        double denom = B[i] - A[i] * c_prime[i-1];
        c_prime[i] = (i < n-1) ? C[i] / denom : 0;
        d_prime[i] = (D[i] - A[i] * d_prime[i-1]) / denom;
    }

    x[n-1] = d_prime[n-1];
    for (int i = n-2; i >= 0; i--)
        x[i] = d_prime[i] - c_prime[i] * x[i+1];

    free(c_prime);
    free(d_prime);
    return x;
}

int main() {
    double ans = 0;
    double* R = (double*)malloc((1 << N) * sizeof(double));

    for (int c = 0; c <= N; c++) {
        for (int subset = 1; subset < (1 << N); subset++)
            R[subset] = R_func(subset, (double)c);

        for (int d = 4; d <= N; d++) {
            int size = d + 1;
            double* A = (double*)calloc(size, sizeof(double));
            double* B = (double*)calloc(size, sizeof(double));
            double* C = (double*)calloc(size, sizeof(double));
            double* D = (double*)calloc(size, sizeof(double));

            for (int i = 1; i <= d; i++)
                A[i] = -(double)(d - i + 1) / d;
            for (int i = 0; i <= d; i++)
                B[i] = 1;
            for (int i = 1; i < d; i++)
                C[i] = -(double)(i + 1) / d;
            for (int subset = 1; subset < (1 << d); subset++)
                D[popcount(subset)] += R[subset];

            double* x = tridiagonalSystem(A, B, C, D, size);
            ans += x[d];

            free(A); free(B); free(C); free(D); free(x);
        }
    }

    free(R);
    printf("%ld\n", (long)round(ans));
    return 0;
}
'''

    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name

    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
    result = subprocess.check_output([exe]).decode().strip()

    os.unlink(c_file)
    os.unlink(exe)

    return int(result)

if __name__ == "__main__":
    print(solve())
