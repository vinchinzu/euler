# Project Euler Problem 861
#
# PROBLEM DESCRIPTION:
# A unitary divisor of a positive integer n is a divisor d of n such that
# gcd(d, n/d) = 1.
#
# A bi-unitary divisor of n is a divisor d for which 1 is the only unitary
# divisor of d that is also a unitary divisor of n/d.
#
# For example, 2 is a bi-unitary divisor of 8, because the unitary divisors
# of 2 are {1,2}, and the unitary divisors of 8/2 are {1,4}, with 1 being
# the only unitary divisor in common.
#
# The bi-unitary divisors of 240 are
# {1,2,3,5,6,8,10,15,16,24,30,40,48,80,120,240}.
#
# Let P(n) be the product of all bi-unitary divisors of n. Define Q_k(N) as
# the number of positive integers 1 < n <= N such that P(n) = n^k.
# For example, Q_2(10^2) = 51 and Q_6(10^6) = 6189.
#
# Find sum_{k=2}^{10} Q_k(10^12).

import subprocess
import os

CPP_SOURCE = r"""
#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>
#include <map>
#include <set>
using namespace std;
typedef long long ll;

ll N_LIMIT = 1000000000000LL; // 10^12
vector<ll> primes;
vector<ll> S_small, S_large;
ll isqrt_val;

void sieve(int limit) {
    vector<bool> is_prime_small(limit + 1, true);
    is_prime_small[0] = is_prime_small[1] = false;
    for (int p = 2; p * p <= limit; ++p) {
        if (is_prime_small[p]) {
            for (int i = p * p; i <= limit; i += p)
                is_prime_small[i] = false;
        }
    }
    for (int p = 2; p <= limit; ++p) {
        if (is_prime_small[p]) primes.push_back(p);
    }
}

void compute_pi(ll N) {
    isqrt_val = sqrt((double)N);
    // Ensure isqrt_val is exact
    while ((isqrt_val + 1) * (isqrt_val + 1) <= N) isqrt_val++;
    while (isqrt_val > 0 && isqrt_val * isqrt_val > N) isqrt_val--;
    
    S_small.assign(isqrt_val + 1, 0);
    S_large.assign(isqrt_val + 1, 0);
    for (ll v = 1; v <= isqrt_val; ++v) S_small[v] = v - 1;
    for (ll k = 1; k <= isqrt_val; ++k) S_large[k] = (N / k) - 1;
    
    for (ll p : primes) {
        if (p > isqrt_val) break;
        ll p2 = p * p;
        if (p2 > N) break;
        ll sp_1 = S_small[p - 1];
        ll k_limit = min(N / p2, (ll)isqrt_val);
        for (ll k = 1; k <= k_limit; ++k) {
            ll v = N / k;
            ll target = v / p;
            ll s_target = (target <= isqrt_val) ? S_small[target] : S_large[k * p];
            S_large[k] -= (s_target - sp_1);
        }
        for (ll v = isqrt_val; v >= p2; --v) {
            S_small[v] -= (S_small[v / p] - sp_1);
        }
    }
}

ll get_pi(ll x) {
    if (x <= 0) return 0;
    if (x <= isqrt_val) return S_small[x];
    return S_large[N_LIMIT / x];
}

// Generate factor partitions: ways to write target as product of `count` factors
void get_factor_partitions(int target, int count, int min_val,
                           vector<int>& current, vector<vector<int>>& results) {
    if (count == 1) {
        if (target >= min_val) {
            vector<int> res = current;
            res.push_back(target);
            results.push_back(res);
        }
        return;
    }
    for (int i = min_val; i <= target; ++i) {
        if (target % i == 0) {
            current.push_back(i);
            get_factor_partitions(target / i, count - 1, i, current, results);
            current.pop_back();
        }
    }
}

// Generate all valid exponent signatures for P(n) = n^k
// D_bu(n) = 2k, where D_bu(n) = product of f(a_i) and f(a) = a if even, a+1 if odd
// f(a) = 2*ceil(a/2), so D_bu = 2^r * product(y_i) where y_i = ceil(a_i/2)
// For r prime factors: 2^r * product(y_i) = 2k => product(y_i) = k/2^(r-1)
void generate_signatures(int k, set<vector<int>>& sigs) {
    int r = 1;
    while (true) {
        ll power_of_2 = 1LL << (r - 1);
        if (power_of_2 > k) break;
        if (k % power_of_2 == 0) {
            int target = k / power_of_2;  // product of y_i values
            vector<vector<int>> partitions;
            vector<int> current;
            get_factor_partitions(target, r, 1, current, partitions);
            
            for (auto& p : partitions) {
                // For each y_i, a_i can be 2*y_i - 1 (odd) or 2*y_i (even)
                int n_choices = 1 << r;
                for (int mask = 0; mask < n_choices; ++mask) {
                    vector<int> a;
                    for (int i = 0; i < r; ++i) {
                        if ((mask >> i) & 1) a.push_back(2 * p[i]);
                        else a.push_back(2 * p[i] - 1);
                    }
                    sort(a.begin(), a.end());
                    sigs.insert(a);
                }
            }
        }
        r++;
    }
}

// Integer nth root: largest x such that x^k <= n
ll integer_root(ll n, int k) {
    if (k == 1) return n;
    if (n <= 1) return n;
    ll lo = 1, hi = (ll)pow((double)n, 1.0/k) + 2;
    ll ans = 1;
    while (lo <= hi) {
        ll mid = lo + (hi - lo) / 2;
        ll p = 1;
        bool over = false;
        for (int i = 0; i < k; ++i) {
            if (p > n / mid) { over = true; break; }
            p *= mid;
        }
        if (!over && p <= n) {
            ans = mid;
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    return ans;
}

// Count numbers with given signature using backtracking
// Groups are sorted by exponent descending for efficiency
ll backtrack_inner(int group_idx, int remain, ll current_prod,
                   vector<ll>& used_primes, const vector<pair<int,int>>& groups,
                   int min_p_idx);

ll backtrack(int group_idx, ll current_prod, vector<ll>& used_primes,
             const vector<pair<int,int>>& groups) {
    if (group_idx == (int)groups.size()) return 1;
    int count = groups[group_idx].second;
    return backtrack_inner(group_idx, count, current_prod, used_primes, groups, 0);
}

ll backtrack_inner(int group_idx, int remain, ll current_prod,
                   vector<ll>& used_primes, const vector<pair<int,int>>& groups,
                   int min_p_idx) {
    int exp = groups[group_idx].first;
    
    if (remain == 0) {
        return backtrack(group_idx + 1, current_prod, used_primes, groups);
    }
    
    ll total = 0;
    
    // Optimization: if this is the last prime to pick, use pi(x)
    if (group_idx == (int)groups.size() - 1 && remain == 1) {
        ll rem = N_LIMIT / current_prod;
        ll limit_p = integer_root(rem, exp);
        
        if (limit_p < 2) return 0;
        
        ll lower_bound_val;
        if (min_p_idx < (int)primes.size()) {
            lower_bound_val = primes[min_p_idx];
        } else {
            lower_bound_val = primes.empty() ? 2 : primes.back() + 1;
        }
        
        if (limit_p < lower_bound_val) return 0;
        
        ll valid_count = get_pi(limit_p) - get_pi(lower_bound_val - 1);
        
        // Exclude primes used by other groups
        for (ll up : used_primes) {
            if (up >= lower_bound_val && up <= limit_p) {
                valid_count--;
            }
        }
        return valid_count;
    }
    
    // Iterate through primes
    for (int i = min_p_idx; i < (int)primes.size(); ++i) {
        ll p = primes[i];
        
        // Check if prime already used by another group
        bool collision = false;
        for (ll up : used_primes) {
            if (up == p) { collision = true; break; }
        }
        if (collision) continue;
        
        // Compute p^exp
        ll p_pow = 1;
        bool over = false;
        for (int k = 0; k < exp; ++k) {
            if (p_pow > N_LIMIT / p) { over = true; break; }
            p_pow *= p;
        }
        if (over) break;
        
        // Check product bound
        if (current_prod > N_LIMIT / p_pow) break;
        ll next_prod = current_prod * p_pow;
        if (next_prod > N_LIMIT) break;
        
        used_primes.push_back(p);
        total += backtrack_inner(group_idx, remain - 1, next_prod, used_primes,
                                 groups, i + 1);
        used_primes.pop_back();
    }
    return total;
}

int main() {
    int sqrtN = 1000001;
    sieve(sqrtN);
    compute_pi(N_LIMIT);
    
    ll total_sum = 0;
    
    for (int k = 2; k <= 10; ++k) {
        set<vector<int>> sigs;
        generate_signatures(k, sigs);
        
        ll q_k = 0;
        for (const auto& sig : sigs) {
            // Group exponents
            map<int, int> counts;
            for (int x : sig) counts[x]++;
            
            // Sort by exponent descending (larger exponents are more constraining)
            vector<pair<int,int>> groups;
            for (auto& p : counts) groups.push_back(p);
            sort(groups.rbegin(), groups.rend());
            
            vector<ll> used;
            ll count = backtrack(0, 1, used, groups);
            q_k += count;
        }
        total_sum += q_k;
    }
    
    cout << total_sum << endl;
    return 0;
}
"""


def solve() -> int:
    """Solve Project Euler Problem 861."""
    script_dir = os.path.dirname(os.path.abspath(__file__))
    cpp_filename = os.path.join(script_dir, "solution_861.cpp")
    bin_filename = os.path.join(script_dir, "solution_861_bin")

    with open(cpp_filename, "w") as f:
        f.write(CPP_SOURCE)

    try:
        compile_cmd = ["g++", "-O3", "-march=native", cpp_filename, "-o", bin_filename]
        subprocess.check_call(compile_cmd)
        result = subprocess.check_output([bin_filename]).decode().strip()
        return int(result)
    finally:
        if os.path.exists(cpp_filename):
            os.remove(cpp_filename)
        if os.path.exists(bin_filename):
            os.remove(bin_filename)


if __name__ == "__main__":
    print(solve())
