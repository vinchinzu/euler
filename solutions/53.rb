# Combinatoric selections
# Problem 53
# How many values of nCr, for 1 ≤ n ≤ 100, are greater than one-million?

LIMIT = 1_000_000
count = 0

(1..100).each do |n|
  (1..n).each do |r|
    ncr = (1..n).inject(1) { |prod, k| prod * k } / ((1..r).inject(1) { |prod, k| prod * k } * (1..(n - r)).inject(1) { |prod, k| prod * k })
    count += 1 if ncr > LIMIT
  end
end

puts count
