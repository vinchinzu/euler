# -------------------------------------------------------------
#  Count all q that can be written as
#
#       q = 2 k0 k1 / gcd(k0 h1 + k1 h0 , 2 k0 k1)
#
# under     q ≤ 1·10⁸
# and with  (h / k)  strictly between 0 and 1/100.
#
#  The search space is explored with the Stern–Brocot tree
#  between 0/1 and 1/101.  Every internal node is visited
#  exactly once, so no memo‑table is needed.
# -------------------------------------------------------------

LIMIT_Q   = 100_000_000
HALF_Q    = LIMIT_Q / 2          # 5·10⁷ — because q = 2·k0·k1 / …
UPPER_FR  = 1                    # 0/1   left border
LOWER_FR  = 101                  # 1/101 right border ( < 1/100 )

# ---------- helper -----------------------------------------------------------

def add_and_check(num1, den1, num2, den2)
  num   = num1 * den2 + den1 * num2         # h₁k₂ + k₁h₂
  den   = 2 * den1 * den2                   # 2 k₁ k₂
  den / num.gcd(den) <= LIMIT_Q
end

# ---------- depth–first enumeration -----------------------------------------

def count_between(hl, kl, hr, kr)
  stack  = [[hl, kl, hr, kr]]
  total  = 0

  until stack.empty?
    hl, kl, hr, kr = stack.pop
    hm, km = hl + hr, kl + kr            # mediant fraction hₘ/kₘ

    #  Stop when 2·kl·km already exceeds the limit in any case.
    next if km > HALF_Q

    #  Keep only fractions strictly below 1/100  ⇒  100·h < k
    next unless 100 * hm < km

    #  Count the two “mid‑points” of the current interval …
    total += 1 if add_and_check(hl, kl, hm, km)   # (hl/kl + hm/km) / 2
    total += 1 if add_and_check(hm, km, hr, kr)   # (hm/km + hr/kr) / 2

    #  … and carry on with the two sub‑intervals.
    stack << [hl, kl, hm, km]
    stack << [hm, km, hr, kr]
  end

  total
end

# ---------- two independent cases -------------------------------------------

case1 = HALF_Q - 50                         # k₁ = 51 … 50 000 000
case2 = count_between(0, 1, 1, LOWER_FR)    # fractions in (0, 1/100)

puts "Case 1: #{case1}"
puts "Case 2: #{case2}"
puts "Total : #{case1 + case2}"

