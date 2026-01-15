"""Project Euler Problem 145.

Count reversible numbers below one billion: n + reverse(n) must contain only odd digits.
"""


def do_backward(psums: list, init_c: int, np: int) -> bool:
    """Check backward propagation."""
    c = init_c
    for i in range(np - 1, -1, -1):
        s = psums[i] + c
        if (s % 10) % 2 == 0:
            return False
        c = s // 10
    return True


def count_for_len(length: int) -> int:
    """Count reversible numbers of given length."""
    np = length // 2
    is_odd = length % 2 == 1

    def rec(layer: int, cin: int, psums: list) -> int:
        """Recursive function."""
        if layer == np:
            c = 0
            carry_back = cin
            if not is_odd:
                c = 1 if do_backward(psums, carry_back, np) else 0
            else:
                minm = 1 if np == 0 else 0
                for m in range(minm, 10):
                    sm = 2 * m + carry_back
                    if (sm % 10) % 2 == 1:
                        com = sm // 10
                        if do_backward(psums, com, np):
                            c += 1
            return c
        else:
            mina = 1 if layer == 0 else 0
            minb = 1 if layer == 0 else 0
            cc = 0
            for a in range(mina, 10):
                for b in range(minb, 10):
                    psu = a + b
                    sf = psu + cin
                    if (sf % 10) % 2 == 1:
                        cou = sf // 10
                        new_psums = psums + [psu]
                        cc += rec(layer + 1, cou, new_psums)
            return cc

    return rec(0, 0, [])


def main() -> int:
    """Main function."""
    total = 0
    for length in range(1, 10):
        total += count_for_len(length)
    return total


if __name__ == "__main__":
    print(main())
