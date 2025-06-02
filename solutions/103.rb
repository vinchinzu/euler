#!/usr/bin/env ruby
# p103.rb  –  Optimum special–sum set for n = 7   (Project Euler 103)

require "set"

# ---------- helper: fast special–sum–set test ---------------------------
def special_sum_set?(arr)
  n        = arr.size
  sum_size = {}              # {subset_sum => subset_size}

  # generate every non‑empty subset via bitmask (2ⁿ – 1 subsets, n ≤ 7)
  1.upto((1 << n) - 1) do |mask|
    s  = 0
    sz = 0
    n.times do |i|
      if (mask & (1 << i)) != 0
        s  += arr[i]
        sz += 1
      end
    end

    # rule 1: sums must be unique
    return false if sum_size.key?(s)
    sum_size[s] = sz
  end

  # rule 2: the subset with more elements must have the larger sum
  sums = sum_size.to_a                      # [ [sum, size], … ]
  sums.combination(2) do |(sum1, size1), (sum2, size2)|
    if size1 > size2
      return false unless sum1 > sum2
    elsif size2 > size1
      return false unless sum2 > sum1
    end
  end

  true
end

# ---------- helper: prefix‑sum pruning ----------------------------------
def prefix_ok?(partial)
  # necessary (but not sufficient) condition for rule 2:
  #  Σ(first k) > Σ(last k – 1)   for every k ≥ 2
  (2..partial.size).each do |k|
    return false if partial.first(k).sum <= partial.last(k - 1).sum
  end
  true
end

# ---------- depth‑first search around the heuristic seed ----------------
def find_optimum(prev_set, delta = 4)
  mid      = prev_set.size / 2
  b        = prev_set[mid]                    # “middle” element
  seed     = ([b] + prev_set.map { |x| x + b }).sort
  n        = seed.size
  best_sum = Float::INFINITY
  best_set = nil

  # Pre‑compute all candidate values for each position (seed[i] ± delta)
  choices  = seed.map { |v| ((v - delta)..(v + delta)).to_a }

  dfs = lambda do |idx, last_val, partial|
    if idx == n
      # full candidate built – validate
      if special_sum_set?(partial)
        s = partial.sum
        if s < best_sum
          best_sum = s
          best_set = partial.dup
          puts "New best: #{best_set.inspect}  (sum = #{best_sum})"
        end
      end
      return
    end

    choices[idx].each do |cand|
      next if cand <= last_val               # must stay strictly increasing

      # quick bound: current sum + (remaining smallest choices) ≥ best_sum ?
      remaining_min = (idx + 1...n).map { |j| choices[j].first }.sum
      next if partial.sum + cand + remaining_min >= best_sum

      next unless prefix_ok?(partial + [cand])

      dfs.call(idx + 1, cand, partial + [cand])
    end
  end

  dfs.call(0, 0, [])
  best_set
end

# ---------- previous optimum for n = 6 ----------------------------------
prev_optimum_n6 = [11, 18, 19, 20, 22, 25]

# ---------- run the search ----------------------------------------------
optimum_n7 = find_optimum(prev_optimum_n6)

puts "\nOptimum special–sum set for n = 7:"
puts "  A = #{optimum_n7.inspect}"
puts "  S(A) = #{optimum_n7.sum}"
puts "  set string = #{optimum_n7.join}"

