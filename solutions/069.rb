# <p>Euler's totient function, $\phi(n)$ [sometimes called the phi function], is d
# efined as the number of positive integers not exceeding $n$ which are relatively
#  prime to $n$. For example, as $1$, $2$, $4$, $5$, $7$, and $8$, are all less th
# an or equal to nine and relatively prime to nine, $\phi(9)=6$.</p>
# <div class="center">
# <table class="grid center"><tr><td><b>$n$</b></td>
# <td><b>Relatively Prime</b></td>
# <td><b>$\phi(n)$</b></td>
# <td><b>$n/\phi(n)$</b></td>
# </tr><tr><td>2</td>
# <td>1</td>
# <td>1</td>
# <td>2</td>
# </tr><tr><td>3</td>
# <td>1,2</td>
# <td>2</td>
# <td>1.5</td>
# </tr><tr><td>4</td>
# <td>1,3</td>
# <td>2</td>
# <td>2</td>
# </tr><tr><td>5</td>
# <td>1,2,3,4</td>
# <td>4</td>
# <td>1.25</td>
# </tr><tr><td>6</td>
# <td>1,5</td>
# <td>2</td>
# <td>3</td>
# </tr><tr><td>7</td>
# <td>1,2,3,4,5,6</td>
# <td>6</td>
# <td>1.1666...</td>
# </tr><tr><td>8</td>
# <td>1,3,5,7</td>
# <td>4</td>
# <td>2</td>
# </tr><tr><td>9</td>
# <td>1,2,4,5,7,8</td>
# <td>6</td>
# <td>1.5</td>
# </tr><tr><td>10</td>
# <td>1,3,7,9</td>
# <td>4</td>
# <td>2.5</td>
# </tr></table></div>
# <p>It can be seen that $n = 6$ produces a maximum $n/\phi(n)$ for $n\leq 10$.</p
# >
# <p>Find the value of $n\leq 1\,000\,000$ for which $n/\phi(n)$ is a maximum.</p>

# Solution for Project Euler Problem 69

# The ratio n/φ(n) can be expressed as Π (p / (p-1)) for each distinct prime factor p of n.
# To maximize this value, we want to multiply by terms p/(p-1).
# These terms are always > 1.
# For smaller primes, the term p/(p-1) is larger:
# p=2: 2/(2-1) = 2
# p=3: 3/(3-1) = 1.5
# p=5: 5/(5-1) = 1.25
# So, to maximize n/φ(n), n should be the product of the smallest distinct primes
# such that n is still within the limit (<= 1,000,000).
# This type of number (product of initial primes) is called a primorial.

limit = 1_000_000
result_n = 1
current_prime_candidate = 2
primes_found = []

# Helper function to check for primality
def is_prime?(num, known_primes)
  return false if num <= 1
  # Check divisibility only by previously found primes up to sqrt(num)
  # This is a simple optimization for this specific problem's scale.
  sqrt_num = Math.sqrt(num)
  known_primes.each do |p|
    break if p > sqrt_num
    return false if num % p == 0
  end
  # If no known primes divide it, check subsequent numbers if necessary
  # (This part is more robust but less efficient than a full sieve if many primes are needed)
  # For this problem, sequential small primes are found quickly.
  # A more complete primality test for larger numbers would continue checking beyond known_primes.
  # However, since we are generating primes sequentially, if a prime isn't in known_primes,
  # it means it's larger than any prime in known_primes. If num is composite,
  # one of its prime factors must be <= sqrt_num. If that factor was not found in known_primes,
  # it means we haven't reached it yet in our prime generation, which is fine as we
  # build up known_primes.
  true # If not divisible by any smaller known prime, it's prime in this context.
end


loop do
  is_p = true
  # Check against already found primes
  sqrt_candidate = Math.sqrt(current_prime_candidate)
  primes_found.each do |p|
    break if p > sqrt_candidate
    if current_prime_candidate % p == 0
      is_p = false
      break
    end
  end

  if is_p
    primes_found << current_prime_candidate
    if result_n * current_prime_candidate <= limit
      result_n *= current_prime_candidate
    else
      # The next product would exceed the limit, so the current result_n is our answer.
      break
    end
  end
  current_prime_candidate += (current_prime_candidate == 2 ? 1 : 2) # Handle 2, then odds
end

puts "The value of n <= #{limit} for which n/φ(n) is a maximum is: #{result_n}"
