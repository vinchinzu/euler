# solutions/181.rb

# Problem 181: Grouping Two Different Coloured Objects
# In how many ways can sixty black objects B and forty white objects W be thus grouped?

B_val = 60
W_val = 40

dp = Array.new(B_val + 1) { Array.new(W_val + 1, 0) }
dp[0][0] = 1

(0..B_val).each do |group_b|
  (0..W_val).each do |group_w|
    next if group_b == 0 && group_w == 0

    (group_b..B_val).each do |b|
      (group_w..W_val).each do |w|
        dp[b][w] += dp[b - group_b][w - group_w]
      end
    end
  end
end

puts dp[B_val][W_val]
