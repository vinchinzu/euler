"""Project Euler Problem 349 - Langton's Ant.

After 10^18 moves, how many black squares are there?

Langton's ant enters a "highway" phase after about 10000 steps,
with a period of 104 steps adding 12 black squares per cycle.
"""

def solve():
    N = 10**18
    L = 20000
    P = 104

    blacks = set()
    num_blacks = []
    ax, ay = 0, 0
    # direction: 0=up, 1=right, 2=down, 3=left
    dx = [0, 1, 0, -1]
    dy = [1, 0, -1, 0]
    d = 0

    for step in range(1, L):
        num_blacks.append(len(blacks))
        pos = (ax, ay)
        if pos in blacks:
            d = (d + 1) % 4
            blacks.remove(pos)
        else:
            d = (d - 1) % 4
            blacks.add(pos)
        ax += dx[d]
        ay += dy[d]

    # Find base index: round down (L - P) to multiple of P, then add N % P
    base = ((L - P) // P) * P + N % P
    base = int(base)
    ans = num_blacks[base] + (N - base) // P * (num_blacks[base] - num_blacks[base - P])
    return ans

if __name__ == "__main__":
    print(solve())
