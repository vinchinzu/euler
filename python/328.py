"""Project Euler Problem 328 - Lowest-cost Search.

C(n) = minimum worst-case cost to find a number in {1..n}.
Find sum_{n=1}^{200000} C(n).

Uses the optimized algorithm from the Java solution based on
tracking (k, s) parameters for the optimal first guess.
"""

def solve():
    N = 200000
    C = [0] * (N + 1)
    k = 0
    s = 0
    right_cost = 1
    ans = 0

    for n in range(2, N + 1):
        C[n] = n - 1 + C[n - 2]

        guess = n - 2 * ((1 << k) + s) - 1
        if guess > 0:
            C[n] = min(C[n], guess + max(C[guess - 1], right_cost))

        # Count trailing ones in s
        num_ending_ones = 0
        tmp = s
        while tmp & 1:
            num_ending_ones += 1
            tmp >>= 1

        if num_ending_ones == k:
            next_k = k + 1
            next_s = 1 if next_k == 1 else 3
            next_right_cost = (next_k + 1) * n - (next_k << (next_k + 1)) + next_k + (-1 if next_k == 1 else 3)
        else:
            next_k = k
            num_remaining_ones = bin(s & -(1 << num_ending_ones)).count('1')
            if num_ending_ones < num_remaining_ones + 3:
                next_s = s + (1 << num_ending_ones)
                next_right_cost = right_cost + ((num_ending_ones - num_remaining_ones) << (num_ending_ones + 1))
            else:
                next_s = s + (1 << (num_remaining_ones + 3))
                next_right_cost = right_cost + (3 << (num_ending_ones + 1))

        next_guess = n - 2 * ((1 << next_k) + next_s) - 1
        if next_guess > 0:
            next_total_cost = next_guess + max(C[next_guess - 1], next_right_cost)
            if next_total_cost <= C[n]:
                k = next_k
                s = next_s
                right_cost = next_right_cost
                C[n] = next_total_cost

        ans += C[n]
        right_cost += k + 1

    return ans

if __name__ == "__main__":
    print(solve())
