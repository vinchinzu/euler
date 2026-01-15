#euler 970

#Starting at zero, a kangaroo hops along the real number line in the positive direction. Each successive hop takes the kangaroo forward a uniformly random distance between $0$ and $1$. Let $H(n)$ be the expected number of hops needed for the kangaroo to pass $n$ on the real line.
#For example, $H(2) \approx 4.67077427047$. The first eight digits after the decimal point that are different from six are $70774270$.
#Similarly, $H(3) \approx 6.6665656395558899$. Here the first eight digits after the decimal point that are different from six are $55395558$.
#Find $H(10^6)$ and give as your answer the first eight digits after the decimal point that are different from six.

#wrong: 54308979
# 1211473
# 39354478
# 08828853
# 42814193

#to check:
# 44754029

from mpmath import *
mp.dps = 200
p = mpf('-2.0888430156130438559570867167749475005456937410367296732391125442446071101031945298324490132996634381766490181479377100162339208682314929202258107038027376301048367882230730215914503570137699548248339')
q = mpf('7.4614892856542545569061166121864153345090949932022092409344113914118766543223746778294494171207940558452136935903335803201911256508001005967490428764038980503480922635851389349404928619906508039054285')
delta3 = mpf('-0.00010102711077676251884819755863314716076642236623138297850426815093006555603466494510338730930934034683217737967386607171615477456118493894066216135527389715216919350143857687760336766859094268393991832')
delta4 = mpf('-0.000062176633971229441187418293036744771896518228013654963781023451740713907527450929782978410269778700445477909135691906209628377365978063318361995951544931621358947313068763205190499786160127774857080954')
delta5 = mpf('-0.0000045980442548086476392947338195980635640855819083233408734685327056625450325913378846227416162287079456897743915060373897012704905348795621971428991089326197813541766245548232446370934603973899316975401')
delta10 = mpf('-0.00000000019034786605250357560690019800980584512192428247457606659281297830507684084320826898337812129463981641303799667589845367875189344193907252132835413544254848211668334868591531252845162282497705574187773')
left3 = delta3 / exp( p *3 )
left4 = delta4 / exp( p *4 )
left5 = delta5 / exp( p *5 )
left10 = delta10 / exp( p *10 )
# now fit with n=5 and n=10
c5 = cos( q *5 )
s5 = sin( q *5 )
c10 = cos( q *10 )
s10 = sin( q *10 )
det = c5 * s10 - c10 * s5
A = (left5 * s10 - left10 * s5 ) / det
B = (left10 * c5 - left5 * c10 ) / det
n = mpf('1000000')
c = cos ( q * n )
s = sin ( q * n )
left = A * c + B * s
log10_delta = log10 (abs (left)) + p * n / log (10)
frac_log = frac ( log10_delta )
m = power (10, frac_log)
g = 6 + mpf('2') /3 - m
floor_g = floor ( g )
frac_g = g - floor_g
# to get the decimal digits, multiply by 10^50, floor, the string
digits = floor ( frac_g * power(10,50) )
digits_str = str(digits).zfill(50)
non_six_digits = ''
for digit in digits_str:
    if digit != '6':
        non_six_digits += digit
    if len(non_six_digits) == 8:
        break
print(non_six_digits)


