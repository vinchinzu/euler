# Project Euler Problem 948
#
# PROBLEM DESCRIPTION:
# <p>Left and Right play a game with a word consisting of L's and R's, alternating turns. On Left's turn, Left can remove any positive number of letters, but not all the letters, from the left side of the word. Right does the same on Right's turn except that Right removes letters from the right side. The game continues until only one letter remains: if it is an 'L' then Left wins; if it is an 'R' then Right wins.</p>
# 
# <p>Let $F(n)$ be the number of words of length $n$ where the player moving first, whether it's Left or Right, will win the game if both play optimally.</p>
# 
# <p>You are given $F(3)=4$ and $F(8)=181$.</p>
# 
# <p>Find $F(60)$.</p>
#

import sys

def solve_dp(n):
    if n <= 0: return 0
    if n == 1: return 0 # F(1) is 0 because type N requires length > 1 (actually check logic below)

    # State: dictionary mapping (k, gp, gs) -> count
    # k: integer (number of proper suffixes of Type L)
    # gp: boolean (Good Prefix property: has proper prefix in {R, P})
    # gs: boolean (Good Suffix property: has proper suffix in {L, P})

    # Base cases for length 1
    # "L": k=0, gp=False, gs=True
    # "R": k=0, gp=False, gs=False

    # Note: gs is derived from "L" or k>0, but for length 1, "L" has gs=True inherently (it is in {L,P})
    # Wait, my DP state derivation assumed gs is stored.
    # "L" -> gs=True.
    # "R" -> gs=False.

    counts = {
        (0, False, True): 1,   # "L"
        (0, False, False): 1   # "R"
    }

    for length in range(1, n):
        new_counts = {}
        for state, count in counts.items():
            k, gp, gs = state

            # Transition for adding "L"
            # k_new = k + 1
            # gp_new = gp or (not gs)
            # gs_new = True (since "L" is in {L, P})

            nk_l = k + 1
            ngp_l = gp or (not gs)
            ngs_l = True
            state_l = (nk_l, ngp_l, ngs_l)
            new_counts[state_l] = new_counts.get(state_l, 0) + count

            # Transition for adding "R"
            # k_new = max(0, k - 1)
            # gp_new = gp or (not gs)
            # gs_new = (k > 0)

            nk_r = max(0, k - 1)
            ngp_r = gp or (not gs)
            ngs_r = (k > 0)
            state_r = (nk_r, ngp_r, ngs_r)
            new_counts[state_r] = new_counts.get(state_r, 0) + count

        counts = new_counts

    # Count Type N strings
    # Type N: gp=True and gs=True
    ans = 0
    for state, count in counts.items():
        k, gp, gs = state
        if gp and gs:
            ans += count

    return ans

def main():
    f60 = solve_dp(60)
    print(f60)

if __name__ == "__main__":
    main()
