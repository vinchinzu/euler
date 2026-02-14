/*
 * Project Euler 629: Scatterstone Nim
 *
 * Compute g(200) = sum_{k=2}^{200} f(200,k) mod 10^9+7,
 * where f(n,k) counts winning positions (non-zero Grundy value).
 *
 * k=2: compute nimbers by mex over splits into 2 parts
 * k=3: compute nimbers by mex over splits into 2 or 3 parts
 * k>=4: nimbers are 0,0,1,2,...,n-1
 *
 * For each nimber table, use partition DP with XOR tracking.
 */
#include <stdio.h>
#include <string.h>

#define NN 200
#define MOD 1000000007LL
#define XS 256

typedef long long ll;

static int nimbers2[NN+1];
static int nimbers3[NN+1];
static int nimbers4[NN+1];

static ll dp[NN+1][XS];
static ll tmp[NN+1][XS];

void compute_nimbers2(void) {
    nimbers2[0] = 0;
    nimbers2[1] = 0;
    for (int n = 2; n <= NN; n++) {
        int used[XS];
        memset(used, 0, sizeof(used));
        for (int i = 1; i < n; i++) {
            int v = nimbers2[i] ^ nimbers2[n - i];
            if (v < XS) used[v] = 1;
        }
        int mex = 0;
        while (mex < XS && used[mex]) mex++;
        nimbers2[n] = mex;
    }
}

void compute_nimbers3(void) {
    nimbers3[0] = 0;
    nimbers3[1] = 0;
    for (int n = 2; n <= NN; n++) {
        int used[XS];
        memset(used, 0, sizeof(used));
        for (int i = 1; i < n; i++) {
            int v = nimbers3[i] ^ nimbers3[n - i];
            if (v < XS) used[v] = 1;
        }
        for (int i = 1; i < n; i++) {
            for (int j = i; j <= (n - i); j++) {
                int k = n - i - j;
                if (k >= j) {
                    int v = nimbers3[i] ^ nimbers3[j] ^ nimbers3[k];
                    if (v < XS) used[v] = 1;
                }
            }
        }
        int mex = 0;
        while (mex < XS && used[mex]) mex++;
        nimbers3[n] = mex;
    }
}

void compute_nimbers4(void) {
    nimbers4[0] = 0;
    nimbers4[1] = 0;
    for (int n = 2; n <= NN; n++) {
        nimbers4[n] = n - 1;
    }
}

ll count_winning(int *nimbers) {
    memset(dp, 0, sizeof(dp));
    dp[0][0] = 1;

    for (int d = 1; d <= NN; d++) {
        int g = nimbers[d];
        memset(tmp, 0, sizeof(tmp));
        for (int a = d; a <= NN; a++) {
            for (int x = 0; x < XS; x++) {
                int px = x ^ g;
                tmp[a][x] = (dp[a - d][px] + tmp[a - d][px]) % MOD;
            }
        }
        for (int a = 0; a <= NN; a++) {
            for (int x = 0; x < XS; x++) {
                dp[a][x] = (dp[a][x] + tmp[a][x]) % MOD;
            }
        }
    }

    ll result = 0;
    for (int x = 1; x < XS; x++) {
        result = (result + dp[NN][x]) % MOD;
    }
    return result;
}

int main(void) {
    compute_nimbers2();
    compute_nimbers3();
    compute_nimbers4();

    ll f2 = count_winning(nimbers2);
    ll f3 = count_winning(nimbers3);
    ll f4 = count_winning(nimbers4);

    ll ans = (f2 + f3 + (ll)(NN - 3) % MOD * (f4 % MOD)) % MOD;
    printf("%lld\n", ans);
    return 0;
}
