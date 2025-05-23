# <p>Euler's totient function, $\phi(n)$ [sometimes called the phi function], is u
# sed to determine the number of positive numbers less than or equal to $n$ which
# are relatively prime to $n$. For example, as $1, 2, 4, 5, 7$, and $8$, are all l
# ess than nine and relatively prime to nine, $\phi(9)=6$.<br>The number $1$ is co
# nsidered to be relatively prime to every positive number, so $\phi(1)=1$. </p>
# <p>Interestingly, $\phi(87109)=79180$, and it can be seen that $87109$ is a perm
# utation of $79180$.</p>
# <p>Find the value of $n$, $1 \lt n \lt 10^7$, for which $\phi(n)$ is a permutati
# on of $n$ and the ratio $n/\phi(n)$ produces a minimum.</p>

# Solution for Project Euler Problem 70

# To minimize n/φ(n), n should have few prime factors, and those factors should be large.
# The formula for n = p1*p2 (p1, p2 are distinct primes) is n/φ(n) = (p1*p2) / ((p1-1)*(p2-1)).
# This value approaches 1 as p1 and p2 get larger.
# We are looking for n < 10^7.

N_LIMIT = 10_000_000

# Sieve of Eratosthenes to generate primes
# Primes up to sqrt(N_LIMIT) * 1.5 or a bit more.
# If p1 ~ sqrt(N_LIMIT) ~ 3162. We need p2 up to N_LIMIT/p1.
# Let's generate primes up to around 5000.
SIEVE_LIMIT = 5000 # Adjusted based on typical search range for p1, p2
primes = []
sieve = Array.new(SIEVE_LIMIT + 1, true)
sieve[0] = sieve[1] = false
(2..Math.sqrt(SIEVE_LIMIT)).each do |i|
  if sieve[i]
    (i*i).step(SIEVE_LIMIT, i) do |multiple|
      sieve[multiple] = false
    end
  end
end
(2..SIEVE_LIMIT).each { |i| primes << i if sieve[i] }

# Helper function to check if two numbers are permutations of each other
def are_permutations?(num1, num2)
  num1.to_s.chars.sort == num2.to_s.chars.sort
end

min_ratio = Float::INFINITY
result_n = 0

# Iterate through possible prime factors p1 and p2
# Optimized search range for p1:
# If p1 is too small, p1/(p1-1) is large.
# If p1 is too large, p2 is constrained and might not exist or be too small.
# We expect p1 and p2 to be somewhat close to sqrt(N_LIMIT) ~ 3162.
# Let's search p1 from around 2000.
primes.each_with_index do |p1, i|
  break if p1 * p1 >= N_LIMIT # Optimization: if p1*p1 > limit, p1*p2 (p2>p1) will also be > limit

  # Start p2 search from p1 to ensure p2 >= p1
  primes[i..-1].each do |p2|
    n = p1 * p2
    break if n >= N_LIMIT # n exceeds the overall limit for this p1

    next if p1 == p2 # p1 and p2 must be distinct for phi = (p1-1)(p2-1)

    phi_n = (p1 - 1) * (p2 - 1)

    if are_permutations?(n, phi_n)
      current_ratio = n.to_f / phi_n
      if current_ratio < min_ratio
        min_ratio = current_ratio
        result_n = n
        # puts "New min: n=#{n}, phi(n)=#{phi_n}, ratio=#{current_ratio}" # For debugging
      end
    end
  end
end

puts "The value of n for which φ(n) is a permutation of n and n/φ(n) is minimal: #{result_n}"
# puts "Minimum ratio: #{min_ratio}" # For verification
