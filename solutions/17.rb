# #euler 17

# If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

# If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?


# NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.

# Pure Ruby solution, no humanize
def number_to_words(n)
  ones = %w[zero one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen]
  tens = %w[zero ten twenty thirty forty fifty sixty seventy eighty ninety]

  return ones[n] if n < 20
  return tens[n / 10] + (n % 10 == 0 ? '' : ones[n % 10]) if n < 100
  if n < 1000
    rem = n % 100
    return ones[n / 100] + 'hundred' + (rem == 0 ? '' : 'and' + number_to_words(rem))
  end
  return 'onethousand' if n == 1000
end

total = (1..1000).map { |x| number_to_words(x) }.join.length
puts total