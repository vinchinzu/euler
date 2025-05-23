#  <p>The first known prime found to exceed one million digits was discovered in 19
# 99, and is a Mersenne prime of the form $2^{6972593} - 1$; it contains exactly $
# 2\,098\,960$ digits. Subsequently other Mersenne primes, of the form $2^p - 1$,
# have been found which contain more digits.</p>
# <p>However, in 2004 there was found a massive non-Mersenne prime which contains
# $2\,357\,207$ digits: $28433 \times 2^{7830457} + 1$.</p>
# <p>Find the last ten digits of this prime number.</p>


x = (28433 * 2 ** 7830457 +1).to_s

puts x[x.length-10..x.length]