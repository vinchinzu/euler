
import math

def calculate_area(x):
    total_trap = 0.0
    for i in range(len(x) - 1):
        u, v = x[i], x[i+1]
        h = v - u
        avg_height = (u**4 + v**4) / 2.0
        total_trap += h * avg_height
    return 2.0 - total_trap

def cbrt(y):
    # Cube root preserving sign
    return math.copysign(abs(y)**(1/3.0), y)

def solve_for_n(n, max_iter=100000, tol=1e-13):
    m = n - 1
    # Initialize with linear symmetric guess
    x = [-1.0 + 2.0 * i / m for i in range(m + 1)]

    # Iterative coordinate descent (Gauss-Seidel)
    for _ in range(max_iter):
        max_diff = 0.0
        for k in range(1, m):
            xp = x[k+1]
            xm = x[k-1]

            # Derived from stationarity condition: 4 * x_k^3 = (x_{k+1}^2 + x_{k-1}^2) * (x_{k+1} + x_{k-1})
            rhs = (xp**2 + xm**2) * (xp + xm)
            new_xk = cbrt(rhs / 4.0)

            diff = abs(new_xk - x[k])
            if diff > max_diff:
                max_diff = diff

            x[k] = new_xk

        if max_diff < tol:
            break

    return calculate_area(x)

def solve() -> str:
    # Compute G(101)
    result = solve_for_n(101)
    return f"{result:.9f}"

if __name__ == "__main__":
    print(solve())
