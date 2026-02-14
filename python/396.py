"""Project Euler Problem 396 - Weak Goodstein Sequence.

Compute the last 9 digits of sum(G(n)) for 1 <= n <= 15, where G(n) is the
number of nonzero terms in the n-th weak Goodstein sequence.

The key insight is that for k-digit binary numbers, the Goodstein sequence length
telescopes into an iterated tower function: x -> x * 2^x applied repeatedly.

For modular computation mod 10^9 = 2^9 * 5^9:
- mod 512: always 0 after first iteration
- mod 5^k: track via CRT chain using ord(2, 5^k) = 4*5^{k-1}
"""
from __future__ import annotations


def solve_mod(B: int, M: int = 10**9) -> int:
    """Compute iter_B mod M where iter_0 = (B+1)*2^{B+1}, iter_{k+1} = iter_k * 2^{iter_k}."""
    M9 = 5**9

    # Track iter mod 5^k for k=1..9
    five_mods: dict[int, int] = {}
    for k in range(1, 10):
        Mk = 5**k
        five_mods[k] = ((B + 1) % Mk * pow(2, B + 1, Mk)) % Mk

    # Iterate B times: iter_{i+1} = iter_i * 2^{iter_i}
    for _ in range(B):
        new_five_mods: dict[int, int] = {}
        for k in range(1, 10):
            Mk = 5**k
            iter_mod_Mk = five_mods[k]

            # Compute iter mod 4*5^{k-1} for the exponent
            if k == 1:
                # ord(2, 5) = 4, iter mod 4 = 0 after first step
                exp_mod = 0
            else:
                # CRT(0 mod 4, five_mods[k-1] mod 5^{k-1})
                M_km1 = 5 ** (k - 1)
                r = five_mods[k - 1]
                inv4 = pow(4, -1, M_km1)
                t = (r * inv4) % M_km1
                exp_mod = 4 * t

            pow2 = pow(2, exp_mod, Mk)
            new_five_mods[k] = (iter_mod_Mk * pow2) % Mk

        five_mods = new_five_mods

    # CRT: combine mod 512 (=0) and mod 5^9
    mod_5_9 = five_mods[9]
    k_val = (mod_5_9 * pow(512, -1, M9)) % M9
    return (512 * k_val) % M


def main() -> None:
    M = 10**9
    total = 0

    # n=1..3: direct values
    for g in (1, 3, 5):
        total = (total + g) % M

    # n=4..7: 3-digit binary. G(n) = c0 * 2^c0 - 3
    for n in range(4, 8):
        bits = bin(n)[2:].zfill(3)
        d1, d0 = int(bits[1]), int(bits[2])
        F_val = (2**d1 - 1) * (3 + d0) + d0
        c0 = 3 + F_val
        c1_mod = (c0 % M * pow(2, c0, M)) % M
        gn = (c1_mod - 3) % M
        total = (total + gn) % M

    # n=8..15: 4-digit binary. G(n) = iter_B - 3
    for n in range(8, 16):
        bits = bin(n)[2:].zfill(4)
        d2, d1, d0 = int(bits[1]), int(bits[2]), int(bits[3])

        if d2 == 0:
            if d1 == 0:
                sub3 = d0
            else:
                sub3 = (2**d1 - 1) * (3 + d0) + d0
        else:
            F_val = (2**d1 - 1) * (3 + d0) + d0
            c0 = 3 + F_val
            c1 = c0 * 2**c0
            sub3 = c1 - 3

        B = 2 + sub3
        iter_B_mod = solve_mod(B, M)
        gn = (iter_B_mod - 3) % M
        total = (total + gn) % M

    print(total)


if __name__ == "__main__":
    main()
