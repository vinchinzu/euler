# Problem 148: Exploring Pascal's triangle
# Counts the number of entries not divisible by p in the first n_rows of Pascal's triangle.
# Entries C(n,k) are not divisible by p if digits of k in base p are <= digits of n in base p.
# Number of such k for a given n (digits n_i) is product(n_i + 1).
# This function sums product(n_i + 1) for n from 0 to n_rows - 1.
def problem_148
  n_rows = 10**9
  p = 7

  # Convert n_rows to base p digits
  # For n_rows = 10^9, p=7, this is [3,3,5,3,1,6,0,0,6,1,6]
  n_digits_base_p = n_rows.to_s(p).chars.map(&:to_i)
  num_digits = n_digits_base_p.length

  # sum_factor is sum_{j=0}^{p-1} (j+1) = p*(p+1)/2
  # For p=7, this is 7*8/2 = 28
  sum_factor = p * (p + 1) / 2

  total_count = 0
  # product_of_prefix_digits_plus_one represents (L_k+1)*(L_{k-1}+1)*... for the current prefix of N
  # where N is n_rows, and L_i are its digits in base p.
  product_of_prefix_digits_plus_one = 1

  n_digits_base_p.each_with_index do |current_digit_val_of_n_rows, index|
    remaining_len = num_digits - 1 - index

    # Iterate for current digit position `d` from 0 up to `current_digit_val_of_n_rows - 1`.
    # These are cases where the number formed is guaranteed to be less than n_rows.
    # For each such `d`:
    # The contribution to total_count is:
    # (product from higher-order digits of n_rows) * (d+1) * (sum_factor ^ remaining_len)
    # where sum_factor ^ remaining_len accounts for all possible combinations of lower-order digits.
    (0...current_digit_val_of_n_rows).each do |d|
      total_count += product_of_prefix_digits_plus_one * (d + 1) * (sum_factor ** remaining_len)
    end

    # Update product_of_prefix_digits_plus_one to include the term from the current digit of n_rows.
    # This will be used for the next iteration (next lower-order digit) or if this is the most significant part.
    product_of_prefix_digits_plus_one *= (current_digit_val_of_n_rows + 1)
  end

  # The loop iterates through all prefixes of N (n_rows).
  # `total_count` accumulates the sum for all numbers `n` from `0` to `n_rows - 1`.
  # The `product_of_prefix_digits_plus_one` at the end of the loop is for `n = n_rows` itself.
  # This term should NOT be added if we are counting for rows 0 to n_rows-1.
  # "first 10^9 rows" typically means rows 0, 1, ..., 10^9 - 1.

  total_count
end
