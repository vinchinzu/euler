# Project Euler Problem 24: Lexicographic permutations
# Find the millionth lexicographic permutation of the digits 0-9

# Efficient factorial number system approach

def nth_lexicographic_permutation(digits, n)
  n -= 1 # zero-based index
  result = []
  available = digits.dup
  (digits.size - 1).downto(0) do |i|
    fact = (1..i).inject(1, :*)
    idx, n = n.divmod(fact)
    result << available.delete_at(idx)
  end
  result
end

puts nth_lexicographic_permutation((0..9).to_a, 1_000_000).join 