import mpmath
mpmath.mp.dps = 300
p = mpmath.mpf('-2.08884301561304385595708671677494750054569374103672967323911254424460711010319452983244901329966343817664901814793771001623392086823149292022581070380273763010483678822307302159145035701376995482483391205983168777323093382252776399813389747183762513752673794948377023096466125985681944239407709953077')
q = mpmath.mpf('7.46148928565425455690611661218641533450909499320220924093441139141187665432237467782944941712079405584521369359033358032019112565080010059674904287640389805034809226358513893494049286199065080390542848635755727788293840828139926193150847650657856720825692770788519871011200968522744547014124594926279')
p2 = mpmath.mpf('-2.66406814242907101707075103880089577555067038780500205566116120840713409167564619144932951403928771646212592015197736647735238287357874431982785415300754882670832604442580778149442765929316041577192203342050067016311954054101170912860084759039895324548777447567383585639028375338778555774414456278608')
q2 = mpmath.mpf('13.8790560027468094253535574218356227903456422011233314180467701206497818769410200088459245699701042341468849271285947187989283837348618087018465551687422142071710595457008326223422300072425558456647601833411679919182462302580659036188692819004012103306447797233967557023992986497337582820566266777619')
delta3 = mpmath.mpf('-0.00010102711077676251884819755863314716076642236623138297850426815093006555603466494510338730930934034683217737967386607171615477456118493894066216135527389715216919350143857687760336766859094268393991832')
delta4 = mpmath.mpf('-0.000062176633971229441187418293036744771896518228013654963781023451740713907527450929782978410269778700445477909135691906209628377365978063318361995951544931621358947313068763205190499786160127774857080954')
delta5 = mpmath.mpf('-0.0000045980442548086476392947338195980635640855819083233408734685327056625450325913378846227416162287079456897743915060373897012704905348795621971428991089326197813541766245548232446370934603973899316975401')
delta10 = mpmath.mpf('-0.00000000019034786605250357560690019800980584512192428247457606659281297830507684084320826898337812129463981641303799667589845367875189344193907252132835413544254848211668334868591531252845162282497705574187773')
left3 = delta3 / mpmath.exp( p *3 )
left4 = delta4 / mpmath.exp( p *4 )
left5 = delta5 / mpmath.exp( p *5 )
left10 = delta10 / mpmath.exp( p *10 )
# now fit with two pairs
n_list = [3,4,5,10]
M = mpmath.matrix(4,4)
for i in range(4):
    ni = mpmath.mpf(n_list[i])
    c1 = mpmath.cos(q * ni)
    s1 = mpmath.sin(q * ni)
    exp2 = mpmath.exp( (p2 - p) * ni )
    c2 = mpmath.cos(q2 * ni)
    s2 = mpmath.sin(q2 * ni)
    M[i,0] = c1
    M[i,1] = s1
    M[i,2] = exp2 * c2
    M[i,3] = exp2 * s2
left_vec = mpmath.matrix([left3, left4, left5, left10])
x = M ** -1 * left_vec
A = x[0,0]
B = x[1,0]
A2 = x[2,0]
B2 = x[3,0]
n = mpmath.mpf('1000000')
c = mpmath.cos ( q * n )
s = mpmath.sin ( q * n )
c2 = mpmath.cos ( q2 * n )
s2 = mpmath.sin ( q2 * n )
exp2 = mpmath.exp( (p2 - p) * n )
left = A * c + B * s + exp2 * ( A2 * c2 + B2 * s2 )
log10_delta = mpmath.log10 (mpmath.fabs (left)) + p * n / mpmath.log (10)
frac_log = mpmath.frac ( log10_delta )
m = mpmath.power (10, frac_log)
g = 6 + mpmath.mpf('2') /3 - m
floor_g = mpmath.floor ( g )
frac_g = g - floor_g
# to get the decimal digits, multiply by 10^50, floor, the string
digits = mpmath.floor ( frac_g * mpmath.power(10,50) )
digits_str = str(digits).zfill(50)
non_six_digits = ''
for digit in digits_str:
    if digit != '6':
        non_six_digits += digit
    if len(non_six_digits) == 8:
        break
print(non_six_digits)



#also wrong

from mpmath import *
mp.dps = 300
def compute_H(n):
  n = mpf(n)
  H = mpf(0)
  for k in range(0, 10000):
    s = mpf(0)
    fn = int(floor(n))
    for j in range(0, fn+1):
      sign = mpf( (-1)**j )
      binom = binomial(k, j)
      base = n - j
      term = sign * binom * power(base, k) / factorial(k)
      s += term
    H += s
    if k > 2*int(n) and abs(s) < power(10, -(mp.dps - 20)):
      break
  return H
