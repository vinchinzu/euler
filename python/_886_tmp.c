
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define NN 34
#define MOD 83456729

int maxCounts[NN + 1];
int prods_arr[NN + 1];
int prod_val = 1;
int gcds_table[NN + 1][NN + 1];
int *cache_arr;
long long ans = 0;
int L;

int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

int primes_list[20];
int num_primes = 0;

void sieve(int n) {
    int is_p[n + 1];
    memset(is_p, 1, sizeof(is_p));
    is_p[0] = is_p[1] = 0;
    for (int i = 2; i * i <= n; i++)
        if (is_p[i])
            for (int j = i * i; j <= n; j += i)
                is_p[j] = 0;
    num_primes = 0;
    for (int i = 2; i <= n; i++)
        if (is_p[i])
            primes_list[num_primes++] = i;
}

long long numPerms(int counts[], int encodedCounts, int numRemaining, int prev) {
    if (numRemaining == 0) return 1;
    int key_prev = (prev % 2 == 0) ? prev / 2 : prev;
    int key = (key_prev - 1) / 2 * prod_val + encodedCounts;
    if (cache_arr[key] != -1) return cache_arr[key];
    long long result = 0;
    for (int num = 1; num <= NN; num++) {
        if (counts[num] > 0 && gcds_table[num][prev] == 1 && num % 2 != prev % 2) {
            counts[num]--;
            result += numPerms(counts, encodedCounts - prods_arr[num], numRemaining - 1, num);
            counts[num]++;
        }
    }
    result = ((result % MOD) + MOD) % MOD;
    cache_arr[key] = (int)result;
    return result;
}

void helper(int num, int counts[], int otherCounts[], int encodedCounts, int numUsed, int numOdds) {
    if (num > NN) {
        if (numUsed == L && numOdds == L / 2) {
            for (int middleNum = 1; middleNum <= NN; middleNum++) {
                if (otherCounts[middleNum] > 0 && middleNum % 2 == L % 2) {
                    otherCounts[middleNum]--;
                    int otherEncoded = prod_val - 1 - prods_arr[middleNum] - encodedCounts;
                    long long val1 = numPerms(counts, encodedCounts, L, middleNum);
                    long long val2 = numPerms(otherCounts, otherEncoded, L, middleNum);
                    ans = (ans + val1 * val2) % MOD;
                    otherCounts[middleNum]++;
                }
            }
        }
        return;
    }
    for (int count = 0; count <= maxCounts[num]; count++) {
        counts[num] += count;
        otherCounts[num] -= count;
        helper(num + 1, counts, otherCounts, encodedCounts + count * prods_arr[num],
               numUsed + count, numOdds + (num % 2 == 1 ? count : 0));
        counts[num] -= count;
        otherCounts[num] += count;
    }
}

int main() {
    L = (NN - 2) / 2;
    sieve(NN / 2);
    memset(maxCounts, 0, sizeof(maxCounts));
    for (int i = 2; i <= NN; i++) {
        int num = 1;
        for (int p = 0; p < num_primes; p++)
            if (i % primes_list[p] == 0)
                num *= primes_list[p];
        maxCounts[num]++;
    }
    prod_val = 1;
    for (int num = 1; num <= NN; num++) {
        prods_arr[num] = prod_val;
        prod_val *= maxCounts[num] + 1;
    }
    int cache_size = NN / 2 * prod_val;
    cache_arr = (int *)malloc(cache_size * sizeof(int));
    memset(cache_arr, -1, cache_size * sizeof(int));
    for (int i = 0; i <= NN; i++)
        for (int j = 0; j <= NN; j++)
            gcds_table[i][j] = gcd(i, j);
    int counts[NN + 1];
    int otherCounts[NN + 1];
    memset(counts, 0, sizeof(counts));
    memcpy(otherCounts, maxCounts, sizeof(maxCounts));
    helper(0, counts, otherCounts, 0, 0, 0);
    long long factorials[NN + 1];
    factorials[0] = 1;
    for (int i = 1; i <= NN; i++)
        factorials[i] = factorials[i - 1] * i % MOD;
    for (int num = 1; num <= NN; num++)
        ans = ans * factorials[maxCounts[num]] % MOD;
    printf("%lld\n", ans);
    free(cache_arr);
    return 0;
}
