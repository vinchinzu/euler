#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef unsigned long long ull;
typedef __uint128_t u128;

long long gcd_ll(long long a, long long b) {
    while (b) { long long t = b; b = a % b; a = t; }
    return a;
}

long long lcm_ll(long long a, long long b) {
    return a / gcd_ll(a, b) * b;
}

ull pow_mod(ull base, ull exp, ull mod) {
    ull result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (u128)result * base % mod;
        base = (u128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

/* Compute multiplicative order of base mod mod */
/* mod must be coprime to base */
long long mult_order(long long base, long long mod) {
    if (mod <= 1) return 1;
    if (gcd_ll(base, mod) != 1) return 0;

    /* Compute Euler totient of mod */
    long long phi = mod;
    long long temp = mod;
    for (long long p = 2; p * p <= temp; p++) {
        if (temp % p == 0) {
            phi -= phi / p;
            while (temp % p == 0) temp /= p;
        }
    }
    if (temp > 1) phi -= phi / temp;

    /* Order divides phi. Find smallest divisor that works */
    long long result = phi;

    /* Factor phi */
    temp = phi;
    for (long long p = 2; p * p <= temp; p++) {
        if (temp % p == 0) {
            while (temp % p == 0) temp /= p;
            while (result % p == 0 && pow_mod(base, result / p, mod) == 1)
                result /= p;
        }
    }
    if (temp > 1) {
        while (result % temp == 0 && pow_mod(base, result / temp, mod) == 1)
            result /= temp;
    }
    return result;
}

int cmp_ull(const void *a, const void *b) {
    ull va = *(const ull *)a;
    ull vb = *(const ull *)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

/* Longest non-decreasing subsequence using patience sorting */
int lis(int *arr, int n) {
    if (n == 0) return 0;
    int *tails = (int *)malloc(n * sizeof(int));
    int len = 0;

    for (int i = 0; i < n; i++) {
        /* bisect_right: find first position where tails[pos] > arr[i] */
        int lo = 0, hi = len;
        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (tails[mid] <= arr[i]) lo = mid + 1;
            else hi = mid;
        }
        tails[lo] = arr[i];
        if (lo == len) len++;
    }

    free(tails);
    return len;
}

long long S(long long n) {
    if (n <= 1) return 1;

    /* Find e_2 and n_2 */
    int e_2 = 0;
    long long n_2 = n;
    while (n_2 % 2 == 0) { e_2++; n_2 /= 2; }

    int e_3 = 0;
    long long n_3 = n;
    while (n_3 % 3 == 0) { e_3++; n_3 /= 3; }

    long long ord_2 = (n_2 > 1) ? mult_order(2, n_2) : 1;
    long long ord_3 = (n_3 > 1) ? mult_order(3, n_3) : 1;
    long long num_stations = (e_2 > e_3 ? e_2 : e_3) + lcm_ll(ord_2, ord_3);

    /* Generate stations encoded as 64-bit: (x << 32) | y */
    ull *stations = (ull *)malloc(num_stations * sizeof(ull));
    long long x = 1 % n, y = 1 % n;
    for (long long i = 0; i < num_stations; i++) {
        stations[i] = ((ull)x << 32) | (ull)y;
        x = (u128)x * 2 % n;
        y = (u128)y * 3 % n;
    }

    /* Remove duplicates by sorting */
    qsort(stations, num_stations, sizeof(ull), cmp_ull);
    long long unique = 0;
    for (long long i = 0; i < num_stations; i++) {
        if (i == 0 || stations[i] != stations[i-1]) {
            stations[unique++] = stations[i];
        }
    }

    /* Extract y-coordinates */
    int *y_coords = (int *)malloc(unique * sizeof(int));
    for (long long i = 0; i < unique; i++) {
        y_coords[i] = (int)(stations[i] & 0xFFFFFFFF);
    }

    int result = lis(y_coords, (int)unique);

    free(stations);
    free(y_coords);
    return result;
}

int main() {
    int N = 30, K = 5;
    long long ans = 0;

    for (int k = 1; k <= N; k++) {
        long long n = 1;
        for (int j = 0; j < K; j++) n *= k;
        long long s = S(n);
        ans += s;
    }

    printf("%lld\n", ans);
    return 0;
}
