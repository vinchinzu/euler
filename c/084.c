#include <stdio.h>
#include <stdlib.h>

#define BOARD_SIZE 40
#define DICE_SIDES 4
#define SIMULATIONS 10000000

/* Board positions */
#define GO 0
#define JAIL 10
#define G2J 30
#define CC1 2
#define CC2 17
#define CC3 33
#define CH1 7
#define CH2 22
#define CH3 36
#define C1 11
#define E3 24
#define H2 39
#define R1 5
#define R2 15
#define R3 25
#define R4 35
#define U1 12
#define U2 28

/* Simple LCG random number generator */
static unsigned long rng_state = 12345;
int rand_int(int lo, int hi) {
    rng_state = rng_state * 6364136223846793005ULL + 1442695040888963407ULL;
    return lo + (int)((rng_state >> 33) % (unsigned long)(hi - lo + 1));
}

int next_railway(int pos) {
    if (pos < R1) return R1;
    if (pos < R2) return R2;
    if (pos < R3) return R3;
    if (pos < R4) return R4;
    return R1;
}

int next_utility(int pos) {
    if (pos < U1 || pos >= U2) return U1;
    return U2;
}

int process_cc(int pos) {
    int card = rand_int(0, 15);
    if (card == 0) return GO;
    if (card == 1) return JAIL;
    return pos;
}

int process_ch(int pos) {
    int card = rand_int(0, 15);
    if (card == 0) return GO;
    if (card == 1) return JAIL;
    if (card == 2) return C1;
    if (card == 3) return E3;
    if (card == 4) return H2;
    if (card == 5) return R1;
    if (card == 6 || card == 7) return next_railway(pos);
    if (card == 8) return next_utility(pos);
    if (card == 9) return (pos + BOARD_SIZE - 3) % BOARD_SIZE;
    return pos;
}

int main(void) {
    long long visits[BOARD_SIZE];
    for (int i = 0; i < BOARD_SIZE; i++) visits[i] = 0;

    int pos = 0;
    int doubles_count = 0;

    for (int sim = 0; sim < SIMULATIONS; sim++) {
        int d1 = rand_int(1, DICE_SIDES);
        int d2 = rand_int(1, DICE_SIDES);

        if (d1 == d2) {
            doubles_count++;
            if (doubles_count == 3) {
                pos = JAIL;
                doubles_count = 0;
                visits[pos]++;
                continue;
            }
        } else {
            doubles_count = 0;
        }

        pos = (pos + d1 + d2) % BOARD_SIZE;

        if (pos == G2J) {
            pos = JAIL;
        } else if (pos == CC1 || pos == CC2 || pos == CC3) {
            pos = process_cc(pos);
        } else if (pos == CH1 || pos == CH2 || pos == CH3) {
            pos = process_ch(pos);
            if (pos == CC1 || pos == CC2 || pos == CC3) {
                pos = process_cc(pos);
            }
        }

        visits[pos]++;
    }

    /* Find top 3 squares by sorting */
    int indices[BOARD_SIZE];
    for (int i = 0; i < BOARD_SIZE; i++) indices[i] = i;

    for (int i = 0; i < 3; i++) {
        for (int j = i + 1; j < BOARD_SIZE; j++) {
            if (visits[indices[j]] > visits[indices[i]]) {
                int tmp = indices[i]; indices[i] = indices[j]; indices[j] = tmp;
            }
        }
    }

    printf("%02d%02d%02d\n", indices[0], indices[1], indices[2]);
    return 0;
}
