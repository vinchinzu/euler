# Problem 106: Special subset sums: meta-testing

def count_equality_tests(n)
  comparisons_needed = 0
  elements = (0...n).to_a

  # Iterate for subset size k from 2 to n/2.
  # For k=1, S(B)=S(C) => b1=c1, impossible for disjoint sets with distinct elements.
  # Max k is n/2, because if 2*k > n, cannot form two disjoint subsets of size k.
  (2..(n / 2)).each do |k|
    elements.combination(k).each do |subset_b|
      remaining_elements = elements - subset_b
      
      # Need at least k elements remaining to form subset_c
      next if remaining_elements.length < k 

      remaining_elements.combination(k).each do |subset_c|
        # To avoid double counting (e.g. (B,C) and (C,B)) and ensure a canonical representation,
        # we can enforce an order, e.g., the first element of B is smaller than the first of C.
        # Ruby's combination method generates sorted subsets.
        next if subset_b.first >= subset_c.first

        # Now we have two disjoint subsets B and C of equal size k.
        # Check if a test S(B) = S(C) is necessary.
        # A test is necessary if NOT all b_i > c_i AND NOT all c_i > b_i.

        b_dominates_c = true
        (0...k).each do |i|
          if subset_b[i] <= subset_c[i]
            b_dominates_c = false
            break
          end
        end

        c_dominates_b = true
        (0...k).each do |i|
          if subset_c[i] <= subset_b[i]
            c_dominates_b = false
            break
          end
        end

        if !b_dominates_c && !c_dominates_b
          comparisons_needed += 1
        end
      end
    end
  end
  comparisons_needed
end

# Verification with example values (optional, can be commented out for final submission)
# puts "For n=4, comparisons needed: #{count_equality_tests(4)}" # Expected: 1
# puts "For n=7, comparisons needed: #{count_equality_tests(7)}" # Expected: 70
# Calculate and print the result for n=12
puts count_equality_tests(12)

