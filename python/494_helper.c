#include <stdio.h>

// M = 3^39
// We compute it at runtime since it's too large for a constant
static unsigned long long M;

unsigned long long helper(unsigned long long n, int numSteps, int nEven) {
    if (numSteps == 0)
        return 1;
    unsigned long long result = helper((n * 2) % M, numSteps - 1, 1);
    if (nEven && n % 3 == 1)
        result += helper((n - 1) / 3, numSteps - 1, 0);
    return result;
}

int main(int argc, char **argv) {
    // Compute M = 3^39
    M = 1;
    for (int i = 0; i < 39; i++)
        M *= 3;

    unsigned long long start;
    int steps;
    int nEven;
    sscanf(argv[1], "%llu", &start);
    sscanf(argv[2], "%d", &steps);
    sscanf(argv[3], "%d", &nEven);

    unsigned long long result = helper(start, steps, nEven);
    printf("%llu\n", result);
    return 0;
}
