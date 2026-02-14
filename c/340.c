/*
 * Project Euler Problem 340 - Crazy Function
 *
 * F(n) = n - c for n > b
 * F(n) = F(a + F(a + F(a + F(a + n)))) for n <= b
 *
 * Closed form: F(n) = n + 4(a-c) + floor((b-n)/a) * (4a - 3c)
 *
 * Sum = tr(b) + (b+1)*4*(a-c) + tr(b//a)*(b%a+1)*(4a-3c) + tr(b//a-1)*(a-b%a-1)*(4a-3c)
 * where tr(n) = n*(n+1)/2, and a=21^7, b=7^21, c=12^7.
 */
#include <stdio.h>

typedef long long ll;
typedef __int128 i128;

#define MOD 1000000000LL

ll mod(i128 x) {
    ll r = (ll)(x % MOD);
    if (r < 0) r += MOD;
    return r;
}

ll tr(ll n) {
    /* n*(n+1)/2 mod MOD */
    if (n <= 0) return 0;
    i128 nn = (i128)n % MOD;
    i128 nn1 = (i128)(n + 1) % MOD;
    if (n % 2 == 0)
        return (ll)((nn / 2 % MOD) * (nn1 % MOD) % MOD);
    else
        return (ll)((nn % MOD) * ((nn1 / 2) % MOD) % MOD);
}

int main(void) {
    /* a = 21^7, b = 7^21, c = 12^7 */
    ll A = 1;
    for (int i = 0; i < 7; i++) A *= 21;
    ll B = 1;
    for (int i = 0; i < 21; i++) B *= 7;
    ll C = 1;
    for (int i = 0; i < 7; i++) C *= 12;

    ll bma = B % A;
    ll bda = B / A;

    i128 ans = 0;
    ans += tr(B);
    ans += (i128)((B + 1) % MOD) * (4 % MOD) % MOD * (((A - C) % MOD + MOD) % MOD) % MOD;
    ans += (i128)tr(bda) * ((bma + 1) % MOD) % MOD * (((4 * A - 3 * C) % MOD + MOD) % MOD) % MOD;
    ans += (i128)tr(bda - 1) * (((A - bma - 1) % MOD + MOD) % MOD) % MOD * (((4 * A - 3 * C) % MOD + MOD) % MOD) % MOD;
    ans = ans % MOD;
    if (ans < 0) ans += MOD;

    printf("%lld\n", (ll)ans);
    return 0;
}
