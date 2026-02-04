from math import gcd, isqrt

def solve():
    N = 10**15
    M = 1031**3 + 2  # 1095912793

    ans = 0

    def is_perfect_cube(n):
        if n <= 0:
            return n == 0
        c = round(n ** (1.0/3.0))
        for cc in (c-1, c, c+1):
            if cc >= 0 and cc*cc*cc == n:
                return True
        return False

    # Iterate over (r, s) pairs first, then compute range of g
    # r is odd, r >= 1
    # sign_a = 1: x = g^2*r*(r-4s)^3 (abs), y = 4*g^2*s*(s+2r)^3
    # sign_a = -1: x = g^2*r*(r+4s)^3 (abs), y = 4*g^2*s*(s-2r)^3 (abs)
    # Constraints: |x| <= N, |y| <= N, gcd(r,s)=1, r odd
    #
    # For sign_a=1: loop bound was 4*g^2*s*(s+2r)^3 <= N and g^2*r^3 <= N
    #   => g <= sqrt(N / (4*s*(s+2r)^3))  and  g <= (N/r^3)^(1/2)
    #   Also need g^2*r*(r-4s)^3_abs <= N (the x<=N condition)
    #
    # For sign_a=-1: loop bound was g^2*r*(r+4s)^3 <= N and g^2*r^3 <= N
    #   => g <= sqrt(N / (r*(r+4s)^3))  and  g <= (N/r^3)^(1/2)
    #   Also need 4*g^2*s*(s-2r)^3_abs <= N (the y<=N condition)

    # For sign_a = 1:
    r = 1
    while r * r * r <= N:  # g=1 requires r^3 <= N
        s = 1
        two_r = 2 * r
        while True:
            v = s + two_r
            if 4 * s * v * v * v > N:  # g=1 case
                break
            if gcd(s, r) == 1:
                val1 = r - 4 * s
                val2 = s + two_r
                av1_3 = abs(val1 * val1 * val1)
                v2_3 = val2 * val2 * val2
                maybeCube = 2 * r * s * s
                if not is_perfect_cube(maybeCube):
                    # x = g^2 * r * av1_3, y = 4 * g^2 * s * v2_3
                    # Need x <= N and y <= N
                    # g^2 <= N / (r * av1_3) if av1_3 > 0, else g^2 <= anything (x=0<=N)
                    # g^2 <= N / (4 * s * v2_3)
                    max_g2 = N // (4 * s * v2_3)
                    if av1_3 > 0:
                        max_g2_x = N // (r * av1_3)
                        if max_g2_x < max_g2:
                            max_g2 = max_g2_x
                    max_g = isqrt(max_g2)
                    if max_g >= 1:
                        # Sum over g=1..max_g of (g^2*(r*av1_3 + 4*s*v2_3)) mod M
                        # = (r*av1_3 + 4*s*v2_3) * sum(g^2, g=1..max_g) mod M
                        coeff = r * av1_3 + 4 * s * v2_3
                        # sum of g^2 from 1 to max_g = max_g*(max_g+1)*(2*max_g+1)//6
                        sg = max_g * (max_g + 1) * (2 * max_g + 1) // 6
                        ans = (ans + coeff % M * (sg % M)) % M
            s += 1
        r += 2

    # For sign_a = -1:
    r = 1
    while r * r * r <= N:
        s = 1
        while True:
            v = r + 4 * s
            if r * v * v * v > N:  # g=1 case
                break
            if gcd(r, s) == 1:
                val1 = r + 4 * s
                val2 = s - 2 * r
                v1_3 = val1 * val1 * val1  # always positive since r,s>0
                av2_3 = abs(val2 * val2 * val2)
                maybeCube = 2 * r * s * s
                if not is_perfect_cube(maybeCube):
                    # x = g^2 * r * v1_3, y = 4 * g^2 * s * av2_3
                    # Need x <= N and y <= N
                    max_g2 = N // (r * v1_3)
                    if av2_3 > 0:
                        max_g2_y = N // (4 * s * av2_3)
                        if max_g2_y < max_g2:
                            max_g2 = max_g2_y
                    max_g = isqrt(max_g2)
                    if max_g >= 1:
                        coeff = r * v1_3 + 4 * s * av2_3
                        sg = max_g * (max_g + 1) * (2 * max_g + 1) // 6
                        ans = (ans + coeff % M * (sg % M)) % M
            s += 1
        r += 2

    print(ans)

if __name__ == "__main__":
    solve()
