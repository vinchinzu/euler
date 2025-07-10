# solutions/183.rb

# Problem 183: Maximum Product of Parts

def is_terminating_denominator_base(d_base)
  return true if d_base == 1
  d = d_base
  while d > 0 && d % 2 == 0 # d > 0 check is mostly for safety, d_base >= 1
    d /= 2
  end
  while d > 0 && d % 5 == 0
    d /= 5
  end
  d == 1
end

total_sum_D_N = 0
E_VAL = Math::E

(5..10000).each do |n|
  n_float = n.to_f
  k0_float = n_float / E_VAL

  k1 = k0_float.floor
  # For N >= 3, N/e >= 3/2.718 > 1, so k1 >= 1. No need for k1=0 check.
  k2 = k0_float.ceil

  # Compare P(N,k1) and P(N,k2) using logarithms: k * (log N - log k)
  # Ensure k > 0 for Math.log(k)
  val1_log_p = k1 * (Math.log(n_float) - Math.log(k1.to_f))

  val2_log_p = k2 * (Math.log(n_float) - Math.log(k2.to_f))

  k_opt = (val1_log_p >= val2_log_p) ? k1 : k2

  common_divisor = n.gcd(k_opt)
  denominator_base = k_opt / common_divisor

  if is_terminating_denominator_base(denominator_base)
    total_sum_D_N -= n
  else
    total_sum_D_N += n
  end
end

puts total_sum_D_N
