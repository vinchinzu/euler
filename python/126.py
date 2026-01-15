"""Project Euler Problem 126: Cuboid Layers."""

from typing import List

N_TARGET_COUNT = 1000  # The target count C(n) we are looking for.
# N_LIMIT is the maximum value of n (number of cubes) we will consider.
# Based on previous successful runs, 20,000 is a sufficient limit for C(n)=1000.
N_LIMIT = 20_000


def main() -> int:
    """Main function."""
    # counts[k] will store C(k), the number of ways k cubes can form a layer.
    counts: List[int] = [0] * (N_LIMIT + 1)

    m = 1  # Layer number, starts from 1
    while True:  # Loop for m (layer number)
        # Pre-calculate m-dependent terms for the formula:
        # N_m(x,y,z) = 2(xy+yz+zx) + 4(x+y+z)(m-1) + 4(m-1)(m-2)
        m_minus_1 = m - 1

        # Term 1: 4*(m-1)*(m-2)
        term_m_fixed_contribution = 4 * m_minus_1 * (m - 2)  # Note: if m=1, this is 0. if m=2, this is 0.

        # Term 2: Factor for 4*(x+y+z)*(m-1)
        term_m_factor_for_sum_dims = 4 * m_minus_1

        # Pruning for m loop: Check if the smallest cuboid (1x1x1) in m layers exceeds N_LIMIT.
        # N_m(1,1,1) = 2*(1*1+1*1+1*1) + 4*(1+1+1)*(m-1) + 4*(m-1)*(m-2)
        #            = 2*3 + 4*3*m_minus_1 + term_m_fixed_contribution
        #            = 6 + 12*m_minus_1 + term_m_fixed_contribution
        cubes_for_1x1x1_m_layers = 6 + 12 * m_minus_1 + term_m_fixed_contribution
        if cubes_for_1x1x1_m_layers > N_LIMIT:
            break  # End m loop, as further layers will only use more cubes.

        z = 1  # Smallest dimension of the cuboid
        while True:  # Loop for z
            # Pruning for z loop: Check if (z x z x z) cuboid in m layers exceeds N_LIMIT.
            # N_m(z,z,z) = 2*(z*z+z*z+z*z) + 4*(z+z+z)*(m-1) + term_m_fixed_contribution
            #            = 6*z*z + 12*z*m_minus_1 + term_m_fixed_contribution
            cubes_for_zxzxz_m_layers = 6 * z * z + 12 * z * m_minus_1 + term_m_fixed_contribution
            if cubes_for_zxzxz_m_layers > N_LIMIT:
                break  # End z loop for current m.

            y = z  # Middle dimension (y >= z)
            while True:  # Loop for y
                # Pruning for y loop: Check if (y x y x z) cuboid (i.e., x=y) in m layers exceeds N_LIMIT.
                # N_m(y,y,z) = 2*(y*y+y*z+z*y) + 4*(y+y+z)*(m-1) + term_m_fixed_contribution
                #            = 2*(y*y + 2*y*z) + 4*(2*y+z)*m_minus_1 + term_m_fixed_contribution
                cubes_for_yxyxz_m_layers = (2 * (y * y + 2 * y * z) +
                                            term_m_factor_for_sum_dims * (2 * y + z) +
                                            term_m_fixed_contribution)
                if cubes_for_yxyxz_m_layers > N_LIMIT:
                    break  # End y loop for current m, z.

                x = y  # Largest dimension (x >= y >= z)
                while True:  # Loop for x
                    # Calculate N_m(x,y,z) for current dimensions
                    # N_m(x,y,z) = 2(xy+yz+zx) + 4(x+y+z)(m-1) + 4(m-1)(m-2)
                    n_m = (2 * (x * y + y * z + z * x) +
                           term_m_factor_for_sum_dims * (x + y + z) +
                           term_m_fixed_contribution)

                    if n_m > N_LIMIT:
                        break  # End x loop for current m, z, y.

                    counts[n_m] += 1
                    x += 1

                y += 1

            z += 1

        m += 1

    # Find the first n with counts[n] == N_TARGET_COUNT
    for n in range(1, N_LIMIT + 1):
        if counts[n] == N_TARGET_COUNT:
            return n

    return 0


if __name__ == "__main__":
    print(main())
