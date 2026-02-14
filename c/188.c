/* Project Euler 188: The hyperexponentiation of a number. */
#include <stdio.h>

#define BASE 1777
#define HEIGHT 1855

static long long phi_cache[200000];
static int phi_cache_n[200000];
static int phi_cache_size = 0;

static long long euler_phi(long long n) {
    if (n == 1) return 1;

    /* Check cache */
    for (int i = 0; i < phi_cache_size; i++) {
        if (phi_cache_n[i] == n) return phi_cache[i];
    }

    long long result = n;
    long long temp = n;
    for (long long i = 2; i * i <= temp; i++) {
        if (temp % i == 0) {
            while (temp % i == 0) temp /= i;
            result -= result / i;
        }
    }
    if (temp > 1) result -= result / temp;

    if (phi_cache_size < 200000) {
        phi_cache_n[phi_cache_size] = (int)n;
        phi_cache[phi_cache_size] = result;
        phi_cache_size++;
    }

    return result;
}

static long long mod_pow(long long base, long long exp, long long mod) {
    if (mod == 1) return 0;
    long long result = 1 % mod;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) {
            result = (__int128)result * base % mod;
        }
        base = (__int128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

static long long gcd(long long a, long long b) {
    while (b) { long long t = b; b = a % b; a = t; }
    return a;
}

static long long tetration(long long a, int height, long long mod) {
    if (mod == 1) return 0;
    if (height == 1) return a % mod;

    long long phi_mod = euler_phi(mod);
    long long exponent = tetration(a, height - 1, phi_mod);
    if (exponent < phi_mod && gcd(a, mod) != 1) {
        exponent += phi_mod;
    }
    return mod_pow(a, exponent, mod);
}

int main(void) {
    long long result = tetration(BASE, HEIGHT, 100000000LL);
    printf("%lld\n", result);
    return 0;
}