f = lambda l: l + exp(-l) - mpf(1)
starting1 = mpf('-2.0888430156130438559570867167749475005456937410367296732391125442446071101031945298324490132996634381766490181479377100162339208682314929202258107038027376301048367882230730215914503570137699548248339') + mpf('7.4614892856542545569061166121864153345090949932022092409344113914118766543223746778294494171207940558452136935903335803201911256508001005967490428764038980503480922635851389349404928619906508039054285') * 1j
lambda1 = findroot(f, starting1, tol=1e-290)
p = re(lambda1)
q = im(lambda1)
starting2 = mpf('-2.6640681424290710170707510388008957755506703878050020556611612084071340916756461914493295140392877164621259201519773664773523828735787443198278541530075488267083260444258077814944276592931604157719220') + mpf('13.8790560027468094253535574218356227903456422011233314180467701206497818769410200088459245699701042341468849271285947187989283837348618087018465551687422142071710595457008326223422300072425558456647602') * 1j
lambda2 = findroot(f, starting2, tol=1e-290)
p2 = re(lambda2)
q2 = im(lambda2)
n_list = [3,4,5,10]
delta_list = []
for ni in n_list:
  H = compute_H(ni)
  delta = H - 2*mpf(ni) - mpf(2)/3
  delta_list.append(delta)
M = matrix(4,4)
for i in range(4):
  ni = mpf(n_list[i])
  c1 = cos(q * ni)
  s1 = sin(q * ni)
  exp2 = exp( (p2 - p) * ni )
  c2 = cos(q2 * ni)
  s2 = sin(q2 * ni)
  M[i,0] = c1
  M[i,1] = s1
  M[i,2] = exp2 * c2
  M[i,3] = exp2 * s2
left_vec = matrix([[delta_list[0]], [delta_list[1]], [delta_list[2]], [delta_list[3]]])
x = M ** -1 * left_vec
A = x[0,0]
B = x[1,0]
A2 = x[2,0]
B2 = x[3,0]
n = mpf('1000000')
c = cos ( q * n )
s = sin ( q * n )
c2 = cos ( q2 * n )
s2 = sin ( q2 * n )
exp2 = exp( (p2 - p) * n )
left = A * c + B * s + exp2 * ( A2 * c2 + B2 * s2 )
log10_delta = log10 (fabs (left)) + p * n / ln(10)
frac_log = frac ( log10_delta )
m = power (10, frac_log)
g = mpf(6) + mpf('2') /3 - m
floor_g = floor ( g )
frac_g = g - floor_g
digits = floor ( frac_g * power(10,50) )
digits_str = str(digits).zfill(50)
non_six_digits = ''
for digit in digits_str:
    if digit != '6':
        non_six_digits += digit
    if len(non_six_digits) == 8:
        break
print(non_six_digits)


# Reviewing puzzle feedback
# The human points out that 08828853 is incorrect, and the assistant's suggestion of 54308979 is also wrong since it was already tried.

# This seems to be Project Euler problem 970, involving the expected number of hops H(10^6) for a kangaroo starting at 0, with each hop uniform in [0,1], until the position exceeds 10^6.
# Recalling asymptotic formula
# For the expected number of uniform [0,1] random variables needed to sum over n, it's approximately 2n plus a constant, since the mean hop is 1/2.

# Looking at H(2) ≈ 4.67077, which is slightly above 2*2 + 2/3 ≈ 4.666.
# Clarifying decimal digits
# In H(3) ≈ 6.6665656395558899, the first eight digits after the decimal that differ from six are collected by skipping the 6's.
# Defining expected hops function
# H(n) represents the expected hops from position 0 to exceed n, using uniform [0,1] increments.

# Let's define f(s) as the expected additional hops from current position s ≤ n to exceed n.
# Exploring integral equation
# The function h(x) satisfies h(x) = 1 + ∫_0^{min(1,x)} h(x - y) dy for x > 0.

# The characteristic equation for the homogeneous part is lambda = 1 - e^{-lambda}, or equivalently lambda + e^{-lambda} = 1.
# Finding characteristic roots
# The equation lambda = 1 - e^{-lambda} is transcendental, so roots are found numerically.
# Analyzing asymptotic expansion
# For large n, H(n) ≈ 2n + 2/3 + sum of terms A_k e^{lambda_k n}, where lambda_k are complex roots with negative real parts.
# Adjusting asymptotic sign
# For n=2, the actual H(2) is larger than 2n + 2/3, suggesting the small delta term might be positive rather than negative in the approximation.
# Evaluating term dominance
# The dominant correction comes from the complex root pair with the least negative real part, around -2.088, as its exponential decay is slowest.
# Planning numerical fitting
# To accurately fit coefficients for the first root pair, we'll use multiple small n values and include the next term's contribution.
# Computing small n values
# To improve fitting, high-precision calculations of H(n) for small n like 3, 4, 5, and 10 are essential.
# Computing small n values
# For small integer n, we can compute H(n) iteratively by solving the integral equation segment by segment across intervals.

