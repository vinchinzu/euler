# Powerful digit sum
# Problem 56
# Considering natural numbers of the form, a^b, where a, b < 100, what is the maximum digital sum?

def digit_sum(n)
  n.to_s.chars.map(&:to_i).sum
end

max_sum = 0
(1...100).each do |a|
  (1...100).each do |b|
    s = digit_sum(a**b)
    max_sum = s if s > max_sum
  end
end

puts max_sum