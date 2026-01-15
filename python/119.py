"""Project Euler Problem 119: Digit Power Sum Sequence.

The number 512 is interesting because it is equal to the sum of its digits raised to 
some power: 5 + 1 + 2 = 8, and 8³ = 512. Another example of a number with this 
property is 614656 = 28⁴.

We shall define aₙ to be the nth term of this sequence and insist that a number must 
contain at least two digits to have a sum.

We are given that a₂ = 512 and a₁₀ = 614656.

Find a₃₀.
"""

from typing import List


def sum_of_digits(n: int) -> int:
    """Calculate the sum of digits of a number."""
    return sum(int(d) for d in str(n))


def valid_power(s: int, p: int) -> bool:
    """Check if a power p = s^k has digit sum s (and p >= 10 for 2+ digits)."""
    return p >= 10 and sum_of_digits(p) == s


def generate_sequence() -> List[int]:
    """Generate the sequence by computing s^k for reasonable ranges of s and k.
    
    s ranges from 2 to 162 (max digit sum for 18-digit numbers)
    k ranges from 2 to 40 (powers grow very quickly)
    """
    sequence = []
    max_s = 162  # Maximum possible digit sum for reasonable numbers
    max_k = 40   # Sufficient to generate numbers up to ~10^18

    for s in range(2, max_s + 1):
        k = 2
        power = s * s  # Start with s^2

        while k <= max_k and power < 10**18:  # Prevent overflow and unreasonable sizes
            if valid_power(s, power):
                sequence.append(power)

            # Next power: power *= s, but check for potential overflow
            if power > (10**18) // s:
                break  # Next power would be too large

            power *= s
            k += 1

    # Sort and remove duplicates, then take first 30 terms
    return sorted(set(sequence))[:30]


def main() -> int:
    """Main execution."""
    sequence = generate_sequence()
    assert sequence[1] == 512, "Verification failed for a2"
    assert sequence[9] == 614656, "Verification failed for a10"
    return sequence[29]


if __name__ == "__main__":
    print(main())
