# Circular primes
# Problem 35
# The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.

# There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.

# How many circular primes are there below one million?

require 'prime'

LIMIT = 1_000_000
primes = Prime.each(LIMIT).to_a
prime_set = primes.to_set

# Only numbers with odd digits (except 2) can be circular primes
# (since any rotation with an even digit will eventually end with an even number)
def has_invalid_digits?(n)
  s = n.to_s
  s.length > 1 && s.chars.any? { |c| c == '0' || c == '2' || c == '4' || c == '5' || c == '6' || c == '8' }
end

def rotations(n)
  s = n.to_s
  (0...s.length).map { |i| (s.chars.rotate(i).join).to_i }
end

circular_primes = []
primes.each do |p|
  next if has_invalid_digits?(p)
  rots = rotations(p)
  if rots.all? { |r| prime_set.include?(r) }
    circular_primes << p
  end
end

# Add 2 and 5 manually (the only even/5-ending circular primes)
circular_primes += [2, 5] if 2 < LIMIT && 5 < LIMIT

puts circular_primes.uniq.count

#10.2 seconds
