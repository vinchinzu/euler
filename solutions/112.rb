class Euler112
  def initialize(target_percentage)
    @target_percentage = target_percentage # e.g., 99

    # Calculate the denominator for the check optimization.
    # If target_percentage/100 = num/den (simplified), current_number must be a multiple of den.
    common_divisor = @target_percentage.gcd(100)
    @check_denominator = 100 / common_divisor
  end

  def get_digits(n)
    # Efficiently get digits. For n=0, this would be [0].
    # Problem deals with positive integers. Smallest is 1.
    return [n] if n < 10 # Single digit numbers handled by is_increasing/is_decreasing
    
    digits = []
    while n > 0
      digits.unshift(n % 10)
      n /= 10
    end
    digits
  end

  def is_increasing?(n_digits)
    return true if n_digits.length <= 1 # Single digit numbers are considered increasing
    (0...(n_digits.length - 1)).each do |i|
      return false if n_digits[i] > n_digits[i+1]
    end
    true
  end

  def is_decreasing?(n_digits)
    return true if n_digits.length <= 1 # Single digit numbers are considered decreasing
    (0...(n_digits.length - 1)).each do |i|
      return false if n_digits[i] < n_digits[i+1]
    end
    true
  end

  def is_bouncy?(n)
    # Numbers less than 100 are never bouncy by problem's implication,
    # but definitions handle them:
    # e.g., 55 is both increasing and decreasing, so not bouncy.
    # 12 is increasing. 21 is decreasing.
    # This method correctly identifies them as non-bouncy.
    n_digits = get_digits(n) # Use the math-based get_digits
    
    increasing = is_increasing?(n_digits)
    decreasing = is_decreasing?(n_digits)
    
    # A number is bouncy if it is NOT increasing AND NOT decreasing.
    !increasing && !decreasing
  end

  def solve
    bouncy_count = 0
    current_number = 0 # Start from 0, loop will increment to 1 first

    loop do
      current_number += 1
      
      if is_bouncy?(current_number)
        bouncy_count += 1
      end

      # Optimized check:
      # Only evaluate the proportion condition when current_number is a multiple of @check_denominator.
      # For 99%, @check_denominator is 100.
      # For 50%, @check_denominator is 2.
      # For 90%, @check_denominator is 10.
      if current_number % @check_denominator == 0
        # Proportion check: bouncy_count / current_number == @target_percentage / 100
        # Use cross-multiplication to avoid floating point:
        # bouncy_count * 100 == @target_percentage * current_number
        if bouncy_count * 100 == @target_percentage * current_number
          # This condition implies that if @target_percentage > 0, bouncy_count must be > 0.
          # If @target_percentage == 0, this means bouncy_count must be 0.
          
          # Handle target_percentage = 0 explicitly if needed, though problem implies > 0.
          if @target_percentage == 0 && bouncy_count != 0
            next # Keep searching if we want 0% but have bouncy numbers
          end

          puts current_number
          break
        end
      end
    end
  end
end

if __FILE__ == $PROGRAM_NAME
  # Problem 112: Find the least number for which the proportion of bouncy numbers is exactly 99%.
  solver = Euler112.new(99)
  solver.solve
end
