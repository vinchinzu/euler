#!/usr/bin/env python3
"""
Debug: Verify c(2,k) by checking specific matrices.
"""

def complexity(matrix):
    n = len(matrix)
    row_set = set()
    for row in matrix:
        row_set.add(tuple(row))
    col_set = set()
    for j in range(n):
        col = tuple(matrix[i][j] for i in range(n))
        col_set.add(col)
    return len(row_set), len(col_set), len(row_set) + len(col_set)

# n=2: All 2x2 binary matrices
# k=0: [[0,0],[0,0]] -> 1 distinct row + 1 distinct col = 2. OK
# k=1: [[1,0],[0,0]] -> rows (1,0),(0,0) = 2; cols (1,0),(0,0) = 2. complexity=4
#       Is there a way to get k=1 with lower complexity? No, with 1 one in a 2x2 matrix,
#       you always have 2 distinct rows and 2 distinct cols.
# k=2: Several options:
#   [[1,1],[0,0]]: rows (1,1),(0,0)=2; cols (1,0),(1,0)=1. complexity=3
#   [[1,0],[1,0]]: rows (1,0),(1,0)=1; cols (1,1),(0,0)=2. complexity=3
#   [[1,0],[0,1]]: rows (1,0),(0,1)=2; cols (1,0),(0,1)=2. complexity=4
#   [[0,1],[1,0]]: same as above, complexity=4
# So c(2,2) = 3. Let me verify:

print("n=2 verification:")
mats = [
    [[0,0],[0,0]],   # k=0
    [[1,0],[0,0]],   # k=1
    [[1,1],[0,0]],   # k=2, comp=3
    [[1,0],[1,0]],   # k=2, comp=3
    [[1,0],[0,1]],   # k=2, comp=4
    [[1,1],[1,0]],   # k=3
    [[1,1],[1,1]],   # k=4
]
for m in mats:
    k = sum(sum(row) for row in m)
    r, c, comp = complexity(m)
    print(f"  {m}: k={k}, rows={r}, cols={c}, comp={comp}")

# C(2) should be 8 according to problem, but I got 15
# Let me re-read: c(n,k) = minimum complexity over all n×n binary matrices with exactly k ones
# C(n) = sum_{k=0}^{n^2} c(n,k)
# For n=2: c(2,0)=2, c(2,1)=4, c(2,2)=3, c(2,3)=4, c(2,4)=2
# Sum = 2+4+3+4+2 = 15, not 8.

# Wait, let me re-read the problem more carefully.
# Maybe the problem defines complexity differently?
# "The complexity of an n×n binary matrix is the number of distinct rows plus distinct columns"
# Hmm, maybe "distinct" means something else? Like the number of rows that are distinct from ALL others?

# Alternative interpretation: "distinct rows" = rows that appear exactly once (unique rows)
print("\n\nAlternative: count rows appearing exactly once + cols appearing exactly once")

def complexity_v2(matrix):
    """Count rows appearing exactly once + cols appearing exactly once."""
    n = len(matrix)
    from collections import Counter
    row_counts = Counter()
    for row in matrix:
        row_counts[tuple(row)] += 1
    col_counts = Counter()
    for j in range(n):
        col = tuple(matrix[i][j] for i in range(n))
        col_counts[col] += 1
    unique_rows = sum(1 for v in row_counts.values() if v == 1)
    unique_cols = sum(1 for v in col_counts.values() if v == 1)
    return unique_rows, unique_cols, unique_rows + unique_cols

for m in mats:
    k = sum(sum(row) for row in m)
    r, c, comp = complexity_v2(m)
    print(f"  {m}: k={k}, unique_rows={r}, unique_cols={c}, comp={comp}")
