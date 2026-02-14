/*
 * Project Euler Problem 509: Divisor Nim.
 * Compute number of winning positions in Divisor Nim.
 * Sprague-Grundy values for pile of size n depend only on v2(n).
 * With 3 piles, count triples where XOR of Grundy values is nonzero.
 */
#include <stdio.h>
#include <stdint.h>

typedef long long ll;

int main() {
    ll N = 123456787654321LL;
    ll M = 1234567890LL;

    ll counts[100];
    int num_counts = 0;

    int k = 0;
    while (1) {
        ll count = ((N >> k) - (N >> (k + 1))) % M;
        counts[num_counts++] = count;
        if (count == 0) break;
        k++;
    }

    ll ans = 0;
    for (int k1 = 0; k1 < num_counts; k1++) {
        for (int k2 = 0; k2 < num_counts; k2++) {
            for (int k3 = 0; k3 < num_counts; k3++) {
                if ((k1 ^ k2 ^ k3) != 0) {
                    ans = (ans + counts[k1] % M * (counts[k2] % M) % M * (counts[k3] % M)) % M;
                }
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
