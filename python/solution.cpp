
#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

typedef long long ll;
typedef __int128_t int128;

const ll N = 100000000000000LL;
const ll MOD = 1000000007LL;
const int LIMIT_PRIME = 22000000;

vector<int> primes;
vector<bool> is_prime_vec;

// Precomputed arrays for small m direct iteration
const int SMALL_M_LIMIT = 100000; // Increased limit
int lp[SMALL_M_LIMIT + 1];
int g_arr[SMALL_M_LIMIT + 1];
int max_p[SMALL_M_LIMIT + 1];
bool sq[SMALL_M_LIMIT + 1];

// DFS variables
ll total_sum = 0;
vector<int> primes_small;

inline ll S2_contribution(ll k) {
    if (k > N) return 0;
    ll m = sqrt(N / k); // integer sqrt
    if (m == 0) return 0;

    // val = m * (m + 1) * (2*m + 1) / 6
    // Use int128 to prevent overflow before modulo
    int128 v = (int128)m * (m + 1);
    v = v * (2 * m + 1);
    v /= 6;

    ll s2 = (ll)(v % MOD);
    return (k % MOD * s2) % MOD;
}

// Check primality for large numbers (rare fallback)
bool is_prime_large(ll p) {
    if (p <= LIMIT_PRIME) return is_prime_vec[p];
    if (p % 2 == 0) return false;
    for (ll d = 3; d * d <= p; d += 2) {
        if (p % d == 0) return false;
    }
    return true;
}

void dfs(int idx, ll current_m, int current_g, int q, ll limit_m) {
    // Check if current configuration yields valid k
    // p = current_g ^ q
    int p = current_g ^ q;

    if (p > q) {
        bool valid = false;
        if (p <= LIMIT_PRIME) {
            if (is_prime_vec[p]) valid = true;
        } else {
            if (is_prime_large(p)) valid = true;
        }

        if (valid) {
            int128 k = (int128)current_m * q * p;
            if (k <= N) {
                total_sum = (total_sum + S2_contribution((ll)k)) % MOD;
            }
        }
    }

    // Iterate primes
    for (int i = idx; i >= 0; --i) {
        int next_p = primes_small[i];
        ll next_m = current_m * next_p;

        if (next_m > limit_m) continue;

        dfs(i - 1, next_m, current_g ^ next_p, q, limit_m);
    }
}

int main() {
    // Sieve
    is_prime_vec.resize(LIMIT_PRIME + 1, true);
    is_prime_vec[0] = is_prime_vec[1] = false;
    for (int i = 2; i * i <= LIMIT_PRIME; ++i) {
        if (is_prime_vec[i]) {
            for (int j = i * i; j <= LIMIT_PRIME; j += i)
                is_prime_vec[j] = false;
        }
    }
    for (int i = 2; i <= LIMIT_PRIME; ++i) {
        if (is_prime_vec[i]) primes.push_back(i);
    }

    // Precompute small m arrays
    vector<int> pr;
    fill(sq, sq + SMALL_M_LIMIT + 1, true);

    for (int i = 2; i <= SMALL_M_LIMIT; ++i) {
        if (lp[i] == 0) {
            lp[i] = i;
            pr.push_back(i);
            g_arr[i] = i;
            max_p[i] = i;
        }
        for (int p : pr) {
            if (p > lp[i] || (ll)i * p > SMALL_M_LIMIT) break;
            lp[i * p] = p;
            max_p[i * p] = max_p[i];
            if (p == lp[i]) {
                sq[i * p] = false;
                g_arr[i * p] = g_arr[i] ^ p;
            } else {
                sq[i * p] = sq[i];
                g_arr[i * p] = g_arr[i] ^ p;
            }
        }
    }

    // k=1 case
    total_sum = (total_sum + S2_contribution(1)) % MOD;

    ll max_q = sqrt(N / 2);

    for (int q : primes) {
        if (q > max_q) break;

        ll q_sq = (ll)q * q;
        ll limit_m = N / q_sq;

        if (limit_m == 0) break;

        if (limit_m <= SMALL_M_LIMIT) {
            // Direct iteration
            // We iterate m from 2 to limit_m
            // m=1 yields p=q, invalid
            for (int m = 2; m <= limit_m; ++m) {
                if (sq[m]) {
                    // Check P(m) < q
                    // If q > SMALL_M_LIMIT, then q > m >= max_p[m], condition always true
                    // If q <= SMALL_M_LIMIT, we check max_p[m] < q
                    if (max_p[m] < q) {
                        int p = g_arr[m] ^ q;
                        if (p > q) {
                            bool valid = false;
                            if (p <= LIMIT_PRIME) {
                                if (is_prime_vec[p]) valid = true;
                            } else {
                                if (is_prime_large(p)) valid = true;
                            }

                            if (valid) {
                                int128 k = (int128)m * q * p;
                                total_sum = (total_sum + S2_contribution((ll)k)) % MOD;
                            }
                        }
                    }
                }
            }
        } else {
            // DFS
            if (2 * q_sq > N) continue;

            // Prepare primes < q
            // Since we iterate q in increasing order, we can maintain primes_small
            // But just rebuilding is safer and fast enough (copying small vector)
            // Optimization: maintain a persistent vector and append
            // But 'dfs' function needs random access.
            // Let's just create primes_small on the fly for small q.
            // Small q means limit_m is large -> q is small.
            // q <= sqrt(N/SMALL_M_LIMIT).
            // N = 10^14, SMALL = 10^5 => q <= sqrt(10^9) ~ 31622.
            // pi(31622) is small (~3400). Copying is trivial.

            primes_small.clear();
            for (int p : primes) {
                if (p >= q) break;
                primes_small.push_back(p);
            }

            dfs(primes_small.size() - 1, 1, 0, q, limit_m);
        }
    }

    cout << total_sum << endl;
    return 0;
}
    