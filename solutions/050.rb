require 'prime'

limit = 1_000_000
primes = Prime.each(limit).to_a # Get all primes below one million

max_len = 0
prime_with_max_len = 0

(0...primes.length).each do |i| # Starting index of the consecutive prime sum
  current_sum = 0
  (i...primes.length).each do |j| # Ending index of the consecutive prime sum
    current_sum += primes[j]
    
    break if current_sum >= limit # Sum exceeds limit, no need to continue with this starting prime
    
    if Prime.prime?(current_sum)
      current_len = j - i + 1
      if current_len > max_len
        max_len = current_len
        prime_with_max_len = current_sum
      end
    end
  end
end

puts prime_with_max_len
