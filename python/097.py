#!/usr/bin/env python3
"""
Large non-Mersenne prime (Problem 97)

Find the last ten digits of 28433 * 2^7830457 + 1.
"""

def main() -> None:
    """Calculate last ten digits."""
    # Use modular arithmetic to avoid large numbers
    modulus = 10**10
    base = 2
    exponent = 7830457
    
    # Calculate 2^7830457 mod 10^10 using fast exponentiation
    result = 1
    while exponent > 0:
        if exponent % 2 == 1:
            result = (result * base) % modulus
        base = (base * base) % modulus
        exponent //= 2
    
    # Calculate 28433 * result + 1 mod 10^10
    final_result = (28433 * result + 1) % modulus
    
    # Format with leading zeros if needed
    print(f"{final_result:010d}")


if __name__ == "__main__":
    main()
