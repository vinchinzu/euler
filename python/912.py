MOD = 10**9 + 7

def solve():
    N = 10**16

    # For a given state s (trailing 1s: 0,1,2) and r remaining bits to fill,
    # compute (count, odd_count, sum_odd_pos, sum_odd_pos_sq) mod MOD
    # where positions are 1-based among all valid completions

    memo = {}

    def dp(s, r):
        """Returns (count, odd_count, sum_odd_pos, sum_odd_pos_sq) for state s with r bits remaining."""
        if (s, r) in memo:
            return memo[(s, r)]

        if r == 0:
            # All bits filled. Odd if s >= 1 (last bit was 1)
            is_odd = 1 if s >= 1 else 0
            result = (1, is_odd, is_odd, is_odd)  # count=1, if odd: pos=1, pos^2=1
            memo[(s, r)] = result
            return result

        # Branch on next bit = 0
        cnt_L, odd_L, sop_L, sopq_L = dp(0, r - 1)

        # Branch on next bit = 1
        if s < 2:
            cnt_R, odd_R, sop_R, sopq_R = dp(s + 1, r - 1)
        else:
            cnt_R, odd_R, sop_R, sopq_R = 0, 0, 0, 0

        cnt = (cnt_L + cnt_R) % MOD
        odd = (odd_L + odd_R) % MOD
        sop = (sop_L + sop_R + cnt_L % MOD * odd_R) % MOD
        sopq = (sopq_L + sopq_R + 2 * (cnt_L % MOD) * sop_R + (cnt_L % MOD) * (cnt_L % MOD) % MOD * odd_R) % MOD

        result = (cnt, odd, sop, sopq)
        memo[(s, r)] = result
        return result

    # We need F(N) = sum of n^2 for n <= N where s_n is odd
    # Process bit-lengths from 1 upward
    # For bit-length b, valid numbers start with 1, then b-1 more bits
    # State after MSB=1 is s=1, remaining bits = b-1

    # First, find how many total valid numbers exist for each bit-length
    # and accumulate until we reach N

    remaining_N = N  # how many more numbers we need to process
    R_offset = 0     # global rank offset

    ans = 0

    # For exact counts, we need the actual counts (not mod)
    # But counts can be huge (tribonacci growth ~ 1.84^b)
    # For b ~ 60, count ~ 1.84^60 ~ 10^16, which fits in Python int

    # Let's compute exact counts separately
    # c[s][r] = exact count (no mod)
    exact_count = {}

    def exact_dp(s, r):
        if (s, r) in exact_count:
            return exact_count[(s, r)]
        if r == 0:
            exact_count[(s, r)] = 1
            return 1
        result = exact_dp(0, r - 1)
        if s < 2:
            result += exact_dp(s + 1, r - 1)
        exact_count[(s, r)] = result
        return result

    # Precompute exact counts for bit-lengths 1..80
    for b in range(1, 80):
        exact_dp(1, b - 1)

    for b in range(1, 80):
        # Count of valid b-bit numbers
        total_b = exact_dp(1, b - 1)

        if total_b <= remaining_N:
            # Process all valid b-bit numbers
            cnt, odd_cnt_mod, sop_mod, sopq_mod = dp(1, b - 1)
            # But we need to use exact odd_count for the middle computation
            # Actually, since we're working mod MOD, we need to be careful

            # Global contribution: sum of (R_offset + pos)^2 for odd positions
            # = sum(R_offset^2 + 2*R_offset*pos + pos^2) for odd
            # = odd_count * R_offset^2 + 2*R_offset*sum_pos + sum_pos^2

            R_off_mod = R_offset % MOD
            contribution = (odd_cnt_mod * R_off_mod % MOD * R_off_mod +
                          2 * R_off_mod % MOD * sop_mod + sopq_mod) % MOD
            ans = (ans + contribution) % MOD

            R_offset += total_b
            remaining_N -= total_b
        else:
            # Process only 'remaining_N' valid b-bit numbers (partial)
            # Need to enumerate using digit DP from MSB
            # The b-bit numbers start with 1, then b-1 more bits
            # We process the remaining bits one by one

            s = 1  # state after MSB=1
            local_offset = 0  # position within b-bit numbers

            for bit_pos in range(b - 2, -1, -1):  # remaining b-1 bits, from high to low
                # Try bit = 0 first (these come first in ordering)
                cnt_0 = exact_dp(0, bit_pos)

                if remaining_N <= cnt_0:
                    # All remaining numbers are in the 0-branch
                    s = 0
                    continue
                else:
                    # All of the 0-branch is included, plus some from 1-branch
                    # Add contribution from 0-branch
                    if cnt_0 > 0:
                        cnt_0_mod, odd_0, sop_0, sopq_0 = dp(0, bit_pos)

                        R_off_mod = (R_offset + local_offset) % MOD
                        contribution = (odd_0 * R_off_mod % MOD * R_off_mod +
                                      2 * R_off_mod % MOD * sop_0 + sopq_0) % MOD
                        ans = (ans + contribution) % MOD

                    remaining_N -= cnt_0
                    local_offset += cnt_0

                    # Go into 1-branch
                    if s >= 2:
                        break  # invalid, can't place 1
                    s = s + 1

            if remaining_N > 0:
                # We've reached the last bit. There's exactly 1 number at this position.
                # It's the number we've been building. Is it odd?
                is_odd = 1 if s >= 1 else 0
                if is_odd:
                    rank = R_offset + local_offset + 1
                    ans = (ans + rank % MOD * (rank % MOD) % MOD) % MOD
                remaining_N -= 1

            break

    print(ans % MOD)

if __name__ == "__main__":
    solve()
