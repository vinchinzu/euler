def solve():
    N = 10000
    total = 0.0
    a, b = 0, 1
    c, d = 1, N
    while True:
        total += 1.0 / (2 * b * d * d)
        k = (N + b) // d
        a, b, c, d = c, d, k * c - a, k * d - b
        if a == 1 and b == 1:
            break
    print(f"{total:.13f}")

if __name__ == "__main__":
    solve()
