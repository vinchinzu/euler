"""Project Euler Problem 196: Prime triplets."""

from typing import Dict, List
import math

TARGET_ROWS = [5_678_027, 7_208_785]


def first_of_row(n: int) -> int:
    """First number in row n."""
    return (n * (n - 1)) // 2 + 1


def last_of_row(n: int) -> int:
    """Last number in row n."""
    return n * (n + 1) // 2


def simple_sieve(limit: int) -> List[int]:
    """Simple sieve for primes."""
    sieve = [True] * (limit + 1)
    sieve[0] = sieve[1] = False
    max_val = int(math.sqrt(limit))
    for p in range(2, max_val + 1):
        if not sieve[p]:
            continue
        step_start = p * p
        for multiple in range(step_start, limit + 1, p):
            sieve[multiple] = False
    primes = []
    for n in range(2, limit + 1):
        if sieve[n]:
            primes.append(n)
    return primes


MAX_ROW = max(TARGET_ROWS) + 1
MAX_VALUE = last_of_row(MAX_ROW)
BASE_PRIMES = simple_sieve(int(math.sqrt(MAX_VALUE)) + 1)


def primes_for_row(row: int, base_primes: List[int]) -> Dict[str, int | List[bool]]:
    """Get primes for a specific row."""
    if row < 1:
        return {"start": 0, "primes": []}

    start_value = first_of_row(row)
    finish_value = start_value + row - 1
    length = row

    sieve = [True] * length
    for p in base_primes:
        square = p * p
        if square > finish_value:
            break

        first_multiple = square
        if first_multiple < start_value:
            remainder = start_value % p
            if remainder == 0:
                first_multiple = start_value
            else:
                first_multiple = start_value + (p - remainder)

        idx = first_multiple - start_value
        while idx < length:
            sieve[idx] = False
            idx += p

    if start_value <= 1:
        index = 1 - start_value
        if 0 <= index < length:
            sieve[index] = False

    return {"start": start_value, "primes": sieve}


def prime_at(rows: Dict[int, Dict], row: int, pos: int) -> bool:
    """Check if position in row is prime."""
    if row < 1:
        return False

    data = rows.get(row)
    if not data:
        return False

    primes = data["primes"]
    if pos < 0 or pos >= len(primes):
        return False

    return primes[pos]


def prime_neighbor_count(rows: Dict[int, Dict], row: int, pos: int) -> int:
    """Count prime neighbors."""
    row_length = len(rows[row]["primes"]) if row in rows else None
    if not row_length or pos >= row_length or pos < 0:
        return 0

    count = 0

    # Same row left/right
    if pos > 0 and prime_at(rows, row, pos - 1):
        count += 1
    if pos < row_length - 1 and prime_at(rows, row, pos + 1):
        count += 1

    # Upper row
    if row > 1:
        upper_length = len(rows[row - 1]["primes"]) if row - 1 in rows else None
        if pos > 0 and upper_length and upper_length > pos - 1:
            if prime_at(rows, row - 1, pos - 1):
                count += 1
        if upper_length and upper_length > pos:
            if prime_at(rows, row - 1, pos):
                count += 1

    # Lower row
    if row + 1 in rows:
        lower_length = len(rows[row + 1]["primes"])
        if lower_length > pos:
            if prime_at(rows, row + 1, pos):
                count += 1
        if pos + 1 < lower_length:
            if prime_at(rows, row + 1, pos + 1):
                count += 1

    return count


def in_triplet(rows: Dict[int, Dict], row: int, pos: int) -> bool:
    """Check if prime is in triplet."""
    return prime_at(rows, row, pos) and prime_neighbor_count(rows, row, pos) >= 2


def sum_for_row(row: int, base_primes: List[int]) -> int:
    """Sum primes in triplets for a row."""
    rows: Dict[int, Dict] = {}
    for r in [row - 1, row, row + 1]:
        if r < 1:
            continue
        rows[r] = primes_for_row(r, base_primes)

    base_value = rows[row]["start"]
    primes = rows[row]["primes"]

    total = 0
    for pos, is_prime in enumerate(primes):
        if not is_prime:
            continue
        if in_triplet(rows, row, pos):
            total += base_value + pos
    return total


def main() -> int:
    """Main function."""
    answer = sum(sum_for_row(row, BASE_PRIMES) for row in TARGET_ROWS)
    return answer


if __name__ == "__main__":
    print(main())
