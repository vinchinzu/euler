"""Project Euler Problem 497: Drunken Tower of Hanoi.

Ported from Java solution.
"""

def solve():
    N = 10000
    M = 10**9

    # numMoves[n][start][end][s][e] = number of times Bob moves from rod s to rod e
    # when moving n disks from start to end
    # We only need current and previous n, but the array is manageable for N=10000
    # Actually N=10000 with 3^4=81 entries per n is fine as a list of dicts

    # Use a flat approach: numMoves[n] is a 3x3x3x3 array
    # But storing all N levels would use too much memory (10000 * 81 longs)
    # Actually 10000 * 81 = 810000 entries, that's fine

    numMoves = [[[[[0]*3 for _ in range(3)] for _ in range(3)] for _ in range(3)] for _ in range(N+1)]

    for start in range(3):
        for end in range(3):
            if start != end:
                numMoves[1][start][end][start][end] = 1

    for n in range(2, N+1):
        for start in range(3):
            for end in range(3):
                if start != end:
                    other = 3 - start - end
                    for s in range(3):
                        for e in range(3):
                            numMoves[n][start][end][s][e] += numMoves[n-1][start][other][s][e]
                    numMoves[n][start][end][other][start] += 1
                    numMoves[n][start][end][start][end] += 1
                    numMoves[n][start][end][end][other] += 1
                    for s in range(3):
                        for e in range(3):
                            numMoves[n][start][end][s][e] += numMoves[n-1][other][end][s][e]
                            numMoves[n][start][end][s][e] %= M

    ans = 0
    for n in range(1, N+1):
        k = pow(10, n, M)
        rods = [pow(3, n, M), pow(6, n, M), pow(9, n, M)]
        for s in range(3):
            for e in range(3):
                if s < e:
                    dist = (rods[e] - 1) * (rods[e] - 1) - (rods[s] - 1) * (rods[s] - 1)
                else:
                    dist = (k - rods[e]) * (k - rods[e]) - (k - rods[s]) * (k - rods[s])
                # Java: numMoves[1][1][0][s][e] + numMoves[n][0][2][s][e]
                # The first term accounts for Bob moving from rod b(=1) to rod a(=0) initially
                count = (numMoves[1][1][0][s][e] + numMoves[n][0][2][s][e]) % M
                ans += dist % M * count % M

    ans = ans % M
    return ans


if __name__ == "__main__":
    print(solve())
