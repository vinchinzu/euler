# <p>The Fibonacci sequence is defined by the recurrence relation:</p>
# <blockquote>$F_n = F_{n - 1} + F_{n - 2}$, where $F_1 = 1$ and $F_2 = 1$.</blockquote>
# <p>Hence the first $12$ terms will be:</p>
# \begin{align}
# F_1 &amp;= 1\\
# F_2 &amp;= 1\\
# F_3 &amp;= 2\\
# F_4 &amp;= 3\\
# F_5 &amp;= 5\\
# F_6 &amp;= 8\\
# F_7 &amp;= 13\\
# F_8 &amp;= 21\\
# F_9 &amp;= 34\\
# F_{10} &amp;= 55\\
# F_{11} &amp;= 89\\
# F_{12} &amp;= 144
# \end{align}
# <p>The $12$th term, $F_{12}$, is the first term to contain three digits.</p>
# <p>What is the index of the first term in the Fibonacci sequence to contain $1000$ digits?</p>


require 'bigdecimal'
require 'bigdecimal/math'
include BigMath

# Set precision (50 digits is more than enough)
precision = 50

# Define constants as BigDecimal
one = BigDecimal('1')
five = BigDecimal('5')
ten = BigDecimal('10')

# Compute phi and sqrt(5)
sqrt5 = five.sqrt(precision)
phi = (one + sqrt5) / BigDecimal('2')

# Compute log base 10 using natural log: log10(x) = ln(x) / ln(10)
log10_phi = BigMath.log(phi, precision) / BigMath.log(ten, precision)
log10_sqrt5 = BigMath.log(sqrt5, precision) / BigMath.log(ten, precision)

# Compute n
x = (BigDecimal('999') + log10_sqrt5) / log10_phi
n = x.ceil.to_i

puts n