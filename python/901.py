import mpmath

# Set high precision to ensure accuracy for 9 decimal places
mpmath.mp.dps = 50

def solve_drilling_problem():
    # The recurrence relation is d_{k+1} = exp(d_k - d_{k-1})
    # The sequence bifurcation occurs at a critical starting value d1.
    # Below this value, the sequence eventually decreases (physically invalid).
    # Above this value, the sequence grows super-exponentially.
    # The optimal cost is found exactly at this critical threshold.

    def check_growth(d1_val):
        """
        Returns True if the sequence defined by d1_val grows (valid high side),
        and False if it collapses/decreases (invalid low side).
        """
        d_prev = mpmath.mpf(0)
        d_curr = d1_val
        
        # Iterate to see behavior
        for _ in range(1000):
            # If difference is large, next term explodes -> Growth
            if d_curr - d_prev > 20: 
                return True
                
            d_next = mpmath.exp(d_curr - d_prev)
            
            # If sequence stops increasing, it collapses -> Low side
            if d_next <= d_curr:
                return False
            
            d_prev = d_curr
            d_curr = d_next
            
            # Safety break for very large numbers
            if d_curr > 1000:
                return True
                
        return True # Assumed growth if it sustained this long

    # Binary search for the critical d1
    low = mpmath.mpf('0.7')
    high = mpmath.mpf('0.8')
    
    for _ in range(100):
        mid = (low + high) / 2
        if check_growth(mid):
            high = mid
        else:
            low = mid
            
    optimal_d1 = high
    
    # Calculate Expected Cost at optimal d1
    # Formula: E = d1 + 1 + sum(exp(-d_k)) for k=1 to infinity
    
    cost = optimal_d1 + 1
    
    d_prev = mpmath.mpf(0)
    d_curr = optimal_d1
    
    # Add terms until convergence
    while True:
        term = mpmath.exp(-d_curr)
        cost += term
        
        if term < mpmath.mpf('1e-25'):
            break
            
        d_next = mpmath.exp(d_curr - d_prev)
        d_prev = d_curr
        d_curr = d_next
        
    return cost

result = solve_drilling_problem()
print(mpmath.nstr(result, 10))