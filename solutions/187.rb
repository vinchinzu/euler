# Solution for Project Euler Problem 187

# Generates primes up to a given limit using a Sieve of Eratosthenes.
# Returns an array of prime numbers.
def sieve(limit)
  return [] if limit < 2
  is_prime = Array.new(limit + 1, true)
  is_prime[0] = is_prime[1] = false

  (2..Math.sqrt(limit).to_i).each do |p|
    if is_prime[p]
      (p * p..limit).step(p) do |multiple|
        is_prime[multiple] = false
      end
    end
  end

  primes = []
  (2..limit).each do |p|
    primes << p if is_prime[p]
  end
  primes
end

LIMIT = 10**8

def solve
  # We need primes p1 and p2 such that p1 * p2 < LIMIT.
  # If p1 <= p2, then p1 <= sqrt(LIMIT).
  # The largest p2 can be is when p1 is the smallest prime (2), so p2 < LIMIT / 2.
  # Thus, the sieve should go up to LIMIT / 2.
  primes = sieve(LIMIT / 2)

  count = 0
  num_primes = primes.length

  (0...num_primes).each do |i|
    p1 = primes[i]

    # Optimization: if p1*p1 >= LIMIT, then any p1*p2 (where p2 >= p1) will also be >= LIMIT.
    # So, we can stop the outer loop.
    break if p1 * p1 >= LIMIT

    (i...num_primes).each do |j|
      p2 = primes[j]

      product = p1 * p2
      if product < LIMIT
        count += 1
      else
        # Since primes are sorted, any further p2 for this p1 will also exceed LIMIT.
        break
      end
    end
  end

  puts count
end

solve if __FILE__ == $PROGRAM_NAME
