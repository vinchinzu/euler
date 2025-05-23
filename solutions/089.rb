#  <p>For a number written in Roman numerals to be considered valid there are basic
#  rules which must be followed. Even though the rules allow some numbers to be ex
# pressed in more than one way there is always a "best" way of writing a particula
# r number.</p>
# <p>For example, it would appear that there are at least six ways of writing the
# number sixteen:</p>
# <p class="margin_left monospace">IIIIIIIIIIIIIIII<br>
# VIIIIIIIIIII<br>
# VVIIIIII<br>
# XIIIIII<br>
# VVVI<br>
# XVI</p>
# <p>However, according to the rules only <span class="monospace">XIIIIII</span> a
# nd <span class="monospace">XVI</span> are valid, and the last example is conside
# red to be the most efficient, as it uses the least number of numerals.</p>
# <p>The 11K text file, <a href="resources/documents/0089_roman.txt">roman.txt</a>
#  (right click and 'Save Link/Target As...'), contains one thousand numbers writt
# en in valid, but not necessarily minimal, Roman numerals; see <a href="about=rom
# an_numerals">About... Roman Numerals</a> for the definitive rules for this probl
# em.</p>
# <p>Find the number of characters saved by writing each of these in their minimal
#  form.</p>
# <p class="smaller">Note: You can assume that all the Roman numerals in the file
# contain no more than four consecutive identical units.</p>

# Solution for Project Euler Problem 89

ROMAN_VALUES = {
  'I' => 1, 'V' => 5, 'X' => 10, 'L' => 50,
  'C' => 100, 'D' => 500, 'M' => 1000
}.freeze

MINIMAL_ROMAN_RULES = [
  [1000, "M"], [900, "CM"], [500, "D"], [400, "CD"], [100, "C"],
  [90, "XC"], [50, "L"], [40, "XL"], [10, "X"], [9, "IX"], [5, "V"],
  [4, "IV"], [1, "I"]
].freeze

# This data will be replaced by the full list from the problem description in execution.
# Using a small example here for syntax validation.
ROMAN_NUMERAL_DATA = <<~HEREDOC_DELIMITER
MMMMDCLXXII
MMDCCLXXXIII
MMMDLXVIIII
XVI
XIIIIII
HEREDOC_DELIMITER

def roman_to_int(roman_str)
  total = 0
  i = 0
  while i < roman_str.length
    val1 = ROMAN_VALUES[roman_str[i]]
    
    if i + 1 < roman_str.length
      val2 = ROMAN_VALUES[roman_str[i+1]]
      if val1 < val2
        total += (val2 - val1)
        i += 2
      else
        total += val1
        i += 1
      end
    else
      total += val1
      i += 1
    end
  end
  total
end

def int_to_minimal_roman(number)
  return "" if number <= 0
  result_roman = ""
  MINIMAL_ROMAN_RULES.each do |value, numeral_string|
    while number >= value
      result_roman += numeral_string
      number -= value
    end
  end
  result_roman
end

# Main logic
total_characters_saved = 0
original_numerals_list = ROMAN_NUMERAL_DATA.strip.lines.map(&:strip)

original_numerals_list.each do |original_roman|
  next if original_roman.empty? # Skip empty lines if any

  original_length = original_roman.length
  
  integer_value = roman_to_int(original_roman)
  minimal_roman = int_to_minimal_roman(integer_value)
  minimal_length = minimal_roman.length
  
  total_characters_saved += (original_length - minimal_length)
end

puts "Total characters saved: #{total_characters_saved}"
