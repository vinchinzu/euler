/*
 * Project Euler Problem 177: Integer Angled Quadrilaterals
 *
 * Count non-similar quadrilaterals where all 8 interior angles
 * (formed by diagonals) are integer degrees.
 *
 * Iterate over angles a,b,c,d, compute f via atan, check if integer.
 * Canonicalize by rotation/reflection.
 */
#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>

#define C 180

/* Hash set for long long values */
#define HASH_SIZE (1 << 20)
#define HASH_MASK (HASH_SIZE - 1)

typedef struct { long long key; int used; } HEntry;
static HEntry htable[HASH_SIZE];
static int ht_count = 0;

static int ht_insert(long long key) {
    unsigned int idx = (unsigned int)((unsigned long long)key * 0xFF51AFD7ED558CCDULL >> 44) & HASH_MASK;
    while (1) {
        if (!htable[idx].used) {
            htable[idx].key = key;
            htable[idx].used = 1;
            ht_count++;
            return 1;
        }
        if (htable[idx].key == key) return 0;
        idx = (idx + 1) & HASH_MASK;
    }
}

int main(void) {
    double RAD = M_PI / 180.0;
    double sin_table[C], cos_table[C];
    for (int i = 0; i < C; i++) {
        sin_table[i] = sin(i * RAD);
        cos_table[i] = cos(i * RAD);
    }

    memset(htable, 0, sizeof(htable));

    for (int a = 1; a <= C / 4; a++) {
        for (int b = a; b <= C - 3 * a; b++) {
            for (int c = a; c <= C - 2 * a - b; c++) {
                for (int d = a; d <= C - a - b - c; d++) {
                    double AD = sin_table[c] / sin_table[a + b + c];
                    double denom = sin_table[b + c + d];
                    if (denom == 0.0) continue;
                    double AC = sin_table[c + d] / denom;
                    double diff = AC - AD * cos_table[a];
                    if (diff == 0.0) continue;
                    double f = atan((AD * sin_table[a]) / diff) * 180.0 / M_PI;
                    int fi = (int)(f + (f >= 0 ? 0.5 : -0.5));
                    if (fabs(f - fi) > 1e-9) continue;
                    if (fi < 0) fi += C;

                    int angles[8];
                    angles[0] = a;
                    angles[1] = b;
                    angles[2] = c;
                    angles[3] = d;
                    angles[4] = C - b - c - d;
                    angles[5] = fi;
                    angles[6] = b + c - fi;
                    angles[7] = C - a - b - c;

                    int ok = 1;
                    for (int i = 0; i < 8; i++)
                        if (angles[i] <= 0) { ok = 0; break; }
                    if (!ok) continue;

                    /* Canonicalize */
                    long long min_hash = -1;
                    int first = 1;
                    for (int start = 0; start < 8; start++) {
                        long long h = 0;
                        if (start % 2 == 0) {
                            for (int i = 0; i < 8; i++)
                                h = h * C + angles[(start + i) % 8];
                        } else {
                            for (int i = 0; i < 8; i++)
                                h = h * C + angles[((start - i) % 8 + 8) % 8];
                        }
                        if (first || h < min_hash) {
                            min_hash = h;
                            first = 0;
                        }
                    }
                    ht_insert(min_hash);
                }
            }
        }
    }

    printf("%d\n", ht_count);
    return 0;
}
