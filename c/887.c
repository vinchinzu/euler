#include <stdio.h>

typedef long long ll;

int main(void) {
    /* N = 7^10 */
    ll N = 1;
    for (int i = 0; i < 10; i++) N *= 7;

    int K = 7;
    ll ans = (N - 1) * N / 2;

    for (int d = 1; d <= K; d++) {
        ll prev_k = 1;
        int t = 1;
        while (prev_k < N) {
            ll k = 1LL << t;  /* 2^t */
            if (t > d) {
                k += t + 1 - d - (1LL << (t - d));
            }
            if (k > N) k = N;
            ans += (k - prev_k) * (ll)t;
            prev_k = k;
            t++;
        }
    }
    printf("%lld\n", ans);
    return 0;
}
