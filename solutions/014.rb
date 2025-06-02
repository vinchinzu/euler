# File: solutions/014.rb

def collatz_length(n, memo)
  return memo[n] if memo[n] != 0
  
  if n == 1
    memo[n] = 1
  elsif n.even?
    memo[n] = 1 + collatz_length(n / 2, memo)
  else
    memo[n] = 1 + collatz_length(3 * n + 1, memo)
  end
  memo[n]
end

def longest_collatz_under(limit)
  memo = Hash.new(0)
  max_length = 0
  starting_number = 1 # Default to 1, as it's the first number checked

  (1...limit).each do |i|
    length = collatz_length(i, memo)
    if length > max_length
      max_length = length
      starting_number = i
    end
  end

  starting_number
end

limit = 1_000_000
puts longest_collatz_under(limit)
