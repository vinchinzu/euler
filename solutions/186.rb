# Solution for Project Euler Problem 186

class LFG
  MOD = 1_000_000

  def initialize
    @s_values = [] # Stores S_1, S_2, ...
    @k = 0 # Represents the number of terms generated so far (index for next term)
  end

  def next_term
    @k += 1 # k is now the term number we are generating (1-indexed)
    new_sk = 0
    if @k <= 55
      new_sk = (100003 - 200003 * @k + 300007 * (@k**3)) % MOD
    else
      # S_k = (S_{k-24} + S_{k-55}) % MOD
      # Since @k is 1-indexed, S_{k-24} is at index @k-24-1 in a 0-indexed array
      # And S_{k-55} is at index @k-55-1
      s_k_minus_24 = @s_values[@k - 24 - 1]
      s_k_minus_55 = @s_values[@k - 55 - 1]
      new_sk = (s_k_minus_24 + s_k_minus_55) % MOD
    end
    @s_values << new_sk
    return new_sk
  end
end

class DSU
  attr_reader :parent, :set_size # For debugging/verification, PM network size

  def initialize(num_users)
    @num_users = num_users
    @parent = Array.new(num_users) { |i| i }
    @set_size = Array.new(num_users, 1)
  end

  def find(i)
    return i if @parent[i] == i
    @parent[i] = find(@parent[i]) # Path compression
    return @parent[i]
  end

  # Returns true if a union happened, false otherwise
  def union(i, j)
    root_i = find(i)
    root_j = find(j)

    return false if root_i == root_j # Already in the same set

    # Union by size: attach smaller tree under root of larger tree
    if @set_size[root_i] < @set_size[root_j]
      @parent[root_i] = root_j
      @set_size[root_j] += @set_size[root_i]
    else
      @parent[root_j] = root_i
      @set_size[root_i] += @set_size[root_j]
    end
    true
  end

  def get_size_of_set(i)
    root_i = find(i)
    @set_size[root_i]
  end
end

TOTAL_USERS = 1_000_000
PM_USER_ID = 524287
TARGET_PERCENTAGE = 0.99
# TARGET_NETWORK_SIZE = (TOTAL_USERS * TARGET_PERCENTAGE).to_i
# Ensure it's integer math, to_i is correct.
# Problem: "99% of users (including the PM as a user)"
# 1,000,000 users. 99% of 1,000,000 is 990,000.
TARGET_NETWORK_SIZE = 990_000


def solve
  lfg = LFG.new
  dsu = DSU.new(TOTAL_USERS)
  successful_calls = 0

  loop do
    caller_n = lfg.next_term
    called_n = lfg.next_term

    if caller_n == called_n
      # Misdial, ignore
      next
    end

    successful_calls += 1

    # Perform union. If they are already connected, this is still a successful call.
    # The problem asks for the number of successful calls *after which* the condition is met.
    dsu.union(caller_n, called_n)

    # Check PM's network size
    # The PM is user 524287. This is a 0-indexed ID if users are 0 to 999,999.
    # The problem says "users are numbered 0 to 999,999". So PM_USER_ID is correct.
    pm_network_size = dsu.get_size_of_set(PM_USER_ID)

    if pm_network_size >= TARGET_NETWORK_SIZE
      break
    end

    # Protective break for very long loops, though problem constraints should be met sooner.
    # if successful_calls > 2_500_000 # Based on typical Euler problem ranges
    #   puts "Loop limit exceeded, something might be wrong."
    #   break
    # end
  end

  puts successful_calls
end

solve
