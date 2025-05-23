# Project Euler 54: Poker hands
# <p>In the card game poker, a hand consists of five cards and are ranked, from lowest to highest, in the following way:</p>
# <ul><li><b>High Card</b>: Highest value card.</li>
# <li><b>One Pair</b>: Two cards of the same value.</li>
# <li><b>Two Pairs</b>: Two different pairs.</li>
# <li><b>Three of a Kind</b>: Three cards of the same value.</li>
# <li><b>Straight</b>: All cards are consecutive values.</li>
# <li><b>Flush</b>: All cards of the same suit.</li>
# <li><b>Full House</b>: Three of a kind and a pair.</li>
# <li><b>Four of a Kind</b>: Four cards of the same value.</li>
# <li><b>Straight Flush</b>: All cards are consecutive values of same suit.</li>
# <li><b>Royal Flush</b>: Ten, Jack, Queen, King, Ace, in same suit.</li>
# </ul><p>The cards are valued in the order:<br>2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.</p>
# <p>If two players have the same ranked hands then the rank made up of the highest value wins; for example, a pair of eights beats a pair of fives (see example 1 below). But if two ranks tie, for example, both players have a pair of queens, then highest cards in each hand are compared (see example 4 below); if the highest cards tie then the next highest cards are compared, and so on.</p>
# <p>Consider the following five hands dealt to two players:</p>
# <div class="center">
# <table><tr><td><b>Hand</b></td><td> </td><td><b>Player 1</b></td><td> </td><td><b>Player 2</b></td><td> </td><td><b>Winner</b></td>
# </tr><tr><td><b>1</b></td><td> </td><td>5H 5C 6S 7S KD<br><div class="smaller">Pair of Fives</div></td><td> </td><td>2C 3S 8S 8D TD<br><div class="smaller">Pair of Eights</div></td><td> </td><td>Player 2</td>
# </tr><tr><td><b>2</b></td><td> </td><td>5D 8C 9S JS AC<br><div class="smaller">Highest card Ace</div></td><td> </td><td>2C 5C 7D 8S QH<br><div class="smaller">Highest card Queen</div></td><td> </td><td>Player 1</td>
# </tr><tr><td><b>3</b></td><td> </td><td>2D 9C AS AH AC<br><div class="smaller">Three Aces</div></td><td> </td><td>3D 6D 7D TD QD<br><div class="smaller">Flush  with Diamonds</div></td><td> </td><td>Player 2</td>
# </tr><tr><td><b>4</b></td><td> </td><td>4D 6S 9H QH QC<br><div class="smaller">Pair of Queens<br>Highest card Nine</div></td><td> </td><td>3D 6D 7H QD QS<br><div class="smaller">Pair of Queens<br>Highest card Seven</div></td><td> </td><td>Player 1</td>
# </tr><tr><td><b>5</b></td><td> </td><td>2H 2D 4C 4D 4S<br><div class="smaller">Full House<br>With Three Fours</div></td><td> </td><td>3C 3D 3S 9S 9D<br><div class="smaller">Full House<br>with Three Threes</div></td><td> </td><td>Player 1</td>
# </tr></table></div>
# <p>The file, <a href="resources/documents/0054_poker.txt">poker.txt</a>, contains one-thousand random hands dealt to two players. Each line of the file contains ten cards (separated by a single space): the first five are Player 1's cards and the last five are Player 2's cards. You can assume that all hands are valid (no invalid characters or repeated cards), each player's hand is in no specific order, and in each hand there is a clear winner.</p>
# <p>How many hands does Player 1 win?</p>

CARD_VALUES = {
  '2' => 2, '3' => 3, '4' => 4, '5' => 5, '6' => 6, '7' => 7, '8' => 8, '9' => 9, 'T' => 10,
  'J' => 11, 'Q' => 12, 'K' => 13, 'A' => 14
}

def parse_hand(cards)
  values = cards.map { |c| CARD_VALUES[c[0]] }.sort.reverse
  suits = cards.map { |c| c[1] }
  [values, suits]
end

def hand_rank(cards)
  values, suits = parse_hand(cards)
  counts = values.tally.sort_by { |v, c| [-c, -v] }
  ordered = counts.map(&:first)
  is_flush = suits.uniq.size == 1
  is_straight = values == (values[0].downto(values[0]-4).to_a)
  is_wheel = values == [14, 5, 4, 3, 2] # A-2-3-4-5 straight
  if is_straight || is_wheel
    straight_high = 5 if is_wheel
    straight_high ||= values.max
  end
  if is_flush && (is_straight || is_wheel)
    return [8, straight_high]
  elsif counts[0][1] == 4
    return [7, counts[0][0], counts[1][0]]
  elsif counts[0][1] == 3 && counts[1][1] == 2
    return [6, counts[0][0], counts[1][0]]
  elsif is_flush
    return [5, *values]
  elsif is_straight || is_wheel
    return [4, straight_high]
  elsif counts[0][1] == 3
    return [3, counts[0][0], *ordered]
  elsif counts[0][1] == 2 && counts[1][1] == 2
    return [2, [counts[0][0], counts[1][0]].max, [counts[0][0], counts[1][0]].min, *ordered]
  elsif counts[0][1] == 2
    return [1, counts[0][0], *ordered]
  else
    return [0, *values]
  end
end

file = File.open("poker.txt")
count = 0
file.each_line do |line|
  cards = line.strip.split
  hand1 = cards[0,5]
  hand2 = cards[5,5]
  count += 1 if (hand_rank(hand1) <=> hand_rank(hand2)) == 1
end
puts count