# In each interval (k, k+1], the function h(x) can be solved assuming it's known for lower intervals.
# Setting up initial condition
# For 0 < x < 1, h(x) = 1 + ∫_0^x h(y) dy, which leads to the differential equation h'(x) = h(x) after differentiation.

# At x=0, h(0) = 1, but the hop from 0 always lands in (0,1), so it's 1 + average of h(u) over u in [0,1].
# Deriving particular solution
# Assuming a linear particular solution h(x) = a x + b for the inhomogeneous equation, we find a = 2 to match the mean increment.

# The full solution combines this particular solution with the homogeneous part, including the constant c and exponential terms from non-zero roots.
# Confirming renewal asymptotic
# Renewal theory confirms E[N(t)] ≈ 2t + 2/3 plus oscillating terms from the non-zero roots.
# Analyzing decimal behavior
# Subtracting a small epsilon from 0.666... results in a pattern like 0.666...59..., where the last 6 becomes 5 followed by 9's.

# The long run of 6's in the decimal expansion occurs when H(n) - 2n is very close to 2/3, causing many matching digits before deviation.
# Interpreting code logic
# In the fitting code, delta for small n like 3 is negative, so H(n) = 2n + 2/3 - delta adds a positive correction, matching the example where H(3) > 6.666.
# Examining decimal addition
# If H(n) - 2n = 2/3 + small positive delta, adding to the infinite 6's would typically carry over, turning trailing 6's into 7's or higher digits.

# Adding a small delta like 10^{-6} to 0.666666 gives 0.666667, changing the sixth digit to 7.
# Understanding mantissa effect
# The code computes m as the mantissa of |delta|, in [1,10), to determine how the small deviation affects the decimal digits after the run of 6's.
# Decoding g calculation
# The code sets g = 6 + 2/3 - m, which is 6.666... minus m where m is between 1 and 10.

# Subtracting m from 6.666... effectively models the borrow in the decimal expansion when subtracting the scaled delta from the repeating 6's.
# Simplifying subtraction trick
# Subtracting m * 10^{-l} from 0.666... involves borrowing across the repeating 6's starting from the lth digit.

# Let's test the code's g calculation with a small example to see if it matches the actual decimal.
# Testing subtraction example
# For delta = 3.141 * 10^{-3}, 2/3 - delta ≈ 0.663525, with decimal digits 663525.
# Modeling borrow propagation
# When subtracting m from 0.666..., since m > 0.666..., it requires borrowing from the units place, turning the decimal into a complement-like pattern.
# Examining positive deviation case
# For positive delta, H(n) - 2n = 2/3 + |delta|, adding to the repeating 6's without initial borrow.

# The addition at position l adds the digits of m to the 6's, potentially causing carries if 6 + digit >=10.
# Testing code example
# Using delta = 0.000345, m ≈ 3.45, g = 6.666... - 3.45 ≈ 3.21666, fractional part 0.21666.
# Analyzing no-borrow case
# In the example with delta = 0.000345, subtracting from 0.666666 gives 0.666321, where digits 3,2,1 are exactly 6 minus the digits of 3.45 (3,4,5).

# If some digit d >6, then borrow from previous.
# Testing borrow example
# For delta = 7.89 * 10^{-4}, 0.666666 - 0.000789 = 0.665877, with digits 665877 after two 6's.
# Refining borrow model
# In the borrow case, the tail calculation becomes 10.666... - m, but the example shows 10.666 - 7.89 = 2.776, with digits 776, yet actual tail is 877.

# The tail digits appear to be 9 minus the digits of m, adjusted for the borrow propagation: for m=7.89, digits 7,8,9, 9-7=2, but actual 8,7,7, suggesting a different adjustment like 9 - (d -1) or something for the first digit after borrow.

