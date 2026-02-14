#include <stdio.h>
#include <string.h>

#define N 11
#define TARGET 2011
#define MAX_ROT (2*(N-1)-1)

void reverse_suffix(int *arr, int start, int n) {
    for (int i = start, j = n - 1; i < j; i++, j--) {
        int tmp = arr[i]; arr[i] = arr[j]; arr[j] = tmp;
    }
}

int count_rotations(int *perm, int n) {
    int arr[N]; memcpy(arr, perm, n * sizeof(int));
    int rotations = 0;
    for (int target = 1; target <= n - 1; target++) {
        int pos = -1;
        for (int j = target - 1; j < n; j++) if (arr[j] == target) { pos = j; break; }
        if (pos == target - 1) continue;
        if (pos != n - 1) { reverse_suffix(arr, pos, n); rotations++; }
        reverse_suffix(arr, target - 1, n); rotations++;
    }
    return rotations;
}

int perm[N], result_count, found;
char answer[N + 1];

/* Generate by swapping: perm[0..pos-1] fixed, perm[pos..N-1] has remaining values */
void gen(int pos) {
    if (found) return;
    if (pos == N) {
        if (count_rotations(perm, N) == MAX_ROT) {
            result_count++;
            if (result_count == TARGET) {
                for (int i = 0; i < N; i++) answer[i] = 'A' + perm[i] - 1;
                answer[N] = '\0';
                found = 1;
            }
        }
        return;
    }

    /* Sort perm[pos..N-1] to ensure lex order */
    int nrem = N - pos;
    for (int i = pos; i < N - 1; i++)
        for (int j = i + 1; j < N; j++)
            if (perm[i] > perm[j]) {
                int tmp = perm[i]; perm[i] = perm[j]; perm[j] = tmp;
            }

    /* Try each value in lex order */
    int saved[N];
    memcpy(saved, perm + pos, nrem * sizeof(int));

    for (int ri = 0; ri < nrem; ri++) {
        /* Put saved[ri] at position pos */
        memcpy(perm + pos, saved, nrem * sizeof(int));
        int v = saved[ri];
        /* Remove saved[ri] and shift rest */
        perm[pos] = v;
        int idx = pos + 1;
        for (int j = 0; j < nrem; j++) {
            if (j == ri) continue;
            perm[idx++] = saved[j];
        }

        gen(pos + 1);
        if (found) return;
    }
    /* Restore */
    memcpy(perm + pos, saved, nrem * sizeof(int));
}

int main(void) {
    for (int i = 0; i < N; i++) perm[i] = i + 1;
    result_count = 0; found = 0;
    gen(0);
    if (found) printf("%s\n", answer);
    else printf("NOT FOUND\n");
    return 0;
}
