#  <p>By replacing each of the letters in the word CARE with $1$, $2$, $9$, and $6$
#  respectively, we form a square number: $1296 = 36^2$. What is remarkable is tha
# t, by using the same digital substitutions, the anagram, RACE, also forms a squa
# re number: $9216 = 96^2$. We shall call CARE (and RACE) a square anagram word pa
# ir and specify further that leading zeroes are not permitted, neither may a diff
# erent letter have the same digital value as another letter.</p>
# <p>Using <a href="resources/documents/0098_words.txt">words.txt</a> (right click
#  and 'Save Link/Target As...'), a 16K text file containing nearly two-thousand c
# ommon English words, find all the square anagram word pairs (a palindromic word
# is NOT considered to be an anagram of itself).</p>
# <p>What is the largest square number formed by any member of such a pair?</p>
# <p class="smaller">NOTE: All anagrams formed must be contained in the given text
#  file.</p>

require 'set'

# Read words.txt
# Assuming words.txt is in the same directory as the script.
# The problem statement link is "resources/documents/0098_words.txt".
# Using 'words.txt' as per the original script.
words = File.read('words.txt').gsub(/"/, '').split(',').map(&:strip)

# Determine maximum word length
max_len = 0
words.each { |w| max_len = [max_len, w.length].max }

# Precompute squares, grouped by length
# No filter for unique digits in the square itself here.
squares_by_length = Hash.new { |h, k| h[k] = Set.new }
n = 1
loop do
  square = n * n
  s_str = square.to_s
  break if s_str.length > max_len
  squares_by_length[s_str.length].add(square)
  n += 1
end

# Group words by sorted letters (signature) to find anagrams
# Only consider groups that can actually form pairs
anagram_groups = words.group_by { |w| w.chars.sort.join }.values.select { |group| group.length > 1 }

max_found_square = 0

anagram_groups.each do |group|
  word_len = group[0].length # All words in an anagram group have the same length
  candidate_squares = squares_by_length[word_len]
  next if candidate_squares.empty?

  group.combination(2).each do |word1, word2|
    # word1 and word2 are anagrams and have length word_len

    candidate_squares.each do |square1_val|
      s1_chars = square1_val.to_s.chars # Length is already word_len

      mapping = {}
      possible_map = true
      
      # Build mapping from word1 to s1_chars
      word1.chars.each_with_index do |char, i|
        digit = s1_chars[i]
        if mapping.key?(char)
          # If char already mapped, ensure consistency
          if mapping[char] != digit
            possible_map = false
            break
          end
        else
          # New char to map.
          # The check for "different letter have same digital value"
          # is done after forming the full mapping.
          mapping[char] = digit
        end
      end
      next unless possible_map

      # Check: "neither may a different letter have the same digital value as another letter."
      # This means the number of unique digits used must equal the number of unique letters in the word.
      # mapping.keys contains unique letters from word1.
      # mapping.values contains the digits they map to.
      # If word1 is "NOON", mapping.keys=["N","O"]. mapping.values might be ["1","0"]. Uniq count is 2. OK.
      # If mapping results in N->1, O->1, then mapping.values=["1","1"]. Uniq count is 1. Not OK.
      next if mapping.values.uniq.length != mapping.keys.length


      # Form number string for word2 using the mapping
      s2_chars = word2.chars.map { |char_in_w2| mapping[char_in_w2] }
      # Anagrams have same set of characters, so all char_in_w2 will be in mapping.keys
      s2_str = s2_chars.join

      # Validate s2_str
      # 1. Must have correct length (guaranteed by map from word2 of same length)
      #    but check anyway for safety, though map should handle it.
      next if s2_str.length != word_len
      
      # 2. No leading zero (if length > 1)
      next if word_len > 1 && s2_str[0] == '0'

      square2_val = s2_str.to_i

      # Check if square2_val is a valid square of the correct length
      if candidate_squares.include?(square2_val)
        # Found a square anagram pair (word1, square1_val) and (word2, square2_val)
        max_found_square = [max_found_square, square1_val, square2_val].max
      end
    end
  end
end

puts "Largest square number: #{max_found_square}"
