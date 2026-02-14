/*
 * Project Euler Problem 336 - Maximix Arrangements
 *
 * Find the 2011th lexicographically smallest Maximix arrangement for
 * 11 carriages (labeled A-K).
 *
 * A Maximix arrangement requires exactly n-1 prefix reversals (pancake flips)
 * to sort using the greedy algorithm: at each step, find the largest unsorted
 * element and flip it into place.
 *
 * The sorting algorithm:
 * For i from n down to 2:
 *   If element i is not in position i:
 *     If element i is not in position 1: reverse prefix up to position of i
 *     Reverse prefix of length i (puts i in position i)
 *
 * A Maximix arrangement uses all n-1 "rounds" (each round for i = n..2 does work).
 * That means: for each i from n down to 2, element i is NOT already in position i.
 *
 * We enumerate by building the arrangement position by position using
 * backtracking with counting.
 */
#include <stdio.h>
#include <string.h>

#define N 11
#define TARGET 2011

/* Simulate the sorting process and count how many rounds are "active" */
/* Return the number of active rounds (where element i is not in position i) */

/* We need to check if an arrangement is "maximix" = uses all n-1 rounds.
 * Actually, we need to count maximix arrangements and find the 2011th. */

/* State: partial arrangement of elements. We fix elements from right to left
 * (or use a different enumeration strategy). */

/* Approach: simulate the greedy sorting algorithm on a given permutation
 * and check if every round i (from n down to 2) requires at least one flip. */

void reverse_prefix(int *arr, int len) {
    for (int i = 0, j = len - 1; i < j; i++, j--) {
        int tmp = arr[i]; arr[i] = arr[j]; arr[j] = tmp;
    }
}

int count_rounds(int *perm, int n) {
    int arr[N];
    memcpy(arr, perm, n * sizeof(int));
    int rounds = 0;
    for (int i = n; i >= 2; i--) {
        /* Find position of element i (0-indexed: element i-1 in 0-indexed) */
        int pos = -1;
        for (int j = 0; j < i; j++)
            if (arr[j] == i) { pos = j; break; }
        if (pos == i - 1) continue; /* already in place */
        rounds++;
        if (pos != 0)
            reverse_prefix(arr, pos + 1);
        reverse_prefix(arr, i);
    }
    return rounds;
}

/* Generate all permutations in lexicographic order and count maximix ones */
/* For n=11, there are 11! = ~40M permutations. We need to be smarter. */

/* Better approach: enumerate by building from left, pruning branches that
 * can't possibly be maximix. */

/* For a Maximix arrangement of n elements, we need exactly n-1 active rounds.
 * Round i is active if element i is not in position i before round i starts.
 *
 * The greedy sorting processes from largest to smallest. For the arrangement
 * to be maximix, after placing elements n, n-1, ..., i+1 in their positions,
 * element i must NOT already be in position i.
 *
 * Key insight: we can simulate the sorting in reverse. For each candidate
 * permutation, check all rounds. But 11! is too many to enumerate naively.
 *
 * Alternative: Use the recursive structure. We can count how many maximix
 * arrangements start with a given prefix, and use this to find the 2011th
 * without full enumeration.
 */

/* Actually, 11! = 39916800 which is feasible to enumerate if we're fast.
 * Let's just generate permutations in lex order and count maximix ones. */

int perm[N];
int result_count;
char answer[N + 1];

/* Check if the current permutation (1-indexed values in perm[0..N-1]) is maximix */
int is_maximix(void) {
    return count_rounds(perm, N) == N - 1;
}

/* Generate permutations in lexicographic order */
int used[N + 1]; /* used[v] = 1 if value v is placed */

int found;

void gen(int pos) {
    if (found) return;
    if (pos == N) {
        if (is_maximix()) {
            result_count++;
            if (result_count == TARGET) {
                for (int i = 0; i < N; i++)
                    answer[i] = 'A' + perm[i] - 1;
                answer[N] = '\0';
                found = 1;
            }
        }
        return;
    }
    for (int v = 1; v <= N; v++) {
        if (used[v]) continue;
        perm[pos] = v;
        used[v] = 1;
        gen(pos + 1);
        if (found) return;
        used[v] = 0;
    }
}

int main(void) {
    memset(used, 0, sizeof(used));
    result_count = 0;
    found = 0;
    gen(0);
    printf("%s\n", answer);
    return 0;
}
