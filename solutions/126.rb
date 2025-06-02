# Problem 126: Cuboid Layers
#
# Problem Statement:
# The minimum number of cubes to cover every visible face on a cuboid measuring $3 	imes 2 	imes 1$ is twenty-two.
#
# If we then add a second layer to this solid it would require forty-six cubes to cover every visible face,
# the third layer would require seventy-eight cubes, and the fourth layer would require
# one-hundred and eighteen cubes to cover every visible face.
#
# However, the first layer on a cuboid measuring $5 	imes 1 	imes 1$ also requires twenty-two cubes;
# similarly the first layer on cuboids measuring $5 	imes 3 	imes 1$, $7 	imes 2 	imes 1$, and $11 	imes 1 	imes 1$
# all contain forty-six cubes.
#
# We shall define $C(n)$ to represent the number of cuboids that contain $n$ cubes in one of its layers.
# So $C(22) = 2$, $C(46) = 4$, $C(78) = 5$, and $C(118) = 8$.
#
# It turns out that $154$ is the least value of $n$ for which $C(n) = 10$.
#
# Find the least value of $n$ for which $C(n) = 1000$.
#
# Notes:
# The problem asks for the least n such that C(n)=1000, where C(n) is the count of (x,y,z,m) combinations
# yielding n cubes in layer m of an x*y*z cuboid. Formula: N_m = 2(xy+yz+zx) + 4(x+y+z)(m-1) + 4(m-1)(m-2).
# The script iterates m, z, y, x (x>=y>=z>=1) and calculates N_m.
# It uses a counts array for N_m values up to N_LIMIT=20000 (found to be sufficient).
# Pruning is applied in loops if the smallest possible N_m for current dimensions exceeds N_LIMIT.
# The first n with counts[n]=1000 is the answer.
# The solution found is 18522.

# Full Ruby script content from temp_problem_126.rb:

class Problem126Solver
  N_TARGET_COUNT = 1000 # The target count C(n) we are looking for.
  # N_LIMIT is the maximum value of n (number of cubes) we will consider.
  # Based on previous successful runs, 20,000 is a sufficient limit for C(n)=1000.
  N_LIMIT = 20_000 

  def solve
    # counts[k] will store C(k), the number of ways k cubes can form a layer.
    counts = Array.new(N_LIMIT + 1, 0)

    m = 1 # Layer number, starts from 1
    loop do # Loop for m (layer number)
      # Pre-calculate m-dependent terms for the formula:
      # N_m(x,y,z) = 2(xy+yz+zx) + 4(x+y+z)(m-1) + 4(m-1)(m-2)
      m_minus_1 = m - 1
      
      # Term 1: 4*(m-1)*(m-2)
      term_m_fixed_contribution = 4 * m_minus_1 * (m - 2) # Note: if m=1, this is 0. if m=2, this is 0.
      
      # Term 2: Factor for 4*(x+y+z)*(m-1)
      term_m_factor_for_sum_dims = 4 * m_minus_1

      # Pruning for m loop: Check if the smallest cuboid (1x1x1) in m layers exceeds N_LIMIT.
      # N_m(1,1,1) = 2*(1*1+1*1+1*1) + 4*(1+1+1)*(m-1) + 4*(m-1)*(m-2)
      #            = 2*3 + 4*3*m_minus_1 + term_m_fixed_contribution
      #            = 6 + 12*m_minus_1 + term_m_fixed_contribution
      cubes_for_1x1x1_m_layers = 6 + 12 * m_minus_1 + term_m_fixed_contribution
      if cubes_for_1x1x1_m_layers > N_LIMIT
        break # End m loop, as further layers will only use more cubes.
      end

      z = 1 # Smallest dimension of the cuboid
      loop do # Loop for z
        # Pruning for z loop: Check if (z x z x z) cuboid in m layers exceeds N_LIMIT.
        # N_m(z,z,z) = 2*(z*z+z*z+z*z) + 4*(z+z+z)*(m-1) + term_m_fixed_contribution
        #            = 6*z*z + 12*z*m_minus_1 + term_m_fixed_contribution
        cubes_for_zxzxz_m_layers = 6 * z * z + 12 * z * m_minus_1 + term_m_fixed_contribution
        if cubes_for_zxzxz_m_layers > N_LIMIT
          break # End z loop for current m.
        end

        y = z # Middle dimension (y >= z)
        loop do # Loop for y
          # Pruning for y loop: Check if (y x y x z) cuboid (i.e., x=y) in m layers exceeds N_LIMIT.
          # N_m(y,y,z) = 2*(y*y+y*z+z*y) + 4*(y+y+z)*(m-1) + term_m_fixed_contribution
          #            = 2*(y*y + 2*y*z) + 4*(2*y+z)*m_minus_1 + term_m_fixed_contribution
          cubes_for_yxyxz_m_layers = 2 * (y * y + 2 * y * z) + term_m_factor_for_sum_dims * (2 * y + z) + term_m_fixed_contribution
          if cubes_for_yxyxz_m_layers > N_LIMIT
            break # End y loop for current m, z.
          end

          x = y # Largest dimension (x >= y)
          loop do # Loop for x
            # Calculate N_m(x,y,z)
            surface_area_term = 2 * (x * y + y * z + z * x)
            sum_dims_term = term_m_factor_for_sum_dims * (x + y + z)
            num_cubes = surface_area_term + sum_dims_term + term_m_fixed_contribution
            
            if num_cubes > N_LIMIT
              break # End x loop for current m, z, y.
            else
              # If num_cubes is within limit, increment its count.
              counts[num_cubes] += 1
            end
            x += 1
          end # End x loop
          y += 1
        end # End y loop
        z += 1
      end # End z loop
      m += 1
    end # End m loop

    # After populating counts, find the least n such that counts[n] == N_TARGET_COUNT.
    # Iterate from n=1 up to N_LIMIT.
    (1..N_LIMIT).each do |n_val|
      if counts[n_val] == N_TARGET_COUNT
        puts n_val # Output the first n_val that meets the target count.
        return     # Terminate.
      end
    end

    # If the loop finishes and no such n_val is found (e.g., if N_LIMIT was too small).
    puts -1 # Indicate that the target count was not found for any n within N_LIMIT.
  end
end

# Create an instance of the solver and run the solve method.
solver = Problem126Solver.new
solver.solve

