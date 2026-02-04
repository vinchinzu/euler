"""Project Euler Problem 640: Shut the Box.

Bob has cards 1..12 face-up. Each turn rolls two 6-sided dice (x,y),
must toggle exactly one card from {x, y, x+y}. Optimal strategy.
Uses value iteration with in-place updates (Gauss-Seidel).
"""

import subprocess
import tempfile
import os


def solve():
    # Use C for speed - value iteration over 4096 states
    c_code = r"""
#include <stdio.h>
#include <math.h>

#define N 6
#define CARDS 12
#define GOAL ((1 << CARDS) - 1)
#define NUM_STATES (1 << CARDS)

double E[NUM_STATES];

int main() {
    int s, x, y, c, iteration;
    double total, best, val, new_val, max_change;

    // Initialize
    for (s = 0; s < NUM_STATES; s++) {
        E[s] = (s == GOAL) ? 0.0 : 100.0;
    }

    for (iteration = 0; iteration < 1000000; iteration++) {
        max_change = 0.0;
        for (s = 0; s < NUM_STATES; s++) {
            if (s == GOAL) continue;

            total = 0.0;
            for (x = 1; x <= N; x++) {
                for (y = 1; y <= N; y++) {
                    // Options: flip card x, y, or x+y (toggle)
                    int opts[3] = {x, y, x + y};
                    best = 1e18;
                    for (int i = 0; i < 3; i++) {
                        c = opts[i];
                        if (c >= 1 && c <= CARDS) {
                            val = E[s ^ (1 << (c - 1))];
                            if (val < best) best = val;
                        }
                    }
                    total += best;
                }
            }

            new_val = 1.0 + total / (N * N);
            double change = fabs(new_val - E[s]);
            if (change > max_change) max_change = change;
            E[s] = new_val;
        }

        if (max_change < 1e-12) break;
    }

    printf("%.6f\n", E[0]);
    return 0;
}
"""

    with tempfile.NamedTemporaryFile(mode='w', suffix='.c', delete=False) as f:
        f.write(c_code)
        c_path = f.name

    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'],
                      check=True, capture_output=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, check=True,
                              timeout=30)
        return result.stdout.strip()
    finally:
        os.unlink(c_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)


def main():
    result = solve()
    print(result)


if __name__ == "__main__":
    main()
