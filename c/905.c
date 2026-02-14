#include <stdio.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 i128;

ll next_turn(ll prev_turn, int player) {
    ll base = player + 1;
    if (base > prev_turn) return base;
    ll k = (prev_turn - base) / 3 + 1;
    return base + 3 * k;
}

/* Power function for big integers using __int128 */
ll ipow(ll base, int exp) {
    i128 result = 1;
    i128 b = base;
    for (int i = 0; i < exp; i++) {
        result *= b;
        /* Check overflow - if too big, cap at a large value */
        if (result > (i128)1e30) return (ll)1e18;
    }
    return (ll)result;
}

/* GCD for big values */
ll gcd_ll(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

ll F(ll A, ll B, ll C) {
    ll cur[3] = {A, B, C};
    int batch_max_idx[10000];
    int batch_mid_idx[10000];
    ll batch_q[10000];
    int num_batches = 0;

    while (1) {
        /* Find max, mid, min */
        int max_i = 0;
        if (cur[1] > cur[max_i]) max_i = 1;
        if (cur[2] > cur[max_i]) max_i = 2;

        ll min_val = -1, mid_val = -1;
        int min_i = -1, mid_i = -1;
        for (int i = 0; i < 3; i++) {
            if (i == max_i) continue;
            if (min_i == -1 || cur[i] < min_val || (cur[i] == min_val && i < min_i)) {
                if (mid_i == -1) {
                    mid_val = min_val; mid_i = min_i;
                }
                min_val = cur[i]; min_i = i;
            } else {
                mid_val = cur[i]; mid_i = i;
            }
        }

        /* Fix: properly find min and mid */
        /* Re-do: find two non-max indices, sorted by value */
        int others[2], oi = 0;
        for (int i = 0; i < 3; i++) if (i != max_i) others[oi++] = i;
        if (cur[others[0]] > cur[others[1]]) {
            int t = others[0]; others[0] = others[1]; others[1] = t;
        }
        min_i = others[0]; min_val = cur[min_i];
        mid_i = others[1]; mid_val = cur[mid_i];

        if (min_val == 0) break;

        ll q = mid_val / min_val;
        ll remainder = mid_val % min_val;

        batch_max_idx[num_batches] = max_i;
        batch_mid_idx[num_batches] = mid_i;
        batch_q[num_batches] = q;
        num_batches++;

        if (q % 2 == 1) {
            cur[max_i] = remainder;
            cur[mid_i] = remainder + min_val;
        } else {
            cur[mid_i] = remainder;
            cur[max_i] = remainder + min_val;
        }
    }

    /* Reverse batches */
    for (int i = 0; i < num_batches / 2; i++) {
        int t;
        t = batch_max_idx[i]; batch_max_idx[i] = batch_max_idx[num_batches - 1 - i]; batch_max_idx[num_batches - 1 - i] = t;
        t = batch_mid_idx[i]; batch_mid_idx[i] = batch_mid_idx[num_batches - 1 - i]; batch_mid_idx[num_batches - 1 - i] = t;
        ll tq = batch_q[i]; batch_q[i] = batch_q[num_batches - 1 - i]; batch_q[num_batches - 1 - i] = tq;
    }

    ll prev_turn = 0;

    for (int b = 0; b < num_batches; b++) {
        int p_a = batch_max_idx[b];
        int p_b = batch_mid_idx[b];
        ll q = batch_q[b];

        int first_player, second_player;
        if (q % 2 == 1) {
            first_player = p_a;
            second_player = p_b;
        } else {
            first_player = p_b;
            second_player = p_a;
        }

        if (q == 0) continue;

        prev_turn = next_turn(prev_turn, first_player);
        ll remaining = q - 1;
        if (remaining == 0) continue;

        prev_turn = next_turn(prev_turn, second_player);
        remaining--;
        if (remaining == 0) continue;

        ll pairs = remaining / 2;
        ll leftover = remaining % 2;

        prev_turn += pairs * 3;
        if (leftover) {
            prev_turn = next_turn(prev_turn, first_player);
        }
    }

    return prev_turn;
}

int main(void) {
    /* Verify */
    /* F(2,1,1) should be 1 */
    /* F(2,7,5) should be 5 */

    ll total = 0;
    for (int a = 1; a <= 7; a++) {
        for (int b = 1; b <= 19; b++) {
            /* A_val = a^b, B_val = b^a, C_val = A_val + B_val */
            /* Need big integer power - use __int128 */
            i128 A_val_128 = 1;
            for (int i = 0; i < b; i++) A_val_128 *= a;
            i128 B_val_128 = 1;
            for (int i = 0; i < a; i++) B_val_128 *= b;
            i128 C_val_128 = A_val_128 + B_val_128;

            /* These can be very large (7^19 ~ 1.1e16, 19^7 ~ 8.9e8)
               but fit in long long */
            ll A_val = (ll)A_val_128;
            ll B_val = (ll)B_val_128;
            ll C_val = (ll)C_val_128;

            ll f = F(A_val, B_val, C_val);
            total += f;
        }
    }
    printf("%lld\n", total);
    return 0;
}
