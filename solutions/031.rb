# Coin sums
# Problem 31
# In England the currency is made up of pound, £, and pence, p, and there are eight coins in general circulation:

# 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
# It is possible to make £2 in the following way:

# 1£1 + 1 50p + 2 20p + 1 5p + 1 2p + 3 1p
# How many different ways can £2 be made using any number of coins?

# In England the currency is made up of pound, £, and pence, p, and there are eight coins in general circulation:
# 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
# It is possible to make £2 in the following way:
# 1£1 + 1 50p + 2 20p + 1 5p + 1 2p + 3 1p
# How many different ways can £2 be made using any number of coins?

def solve_coin_sums
  target_amount = 200
  coins = [1, 2, 5, 10, 20, 50, 100, 200]

  # Create an array to store the number of ways to make each amount.
  # ways[i] will be the number of ways to make amount i.
  # Initialize all to 0, except for ways[0].
  ways = Array.new(target_amount + 1, 0)

  # Base case: There is 1 way to make amount 0 (by using no coins).
  ways[0] = 1

  # Iterate over each type of coin.
  coins.each do |coin_value|
    # For each coin, update the ways array for amounts from coin_value up to target_amount.
    # We start 'amount' from 'coin_value' because we can't make a smaller amount
    # using this coin as the last coin.
    (coin_value..target_amount).each do |amount|
      # The number of ways to make 'amount' using the current set of coins
      # (up to and including 'coin_value') is increased by the number of ways
      # to make 'amount - coin_value' (using coins up to and including 'coin_value').
      # This is because if we can make 'amount - coin_value', we can add the current
      # 'coin_value' to form 'amount'.
      ways[amount] += ways[amount - coin_value]
    end
  end

  # The final answer is the number of ways to make the target_amount.
  ways[target_amount]
end

# To run the solution and print the result:
# result = solve_coin_sums
# puts "Number of ways to make 200p: #{result}"
# Expected output: 73682
