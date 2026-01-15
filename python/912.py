"""
Project Euler Problem 912: Binary Sequences without Three Consecutive Ones

Let s_n be the n-th positive integer that does not contain three consecutive ones
in its binary representation. For example, s_1 = 1 and s_7 = 8.

Define F(N) to be the sum of n^2 for all n <= N where s_n is odd.
You are given F(10) = 199.

Find F(10^16) giving your answer modulo 10^9+7.

Solution approach:
- Use digit DP to count valid numbers without "111" in binary
- Track statistics of odd numbers and their squared ranks
- Time complexity: O(log^2 N) for digit DP

Time complexity: O(log^2 N)
Space complexity: O(log N)
"""

MOD = 1_000_000_007


class Stats:
    """Statistics tracker for digit DP"""
    
    def __init__(self):
        self.total_count = 0
        self.odd_count = 0
        self.sum_r = 0
        self.sum_r2 = 0
    
    def apply_offset(self, offset, mod):
        """Apply rank offset to statistics"""
        new_sum_r = (self.sum_r + (self.odd_count * (offset % mod)) % mod) % mod
        
        offset_mod = offset % mod
        offset_sq = (offset_mod * offset_mod) % mod
        cross_term = (2 * self.sum_r * offset_mod) % mod
        quadratic_term = (self.odd_count * offset_sq) % mod
        
        new_sum_r2 = (self.sum_r2 + cross_term + quadratic_term) % mod
        
        result = Stats()
        result.total_count = self.total_count
        result.odd_count = self.odd_count
        result.sum_r = new_sum_r
        result.sum_r2 = new_sum_r2
        return result
    
    def combine(self, other, offset, mod):
        """Combine two stats with offset"""
        other_offset = other.apply_offset(offset, mod)
        
        result = Stats()
        result.total_count = (self.total_count + other_offset.total_count) % mod
        result.odd_count = (self.odd_count + other_offset.odd_count) % mod
        result.sum_r = (self.sum_r + other_offset.sum_r) % mod
        result.sum_r2 = (self.sum_r2 + other_offset.sum_r2) % mod
        return result
    
    def add_single_odd(self, rank, mod):
        """Add a single odd number with given rank"""
        rank_mod = rank % mod
        rank_sq = (rank_mod * rank_mod) % mod
        
        self.odd_count = (self.odd_count + 1) % mod
        self.sum_r = (self.sum_r + rank_mod) % mod
        self.sum_r2 = (self.sum_r2 + rank_sq) % mod
        self.total_count = (self.total_count + 1) % mod


def binary_digits(n):
    """Convert n to binary digit array"""
    if n == 0:
        return []
    digits = []
    temp = n
    while temp > 0:
        digits.append(temp % 2)
        temp //= 2
    return list(reversed(digits))


def f_n_digit_dp(n, mod):
    """Compute F(N) using digit DP"""
    if n < 1:
        return 0
    
    bin_n = binary_digits(n)
    length = len(bin_n)
    memo = {}
    
    def dp(pos, tight, consec, started):
        """Digit DP helper"""
        if pos == length:
            return Stats()
        
        key = (pos, tight, consec, 1 if started else 0)
        if key in memo:
            return memo[key]
        
        result = Stats()
        max_digit = bin_n[pos] if tight == 1 else 1
        current_offset = 0
        
        for digit in range(max_digit + 1):
            new_tight = 1 if (tight == 1 and digit == max_digit) else 0
            new_consec = 0 if digit == 0 else min(consec + 1, 2)
            new_started = started or (digit == 1)
            
            if new_consec < 3 or pos == length - 1:
                sub_stats = dp(pos + 1, new_tight, new_consec, new_started)
                
                if pos == length - 1:
                    if digit == 1 and new_started:  # Odd number
                        rank = current_offset + 1
                        result.add_single_odd(rank, mod)
                    if new_started:
                        result.total_count = (result.total_count + 1) % mod
                else:
                    if new_started:
                        adjusted_sub = sub_stats.apply_offset(current_offset, mod)
                        result.total_count = (result.total_count + adjusted_sub.total_count) % mod
                        result.odd_count = (result.odd_count + adjusted_sub.odd_count) % mod
                        result.sum_r = (result.sum_r + adjusted_sub.sum_r) % mod
                        result.sum_r2 = (result.sum_r2 + adjusted_sub.sum_r2) % mod
                
                branch_total = 1 if new_started else 0
                if pos < length - 1:
                    branch_total = sub_stats.total_count
                current_offset += branch_total
        
        memo[key] = result
        return result
    
    initial_stats = dp(0, 1, 0, False)
    return initial_stats.sum_r2


def main():
    """Main entry point"""
    import sys
    
    if len(sys.argv) > 1:
        N = int(sys.argv[1])
    else:
        N = 10**16
    
    result = f_n_digit_dp(N, MOD)
    print(result)


if __name__ == "__main__":
    main()
