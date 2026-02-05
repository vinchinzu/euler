"""Project Euler Problem 309: Integer Ladders

Find the number of integer triplets (x, y, h) with 0 < x < y < 1000000
where two ladders of length x and y lean against opposite walls distance w apart,
and they intersect at height h (all integers).

The intersection height h is the harmonic mean of the heights H1 and H2 where
the ladders touch the walls: h = H1*H2 / (H1 + H2).

For h to be integer, H1*H2 must be divisible by H1+H2.

Algorithm (from Java):
1. Generate all Pythagorean triples with one leg < N
2. For each width w, collect all heights H where (w, H, hypotenuse) forms a Pythagorean triple
3. For each pair of heights (H1, H2) with same w, check if H1*H2 % (H1+H2) == 0
"""

def generate_pythagorean_triples(limit):
    """Generate all primitive Pythagorean triples with hypotenuse < limit.

    Uses the standard parameterization: a = k(m^2 - n^2), b = k(2mn), c = k(m^2 + n^2)
    where m > n > 0, gcd(m,n) = 1, m and n not both odd, k >= 1.

    Returns dict mapping leg -> list of other legs from same triple.
    """
    from math import gcd

    # Map from leg value to list of other leg values
    leg_to_heights = {}

    # Generate primitive triples (k=1) then scale
    m = 2
    while True:
        # Check if any triple with this m will fit
        if m * m + 1 >= limit:
            break

        for n in range(1, m):
            # Check conditions for primitive triple
            if gcd(m, n) != 1:
                continue
            if (m % 2) == (n % 2):  # Both odd or both even
                continue

            # Generate primitive triple
            a = m * m - n * n
            b = 2 * m * n
            c = m * m + n * n

            # Generate all multiples
            k = 1
            while k * c < limit:
                ka, kb, kc = k * a, k * b, k * c

                # Both (ka, kb) and (kb, ka) are valid orderings
                # Store as: leg -> [other_leg]
                if ka not in leg_to_heights:
                    leg_to_heights[ka] = []
                if kb not in leg_to_heights:
                    leg_to_heights[kb] = []

                leg_to_heights[ka].append(kb)
                leg_to_heights[kb].append(ka)

                k += 1

        m += 1

    return leg_to_heights

def solve():
    N = 1000000

    # Generate all Pythagorean triples
    # In each triple (a, b, c), a and b are the two legs (one is width, one is height)
    leg_to_heights = generate_pythagorean_triples(N)

    count = 0

    # For each width w, count pairs of heights
    for w in range(1, N):
        if w not in leg_to_heights:
            continue

        heights = leg_to_heights[w]

        # Count pairs (h1, h2) where h1 < h2 and h1*h2 % (h1+h2) == 0
        for i in range(len(heights)):
            for j in range(i + 1, len(heights)):
                h1 = heights[i]
                h2 = heights[j]

                # Check if harmonic mean is integer
                if (h1 * h2) % (h1 + h2) == 0:
                    count += 1

    return count

if __name__ == "__main__":
    print(solve())
