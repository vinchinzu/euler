#include <stdio.h>

typedef long long ll;

ll power_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    ll P = 900497239;
    int N = 10000;

    ll four_n = power_mod(4, N, P);
    ll two_n = power_mod(2, N, P);
    ll three_inv = power_mod(3, P - 2, P);

    ll s = (four_n + 2) % P * three_inv % P - two_n;
    s = ((s % P) + P) % P;

    printf("%lld\n", s);
    return 0;
}
