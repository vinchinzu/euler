"""Project Euler Problem 104: Pandigital Fibonacci ends."""

import sys
import math

# Increase limit for integer string conversion
sys.set_int_max_str_digits(0)


def is_pandigital_1_9(s: str) -> bool:
    """Check if a string 's' is 1-9 pandigital.
    
    's' must be exactly 9 characters long and contain digits '1' through '9' once.
    """
    return len(s) == 9 and sorted(s) == list("123456789")


def main() -> int:
    """Find the first Fibonacci number with pandigital first and last 9 digits."""
    # Initialize Fibonacci sequence variables
    # For full Fibonacci numbers
    a = 1  # Represents F_1
    b = 1  # Represents F_2

    # For last 9 digits (modulo 10^9)
    MOD = 1_000_000_000
    a_last9 = 1  # Represents F_1 % MOD
    b_last9 = 1  # Represents F_2 % MOD

    # k is the index of the Fibonacci number.
    # We start with F_1, F_2 already defined, so the loop calculates F_3, F_4, ...
    k = 2
    result = None

    while result is None:
        k += 1  # k is now the index of F_k being computed

        # Calculate next Fibonacci term for the full number
        c_val = a + b
        a = b
        b = c_val

        # Calculate next Fibonacci term for the last 9 digits
        c_last9_val = (a_last9 + b_last9) % MOD
        a_last9 = b_last9
        b_last9 = c_last9_val

        # Check conditions for F_k (which is current 'b' and 'b_last9')

        # 1. Check pandigital property for the last nine digits
        # The number formed by the last 9 digits of F_k must be 1-9 pandigital.
        # For b_last9 to be 1-9 pandigital, its string representation must be 9 digits long.
        # This means b_last9 must be >= 100_000_000.
        # The is_pandigital_1_9 function's len(s) == 9 check handles this implicitly.
        # If b_last9 < 100_000_000, str(b_last9) will have < 9 digits.
        last_9_digits_str = str(b_last9)

        if is_pandigital_1_9(last_9_digits_str):
            # If last 9 digits are pandigital, then check the first 9 digits of the full F_k
            # Use logarithms to get first 9 digits without converting entire number to string
            # F_k = b, we want first 9 digits
            # log10(F_k) = log10(b)
            # Let f = log10(b) - floor(log10(b))  (fractional part)
            # Then first_digits = 10^(f + 8) gives us first 9 digits
            
            if b > 0:
                log10_b = math.log10(b)
                fractional_part = log10_b - math.floor(log10_b)
                first_9_digits = int(10 ** (fractional_part + 8))
                first_9_digits_str = str(first_9_digits)[:9]
                
                # Pad with zeros if needed (shouldn't happen, but be safe)
                if len(first_9_digits_str) < 9:
                    first_9_digits_str = first_9_digits_str.zfill(9)
                else:
                    first_9_digits_str = first_9_digits_str[:9]

                if is_pandigital_1_9(first_9_digits_str):
                    # Both conditions met
                    result = k
                    break

    return result


if __name__ == "__main__":
    print(main())
