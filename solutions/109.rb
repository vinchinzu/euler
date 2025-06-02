# frozen_string_literal: true

# Solution for Project Euler Problem 109

dart_options = []
current_id = 0 # Used to ensure canonical order for combinations

# Singles S1-S20
(1..20).each do |i|
  dart_options << { id: current_id, points: i, is_double: false, name: "S#{i}" }
  current_id += 1
end

# Doubles D1-D20
(1..20).each do |i|
  dart_options << { id: current_id, points: i * 2, is_double: true, name: "D#{i}" }
  current_id += 1
end

# Trebles T1-T20
(1..20).each do |i|
  dart_options << { id: current_id, points: i * 3, is_double: false, name: "T#{i}" }
  current_id += 1
end

# Outer Bull (S25)
dart_options << { id: current_id, points: 25, is_double: false, name: 'S25' }
current_id += 1

# Inner Bull (D25)
dart_options << { id: current_id, points: 50, is_double: true, name: 'D25' }

# dart_options is now sorted by id by construction.

# Filter for double options (for the checkout dart)
double_options = dart_options.filter { |opt| opt[:is_double] }

checkout_ways = 0

# 1. One-Dart Checkouts (d1_out)
# The dart must be a double.
double_options.each do |d1_out|
  if d1_out[:points] < 100
    checkout_ways += 1
    # puts "#{d1_out[:name]} - Score: #{d1_out[:points]}"
  end
end

# 2. Two-Dart Checkouts (t1, d2_out)
# t1 can be any dart_option. d2_out must be a double.
# Order matters: (S1, D20) is different from (D20, S1) if D20 is not the checkout dart.
# Here, d2_out IS the checkout dart, so t1 is the first dart. Sequence is fixed (t1, d2_out).
dart_options.each do |t1|
  double_options.each do |d2_out|
    if t1[:points] + d2_out[:points] < 100
      checkout_ways += 1
      # puts "#{t1[:name]}, #{d2_out[:name]} - Score: #{t1[:points] + d2_out[:points]}"
    end
  end
end

# 3. Three-Dart Checkouts (t1, t2, d3_out)
# t1, t2 can be any dart_option. d3_out must be a double.
# Based on the clarification: "the combination S1 T1 D1 is considered the same as T1 S1 D1",
# the order of the first two darts (t1, t2) does not matter.
# To achieve this, we iterate t1 and then t2 starting from t1's index.
# This ensures that if t1 has id X and t2 has id Y, we only consider pairs where X <= Y.
dart_options.each_with_index do |t1, i|
  # slice(i..-1) ensures that t2's id is >= t1's id because dart_options is sorted by id.
  dart_options.slice(i..-1).each do |t2| 
    double_options.each do |d3_out|
      if t1[:points] + t2[:points] + d3_out[:points] < 100
        checkout_ways += 1
        # puts "#{t1[:name]}, #{t2[:name]}, #{d3_out[:name]} - Score: #{t1[:points] + t2[:points] + d3_out[:points]}"
      end
    end
  end
end

puts checkout_ways