# The 5th digit: 6 -8 -1 (from 6th borrow) = -3, can't, so borrow from 4th, the 5th becomes 16 -8 -1 =7, borrow 1 from 4th.
# Tracing borrow propagation
# Position 6: 6 -9 requires borrow, so 16 -9 =7, and position 5 borrows 1.

# Position 5: 6 -8 -1 = -3, requires borrow, so 16 -8 -1 =7, position 4 borrows 1.

# Position 4: 6 -7 -1 = -2, requires borrow, so 16 -7 -1 =8, position 3 borrows 1.

# The pattern shows each borrowed position gets 16 - d -1 =15 - d for the ones that have borrow from lower, but for the lowest, 16 - d.

# For position 6 (lowest): 16 -9 =7
# Generalizing borrow digits
# The lowest borrowed position uses 16 - d, while subsequent positions use 15 - d, matching the example: 16-9=7, 15-8=7, 15-7=8.
# Matching code to example
# For n=3, the non-6 digits 55395558 start with 5, followed by various digits, suggesting the code's frac(g) produces a decimal whose non-6 digits match this sequence.
# Clarifying fractional part trick
# Computing frac(6 + 2/3 - m) gives the same result regardless of the integer 6, as long as it's large enough to handle negative values.

# frac(2/3 - m) effectively computes the tail digits after the borrow in the subtraction from repeating 6's.
# Reversing digit complements
# The borrow digits appear in reverse order of the mantissa digits, with the lowest (rightmost) using 16 - d and others 15 - d.
# Ordering tail digits
# The tail digits are ordered from left to right as 15 - d0 for the highest digit d0 of m, then 15 - d1, and 16 - d_last for the lowest non-zero digit.

# For the example with m=7.89, d0=7, d1=8, d2=9, the tail is 15-7=8, 15-8=7, 16-9=7, matching 877.
# Observing digit reversal
# The tail digits 877 for the example correspond to the complements applied in the order of the mantissa digits from highest to lowest, with adjustment for the lowest one.

# 7 =15 -8
# Deriving explicit formula
# H(n) equals the sum from k=0 to infinity of P(S_k ≤ n), where S_k is the sum of k uniform [0,1] random variables.
# Computing Irwin-Hall cdf
# The cdf P(S_k ≤ x) for Irwin-Hall distribution is given by 1/k! times sum_{j=0}^{floor(x)} (-1)^j * binom(k,j) * (x - j)^k.

# For n=10, floor(x)=10, so the sum has only 11 terms for every k, making it computationally feasible even for large k.
# Assessing precision needs
# For large k, terms like 10^k / k! grow initially but the alternating sum cancels to a small probability.

# The cancellation in the sum is such that the result is small, P ~ exp(- (k/2 -10)^2 / (k/12)) / sqrt(2pi var) by normal, since mean k/2, var k/12, for k large, P(S_k <=10) ≈ 0 when 10 << k/2, i.e. k >>20
# Estimating sum range
# To achieve precision of 10^{-200}, we need to sum up to k around 400, where the tail probability drops below that level.
# Evaluating term sizes
# For k=400, the j=0 term 10^k / k! has log10 approximately -169, which is well within 200 decimal places of precision.

# For j=9, the term is -1 / (9! (k-9)!) * 1^k, which is extremely small for large k like 400.
# Preparing high-precision computation
# We're setting up a Python code using mpmath with dps=300 to compute H(n) for n=3,4,5,10 by summing the Irwin-Hall cdfs up to k=1000 or until the addition is negligible.

# Just found that for k<=n, the cdf is 1, so we can add n+1 for the first part, then compute for k>n.

# The function f(l) = l + exp(-l) - 1 is used to find the roots with high precision using findroot starting from approximate values.
# Refining root finding
# The log10_delta calculation uses the absolute value of left plus p * n divided by log(10), accounting for the exponential decay since p is negative.
# Implementing full code
# The code defines the root-finding function and computes lambda1 and lambda2 with tolerance 1e-290 for high precision p and q.

# Just found interesting result from running the complete code: the first eight non-6 digits for H(10^6) are 54308979, but since the human said this is wrong, perhaps the fit needs more terms or higher dps.