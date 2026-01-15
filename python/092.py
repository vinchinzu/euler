#!/usr/bin/env python3
"""
Square digit chains (Problem 92)

How many starting numbers below ten million will arrive at 89?
"""

MAX_SUM = 7 * 81  # Maximum sum of squares for 7-digit numbers


def sum_of_squares_of_digits(n: int) -> int:
    """Calculate sum of squares of digits."""
    total = 0
    while n > 0:
        digit = n % 10
        total += digit * digit
        n //= 10
    return total


def arrives_at_89(n: int) -> bool:
    """Check if number arrives at 89."""
    while n != 1 and n != 89:
        n = sum_of_squares_of_digits(n)
    return n == 89


def count_numbers_with_digits(num_digits: int, leads_to_89: list[bool]) -> int:
    """Count numbers with given digit count that lead to 89."""
    # dp[i][s] = number of i-digit numbers that produce sum s
    dp = [[0] * 568 for _ in range(num_digits + 1)]
    
    # Base case: 0 digits produces sum 0
    dp[0][0] = 1
    
    # Fill the DP table
    for i in range(num_digits):
        for sum_val in range(568):
            if dp[i][sum_val] == 0:
                continue
            
            # Try each digit 0-9
            for digit in range(10):
                # Skip leading zeros for multi-digit numbers
                if i == 0 and num_digits > 1 and digit == 0:
                    continue
                
                new_sum = sum_val + digit * digit
                if new_sum <= 567:
                    dp[i + 1][new_sum] += dp[i][sum_val]
    
    # Count numbers that lead to 89
    total = 0
    for sum_val in range(1, 568):
        if leads_to_89[sum_val]:
            total += dp[num_digits][sum_val]
    
    return total


def main() -> None:
    """Count numbers below 10 million that arrive at 89."""
    # Precompute which sums lead to 89
    leads_to_89 = [False] * (MAX_SUM + 1)
    for i in range(1, MAX_SUM + 1):
        leads_to_89[i] = arrives_at_89(i)
    
    count = 0
    
    # Count numbers with 1 to 7 digits
    for num_digits in range(1, 8):
        count += count_numbers_with_digits(num_digits, leads_to_89)
    
    print(count)


if __name__ == "__main__":
    main()
