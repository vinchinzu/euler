# File: solutions/032.rb
require 'set'

def solve_problem_032
  products = Set.new
  digits = (1..9).to_a

  digits.permutation.each do |p|
    # p is an array of 9 digits, e.g., [d1, d2, d3, d4, d5, d6, d7, d8, d9]

    # Split 1: a (1 digit), b (4 digits), product (4 digits)
    # a = p[0], b = p[1..4], c = p[5..8]
    # Last digits: a_ld = p[0], b_ld = p[4], c_ld = p[8]
    if (p[0] * p[4]) % 10 == p[8]
      a = p[0]
      b = p[1..4].map(&:to_s).join.to_i
      c = p[5..8].map(&:to_s).join.to_i
      products.add(c) if a * b == c
    end

    # Split 2: a (2 digits), b (3 digits), product (4 digits)
    # a = p[0..1], b = p[2..4], c = p[5..8]
    # Last digits: a_ld = p[1], b_ld = p[4], c_ld = p[8]
    if (p[1] * p[4]) % 10 == p[8]
      a = p[0..1].map(&:to_s).join.to_i
      b = p[2..4].map(&:to_s).join.to_i
      c = p[5..8].map(&:to_s).join.to_i # Structure of c is the same
      products.add(c) if a * b == c
    end
  end

  return products.inject(0, :+) # Using inject for compatibility as in original
end

puts solve_problem_032
