# Self powers
# Problem 48
# The series, 11 + 22 + 33 + ... + 1010 = 10405071317.

# Find the last ten digits of the series, 11 + 22 + 33 + ... + 10001000.

answer = (1..1000).map {|x| x**x}.inject(:+).to_s
puts answer[answer.length-10..answer.length]	