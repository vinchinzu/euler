require 'prime'

def is_left_truncatable?(n)
  return false if [2, 3, 5, 7].include?(n) # Single digit primes are not considered
  s = n.to_s
  return false unless Prime.prime?(n)
  (1...s.length).each do |i|
    return false unless Prime.prime?(s[i..-1].to_i)
  end
  true
end

def is_right_truncatable?(n)
  return false if [2, 3, 5, 7].include?(n) # Single digit primes are not considered
  s = n.to_s
  return false unless Prime.prime?(n)
  (1...s.length).each do |i|
    return false unless Prime.prime?(s[0...-i].to_i)
  end
  true
end

truncatable_primes = []
num = 11 # Start checking from the first two-digit number

while truncatable_primes.count < 11
  if Prime.prime?(num) && is_left_truncatable?(num) && is_right_truncatable?(num)
    truncatable_primes << num
  end
  # Optimization: After 11 (prime), check 13, 17, 19, 23...
  # Only need to check numbers ending in 3, 7, 9 for primes (except 2, 5)
  # And for truncatable, first digit also matters (can't be even or 5 for multi-digit)
  # A simpler increment strategy is fine for now given the small number of primes needed.
  num += 2 # Check odd numbers
  num += 2 if num % 5 == 0 # Skip numbers ending in 5 (e.g. if num becomes 15, next is 17)
end

puts truncatable_primes.sum
