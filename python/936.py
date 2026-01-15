"""
Project Euler Problem 936: Peerless Trees

A peerless tree is a tree with no edge between two vertices of the same degree.
P(n) = number of peerless trees on n unlabelled vertices.

Given: P(7) = 6, S(10) = 74
Find: S(50) where S(N) = sum of P(n) for n from 3 to N
"""

import sys

# Set recursion limit just in case, though we are using iterative DP
sys.setrecursionlimit(2000)

def binom(n, k):
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1
    if k > n // 2:
        k = n - k

    res = 1
    for i in range(k):
        res = res * (n - i) // (i + 1)
    return res

class Poly:
    def __init__(self, max_x, max_y):
        self.max_x = max_x
        self.max_y = max_y
        # coeffs[x][y]
        self.coeffs = [[0] * (max_y + 1) for _ in range(max_x + 1)]
        self.coeffs[0][0] = 1

    def copy(self):
        new_poly = Poly(self.max_x, self.max_y)
        for x in range(self.max_x + 1):
            for y in range(self.max_y + 1):
                new_poly.coeffs[x][y] = self.coeffs[x][y]
        return new_poly

    def multiply_by_inv_factor(self, s, count):
        """
        Multiplies the polynomial by (1 - x^s * y)^(-count).
        This adds items to the pool (Infinite Knapsack logic).
        Expands to sum_{j=0} binom(count+j-1, j) * (x^s * y)^j
        """
        # Iterate x increasing, y increasing (because we are adding current values to future values)
        # G[x][y] += sum G[x-js][y-j] * binom
        # To do this in place, we must process order correctly.
        # Since we use values from 'current' iteration (if j > 1), we should treat as one item type at a time?
        # No, (1 - Z)^(-C) is a single factor.
        # We can implement it as a convolution.
        # But in-place update for "Infinite Knapsack" typically goes increasing order
        # if we treat it as 1 item.
        # Here we have a "super item" with expansion.
        # Let's use a temporary buffer or handle carefully.
        # Actually, if we process x increasing, y increasing:
        # At (x, y), we add contributions from (x-s, y-1), (x-2s, y-2), etc.
        # If we update (x, y) immediately, does it affect (x+s, y+1)?
        # Yes.
        # Is (1 - Z)^-C = (1 + Z + Z^2 ...) ^ C?
        # No. It is 1 + C Z + ...
        # If we process x increasing, it's like using the item multiple times?
        # The expansion accounts for multiplicity.
        # So we should strictly convolve.
        # To do strictly convolve in place, we iterate decreasing.

        # Max j such that s*j <= max_x and j <= max_y
        max_j = min(self.max_x // s, self.max_y)

        for x in range(self.max_x, -1, -1):
            for y in range(self.max_y, -1, -1):
                # We want to add contributions from lower terms.
                # G_new[x][y] = G_old[x][y] + G_old[x-s][y-1]*C1 + G_old[x-2s][y-2]*C2 ...
                # Since we iterate decreasing, G[x][y] is currently G_old[x][y].
                # And we look at G[x-s][y-1], which is also G_old (since x-s < x).
                # So decreasing order works.

                term = 0
                for j in range(1, max_j + 1):
                    px = x - j * s
                    py = y - j
                    if px < 0 or py < 0:
                        break

                    # C(count+j-1, j)
                    c = binom(count + j - 1, j)
                    term += self.coeffs[px][py] * c

                self.coeffs[x][y] += term

    def multiply_by_factor(self, s, count):
        """
        Multiplies by (1 - x^s * y)^count.
        This removes items / finite knapsack logic.
        Expands to sum_{j=0} binom(count, j) * (-1)^j * (x^s * y)^j
        """
        # Also strictly convolve. Iterate decreasing.
        max_j = min(self.max_x // s, self.max_y)
        max_j = min(max_j, count) # Since powers limited by count

        for x in range(self.max_x, -1, -1):
            for y in range(self.max_y, -1, -1):
                term = 0
                for j in range(1, max_j + 1):
                    px = x - j * s
                    py = y - j
                    if px < 0 or py < 0:
                        break

                    c = binom(count, j)
                    if j % 2 == 1:
                        term -= self.coeffs[px][py] * c
                    else:
                        term += self.coeffs[px][py] * c

                self.coeffs[x][y] += term

def solve():
    MAX_N = 50
    # A[n][k] stores number of rooted trees of size n, root degree k (valid assuming root degree k+1)
    A = [[0] * (MAX_N + 1) for _ in range(MAX_N + 1)]

    # G_total stores the generating function of available subtrees (A types)
    G_total = Poly(MAX_N, MAX_N) # Max size MAX_N, max root degree MAX_N

    S_total = 0

    for n in range(1, MAX_N + 1):
        # Step 1: Compute A[n][K]
        # We need coefficient of x^{n-1} y^K from "G_total without forbidden types"
        # Forbidden types for target K are those with k = K
        # We iterate over possible target degrees K

        # Optimization: We only need row n-1.
        # G_total already contains contribution of all trees size < n.
        # But we need to temporarily modify it for each K.
        # Instead of full copy, we can just compute the specific coefficient?
        # But modification involves polynomial multiplication which affects many terms.
        # For small K, we remove many factors? No.
        # Factors to remove are items with k = K.
        # These are trees of size s < n with root degree K.
        # There are few such trees.
        # We can copy G_total.

        for K in range(n): # Target root degree (number of children)
            # Forbidden trees are those with root_degree_in_subtree = K.
            # (which means they would connect to our root (deg K+1) with deg K+1? No.)
            # Wait. Logic check.
            # A[s][k] counts trees where root has k children.
            # When attached to our root (degree K+1), the child root has degree k+1.
            # We require deg(our root) != deg(child root).
            # K+1 != k+1 => K != k.
            # So forbidden items are those with k = K.

            # Construct list of forbidden items (s, count) where k=K
            forbidden_items = []
            for s in range(1, n): # sizes < n
                count = A[s][K]
                if count > 0:
                    forbidden_items.append((s, count))

            if not forbidden_items:
                # No forbidden items, just take coeff
                A[n][K] = G_total.coeffs[n-1][K]
            else:
                # Need to remove forbidden items.
                # Since we only need one coefficient [x^{n-1} y^K],
                # and we are multiplying G by P_corr = product (1 - x^s y)^count.
                # P_corr = 1 + terms with y^1, y^2...
                # Coeff = sum_{j} G[n-1 - shift_x][K - j] * P_corr_coeff[shift_x][j]
                # We can construct P_corr for just the necessary range?
                # Max y power in P_corr we care about is K.
                # Actually, forbidden items have 'y' factor.
                # P_corr has terms x^s y.
                # So P_corr is a polynomial.
                # We can build P_corr in a small temp poly?
                # Max x needed is n-1. Max y needed is K.

                # Let's just clone and run multiply_by_factor (remove).
                # It might be slightly slow but safe.
                temp_G = G_total.copy() # Optimizable if needed
                for s, count in forbidden_items:
                    temp_G.multiply_by_factor(s, count)

                A[n][K] = temp_G.coeffs[n-1][K]

        # Step 2: Add newly computed A[n][K] trees to G_total
        # They are added as items (size n, 'weight' 1 in y) with multiplicity A[n][K]
        # Factor: (1 - x^n y)^{-A[n][K]}
        # Since x^n is added, it only affects terms x >= n.
        # We are at step n. Future queries will be for size > n.
        # So we can update in place safely.
        for K in range(n):
            count = A[n][K]
            if count > 0:
                G_total.multiply_by_inv_factor(n, count)

        # Step 3: Compute P(n)
        if n < 3:
            continue

        p_n = 0

        # Case 1: Single Centroid
        # We need trees of size n rooted at centroid.
        # Subtrees must have size <= (n-1)//2.
        # We need to sum over all possible root degrees D.
        # Root degree D in full tree.
        # Condition: D != deg(child). Child deg is k+1.
        # So forbidden k = D-1.

        # We need a generating function G_small containing only trees of size <= (n-1)//2.
        # Since this G_small depends on n, we rebuild it.
        # Optimization: (n-1)//2 increases monotonically. We can maintain G_small?
        # Yes, G_small contains items up to size M.
        # When M increases, we add new items.
        # But let's just rebuild for simplicity first. n=50 is small.

        limit_s = (n - 1) // 2
        G_small = Poly(n-1, n-1) # We need coeff x^{n-1}. Max y can be n-1.

        # Add all valid small trees to G_small
        for s in range(1, limit_s + 1):
            for k in range(s): # max k is s-1
                count = A[s][k]
                if count > 0:
                    G_small.multiply_by_inv_factor(s, count)

        for D in range(n): # Root degree
            # Forbidden k = D - 1
            forbidden_k = D - 1

            # Check if we need to remove anything
            # We need to remove items from G_small that have k = forbidden_k
            forbidden_items = []
            if forbidden_k >= 0:
                for s in range(1, limit_s + 1):
                    if forbidden_k < len(A[s]):
                        count = A[s][forbidden_k]
                        if count > 0:
                            forbidden_items.append((s, count))

            val = 0
            if not forbidden_items:
                val = G_small.coeffs[n-1][D]
            else:
                temp_G = G_small.copy()
                for s, count in forbidden_items:
                    temp_G.multiply_by_factor(s, count)
                val = temp_G.coeffs[n-1][D]

            p_n += val

        # Case 2: Bicentroid (only if n is even)
        if n % 2 == 0:
            half = n // 2
            # We need to pair two trees of size 'half'.
            # A[half][k] gives count of trees with root degree k (subtree).
            # In full tree, root degrees become k+1.
            # Let trees be T1, T2 with root child-counts k1, k2.
            # Degs: k1+1, k2+1.
            # Edge condition: k1+1 != k2+1 => k1 != k2.
            # Pairs must be unordered.
            # Iterate k1 < k2.

            # Get counts for size 'half'
            counts = A[half]
            # k can go up to half-1
            for k1 in range(len(counts)):
                if counts[k1] == 0: continue
                for k2 in range(k1 + 1, len(counts)):
                    if counts[k2] == 0: continue
                    p_n += counts[k1] * counts[k2]

        S_total += p_n
        if n == 7:
            print(f"P(7) = {p_n}")
        if n == 10:
            print(f"S(10) = {S_total}")

    return S_total

if __name__ == "__main__":
    result = solve()
    print(f"S(50) = {result}")

    # Save answer
    with open("unsolved/936/answer.txt", "w") as f:
        f.write(str(result))
