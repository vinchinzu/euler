# Solution for Project Euler Problem 188

$phi_cache = {}
$memo_tetrate = {}

# Euler's Totient Function phi(n)
def phi(n)
  return $phi_cache[n] if $phi_cache.key?(n)

  result = n
  p = 2
  temp_n = n
  while p * p <= temp_n
    if temp_n % p == 0
      result -= result / p
      while temp_n % p == 0
        temp_n /= p
      end
    end
    p += 1
  end

  if temp_n > 1 # Remaining temp_n is a prime factor
    result -= result / temp_n
  end

  $phi_cache[n] = result
  result
end

# Calculates (base^exp) % mod and whether base^exp >= mod (before modulo)
# exp is the actual exponent to be used (potentially adjusted with phi)
def power_and_compare(base, exp, mod)
  # Edge case: mod == 1. Result is always 0. Tower value is >= 1.
  return [0, true] if mod == 1

  # Calculate (base^exp) % mod
  result_mod_mod = base.pow(exp, mod)

  # Determine if base^exp >= mod (before modulo)
  # This flag is crucial for the layer above in tetration.
  was_value_ge_mod_flag = false

  if base == 0
    # 0^0 = 1. For exp > 0, 0^exp = 0.
    val = (exp == 0) ? 1 : 0
    was_value_ge_mod_flag = (val >= mod)
  elsif exp == 0
    # base^0 = 1 (for base != 0, handled by base == 0 case if base is 0)
    was_value_ge_mod_flag = (1 >= mod)
  elsif base == 1
    # 1^exp = 1
    was_value_ge_mod_flag = (1 >= mod)
  elsif base >= mod # If base itself is already >= mod (and exp > 0)
    was_value_ge_mod_flag = true
  else
    # General case: 1 < base < mod, exp > 0
    # Check if base^exp >= mod. This is equivalent to exp * log(base) >= log(mod)
    # or exp >= log(mod) / log(base).
    # Add a small epsilon for floating point comparisons to be safe,
    # ensuring that if base^exp is extremely close to mod (e.g. mod - tiny_fraction),
    # it's handled correctly.
    # However, standard problems usually avoid such fine precision issues.
    # Using direct comparison: exp >= log_base(mod)
    # Math.log(mod, base) might be slightly off.
    # A robust way for integers without large number arithmetic for base^exp:
    # If base^exp is needed, it's complex. But we only need comparison.
    # `exp >= Math.log(mod, base)` should be sufficient.
    # A common way to handle this is that if exp is already large enough, like >= phi(mod),
    # the flag is true. But exp here is the *already adjusted* exponent.
    # The flag is about the raw value of base^exp.
    if mod == 0 # Should not happen with positive modulus
        was_value_ge_mod_flag = true # Or handle as error
    else
        # Handle base > 1, mod > 1, exp > 0
        # Critical exponent: the smallest k such that base^k >= mod
        # This is ceil(log_base(mod)).
        # If base is large, log_base(mod) can be < 1. e.g. base=100, mod=10, log=0.5, ceil=1. 100^1 >= 10.
        # If exp is an integer, we need exp >= ceil(log_base(mod)).
        # log_val = Math.log(mod) / Math.log(base)
        # was_value_ge_mod_flag = (exp >= log_val) # This is a common heuristic

        # More robust check for integer exponent `exp`:
        # Iterate power up to `mod` or `exp` limit to find threshold.
        # This is too slow if `exp` is large.
        # The problem relies on `exp >= log_base(mod)` for `base^exp >= mod`.
        # Smallest k such that base^k >= mod.
        # If base = 2, mod = 8, log2(8) = 3. exp >= 3.
        # If base = 3, mod = 8, log3(8) = 1.89. exp >= 1.89. So exp=2 works.
        # threshold_exp = Math.log(mod, base)
        # was_value_ge_mod_flag = (exp >= threshold_exp)

        # Let's be more careful:
        # if exp is very large (e.g. > 60, as 2^60 > 10^18, much larger than any mod here),
        # then base^exp will surely be >= mod (assuming base >= 2).
        # Max mod is 10^8. log2(10^8) = 8 * log2(10) approx 8 * 3.32 = 26.5. So exp around 27-30 for base=2.
        # If exp is larger than roughly 30-60, flag is true for base >=2.
        if base >= 2 && exp >= 60 # Heuristic: 2^60 is huge
             was_value_ge_mod_flag = true
        else
            # Direct calculation for smaller exponents if worried about precision of logs
            # However, this path is tricky because `base^exp` can be huge.
            # The problem implies this flag is for Euler's theorem application.
            # The most reliable is `exp * Math.log(base) >= Math.log(mod)`.
            # We can use a small tolerance for floating point comparison.
            # E.g. exp * log(base) > log(mod) - epsilon
            if base > 1 # log(1) is 0, log(0) is -Infinity
                was_value_ge_mod_flag = (exp * Math.log(base) >= Math.log(mod) - 1e-9)
            else # base must be 0 or 1, handled above. This path for safety.
                was_value_ge_mod_flag = false # Should not be reached
            end
        end
    end
  end

  [result_mod_mod, was_value_ge_mod_flag]
