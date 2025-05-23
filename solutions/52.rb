# Permuted multiples
# Problem 52
# Find the smallest positive integer x such that 2x, 3x, 4x, 5x, and 6x contain the same digits.

def same_digits?(x, y)
  x.to_s.chars.sort == y.to_s.chars.sort
end

x = 1
loop do
  if (2..6).all? { |mult| same_digits?(x, x * mult) }
    puts x
    break
  end
  x += 1
end