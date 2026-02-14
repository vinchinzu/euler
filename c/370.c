#include <stdio.h>
#include <math.h>
typedef long long ll;

#define MAXQ 2900000
int spf[MAXQ + 1];
void sieve() {
    for (int i=0; i<=MAXQ; i++) spf[i]=i;
    for (int i=2; (ll)i*i<=MAXQ; i++)
        if (spf[i]==i) for (int j=i*i; j<=MAXQ; j+=i) if(spf[j]==j) spf[j]=i;
}
int get_pf(int q, int f[]) {
    int n=0;
    while(q>1){int p=spf[q]; f[n++]=p; while(q%p==0) q/=p;}
    return n;
}

ll sum_floor(ll L, ll a, ll b, ll c, ll R) {
    if (R <= 0) return 0;
    ll D1 = a + b + c;
    if (D1 > L) return 0;
    {
        ll DR = a + b*R + c*R*R;
        if (DR > L) {
            double disc = (double)b*b - 4.0*(double)c*((double)a - (double)L);
            if (disc < 0) return 0;
            R = (ll)((-b + sqrt(disc)) / (2.0*c));
            while (R + 1 >= 1 && a + b*(R+1) + c*(R+1)*(R+1) <= L) R++;
            while (R >= 1 && a + b*R + c*R*R > L) R--;
            if (R <= 0) return 0;
        }
    }
    ll DR = a + b*R + c*R*R;
    ll t_max = L / D1;
    ll t_min = L / DR;
    if (t_max - t_min + 1 < R) {
        ll sum = 0; ll prev_r = 0;
        for (ll t = t_max; t >= t_min && t >= 1; t--) {
            ll Dt = L / t;
            double disc = (double)b*b - 4.0*(double)c*((double)a - (double)Dt);
            if (disc < 0) continue;
            ll r_up = (ll)((-b + sqrt(disc)) / (2.0*c));
            while (r_up + 1 <= R && a + b*(r_up+1) + c*(r_up+1)*(r_up+1) <= Dt) r_up++;
            while (r_up >= 1 && a + b*r_up + c*r_up*r_up > Dt) r_up--;
            if (r_up > R) r_up = R;
            if (r_up <= prev_r) continue;
            sum += t * (r_up - prev_r);
            prev_r = r_up;
            if (prev_r >= R) break;
        }
        return sum;
    } else {
        ll sum = 0;
        for (ll rp = 1; rp <= R; rp++) {
            ll D = a + b*rp + c*rp*rp;
            if (D > L) break;
            sum += L / D;
        }
        return sum;
    }
}

int main() {
    ll L = 25000000000000LL;
    sieve();
    ll total = L / 3;
    ll Q = (ll)sqrt(L / 3.0) + 2;
    while (3*Q*Q + 3*Q + 1 > L) Q--;
    if (Q > MAXQ) Q = MAXQ;

    for (ll q = 1; q <= Q; q++) {
        if (3*q*q + 3*q + 1 > L) break;
        int factors[20]; int nf = get_pf((int)q, factors);
        ll max_r = (ll)(q * 0.6180339887498949);
        while (max_r >= 1 && q*q <= q*max_r + max_r*max_r) max_r--;
        if (max_r < 1) continue;
        int nsub = 1 << nf;
        for (int mask = 0; mask < nsub; mask++) {
            int sign = 1; ll d = 1;
            for (int i = 0; i < nf; i++)
                if (mask & (1 << i)) { sign = -sign; d *= factors[i]; }
            ll R = max_r / d;
            if (R < 1) continue;
            ll s = sum_floor(L, 3*q*q, 3*q*d, d*d, R);
            total += sign * s;
        }
    }
    printf("%lld\n", total);
    return 0;
}