end


# Calculates (base ^^ height) % mod
# Returns [result_mod_mod, was_value_ge_mod_flag]
# where flag indicates if (base ^^ height) >= mod (before modulo)
def tetrate_mod(base, height, mod)
  return $memo_tetrate[[base, height, mod]] if $memo_tetrate.key?([base, height, mod])

  # Base case for modulo: if mod is 1, result is 0, and original value is >= 1.
  if mod == 1
    return $memo_tetrate[[base, height, mod]] = [0, true]
  end

  # Base case for height: if height is 1, result is base % mod.
  # Flag is true if base >= mod.
  if height == 1
    # The power_and_compare function can determine this with exp=1
    # return $memo_tetrate[[base, height, mod]] = [base % mod, base >= mod]
    # Using power_and_compare for consistency, with exp=1
     return $memo_tetrate[[base, height, mod]] = power_and_compare(base, 1, mod)

  end

  # Recursive step for height > 1
  # We need to calculate base ^ (base ^^ (height - 1)) % mod
  # Let exp_tower = base ^^ (height - 1)
  # We need exp_tower % phi(mod) and whether exp_tower >= phi(mod)

  current_phi = phi(mod) # Calculate phi(mod) once

  # Recursively find properties of the exponent tower:
  # exp_val_mod_phi is (base ^^ (height - 1)) % current_phi
  # exp_is_ge_phi_flag is true if (base ^^ (height - 1)) >= current_phi
  exp_val_mod_phi, exp_is_ge_phi_flag = tetrate_mod(base, height - 1, current_phi)

  # Determine the actual exponent to use based on Euler's totient theorem extension
  # If exp_tower >= current_phi, exponent = (exp_tower % current_phi) + current_phi
  # Otherwise, exponent = exp_tower % current_phi
  actual_exponent = exp_is_ge_phi_flag ? (exp_val_mod_phi + current_phi) : exp_val_mod_phi

  # Now calculate base^actual_exponent % mod and whether base^actual_exponent >= mod
  result = power_and_compare(base, actual_exponent, mod)

  $memo_tetrate[[base, height, mod]] = result
  result
end

BASE_VAL = 1777
HEIGHT_VAL = 1855
MODULUS_VAL = 100_000_000 # 10^8

# Clear caches for multiple runs if any (not strictly needed for single script run)
$phi_cache = {}
$memo_tetrate = {}

# Specific handling for known problematic phi values for this problem if any
# phi(1) is not standardly defined but often taken as 1.
# Our phi function will correctly give phi(1)=1 if called (result=1, p*p > 1, temp_n=1).
# phi(2)=1.

final_result_val, _ = tetrate_mod(BASE_VAL, HEIGHT_VAL, MODULUS_VAL)
puts final_result_val
