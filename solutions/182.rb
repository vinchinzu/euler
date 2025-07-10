# solutions/182.rb

# Problem 182: RSA Encryption

p_val = 1009
q_val = 3643

phi = (p_val - 1) * (q_val - 1)
p_minus_1 = p_val - 1
q_minus_1 = q_val - 1

sum_of_e_values = 0

# Iterate e from 2 up to (phi - 1)
# The problem states 1 < e < phi
(2...phi).each do |e|
  # Condition: gcd(e, phi) == 1
  next unless e.gcd(phi) == 1

  # Condition: Number of unconcealed messages is minimum.
  # This occurs when gcd(e-1, p-1) == 1 and gcd(e-1, q-1) == 1.
  e_minus_1 = e - 1

  # Optimization: if e_minus_1 is 0 (e=1), skip. (covered by 2...phi)
  # if e_minus_1 is 1 (e=2), gcd(1, anything) is 1.
  # if p_minus_1 is 1 (p=2), gcd(anything, 1) is 1.
  # if q_minus_1 is 1 (q=2), gcd(anything, 1) is 1.
  # These are true for p_val=1009, q_val=3643.

  # Check gcd conditions for minimal unconcealed messages
  # For p=1009, q=3643, p-1 and q-1 are even.
  # gcd(e,phi)=1 implies e is odd, so e-1 is even.
  # Thus gcd(e-1, p-1) must be even, and gcd(e-1, q-1) must be even.
  # The minimal possible value for these gcds is 2.
  # So, the number of unconcealed messages (1+gcd_p)(1+gcd_q) is minimized when gcd_p=2 and gcd_q=2.
  next unless e_minus_1.gcd(p_minus_1) == 2
  next unless e_minus_1.gcd(q_minus_1) == 2

  sum_of_e_values += e
end

puts sum_of_e_values
