# Problem 107: Minimal Network
#<p>The following undirected network consists of seven vertices and twelve edges with a total weight of 243.</p>
#<div class="center">
#<img src="resources/images/0107_1.png?1678992052" class="dark_img" alt=""><br></div>
#<p>The same network can be represented by the matrix below.</p>
#<table cellpadding="5" cellspacing="0" border="1" align="center"><tr><td>    </td><td><b>A</b></td><td><b>B</b></td><td><b>C</b></td><td><b>D</b></td><td><b>E</b></td><td><b>F</b></td><td><b>G</b></td>
#</tr><tr><td><b>A</b></td><td>-</td><td>16</td><td>12</td><td>21</td><td>-</td><td>-</td><td>-</td>
#</tr><tr><td><b>B</b></td><td>16</td><td>-</td><td>-</td><td>17</td><td>20</td><td>-</td><td>-</td>
#</tr><tr><td><b>C</b></td><td>12</td><td>-</td><td>-</td><td>28</td><td>-</td><td>31</td><td>-</td>
#</tr><tr><td><b>D</b></td><td>21</td><td>17</td><td>28</td><td>-</td><td>18</td><td>19</td><td>23</td>
#</tr><tr><td><b>E</b></td><td>-</td><td>20</td><td>-</td><td>18</td><td>-</td><td>-</td><td>11</td>
#</tr><tr><td><b>F</b></td><td>-</td><td>-</td><td>31</td><td>19</td><td>-</td><td>-</td><td>27</td>
#</tr><tr><td><b>G</b></td><td>-</td><td>-</td><td>-</td><td>23</td><td>11</td><td>27</td><td>-</td>
#</tr></table><p>However, it is possible to optimise the network by removing some edges and still ensure that all points on the network remain connected. The network which achieves the maximum saving is shown below. It has a weight of 93, representing a saving of 243 − 93 = 150 from the original network.</p>
#<div class="center">
#<img src="resources/images/0107_2.png?1678992052" class="dark_img" alt=""><br></div>
#<p>Using <a href="resources/documents/0107_network.txt">network.txt</a> (right click and 'Save Link/Target As...'), a 6K text file containing a network with forty vertices, and given in matrix form, find the maximum saving which can be achieved by removing redundant edges whilst ensuring that the network remains connected.</p>

class DSU
  def initialize(n)
    @parent = (0...n).to_a
    @rank = Array.new(n, 0)
  end

  def find(x)
    @parent[x] = find(@parent[x]) unless @parent[x] == x
    @parent[x]
  end

  def union(x, y)
    px, py = find(x), find(y)
    return false if px == py
    if @rank[px] < @rank[py]
      @parent[px] = py
    elsif @rank[px] > @rank[py]
      @parent[py] = px
    else
      @parent[py] = px
      @rank[px] += 1
    end
    true
  end
end

def max_saving(network_data)
  edges = []
  total_weight = 0

  # Parse the matrix
  rows = network_data.strip.split("\n")
  rows.each_with_index do |line, u|
    # Split and strip each entry to handle spaces around '-'
    parts = line.strip.split(',').map(&:strip)
    parts.each_with_index do |part, v|
      next if part == '-' # Skip non-edges
      weight = part.to_i
      if u < v
        edges << [weight, u, v]
        total_weight += weight
      end
    end
  end

  # Kruskal's algorithm
  edges.sort_by!(&:first) # Sort by weight efficiently
  dsu = DSU.new(rows.length)
  mst_weight = 0
  edge_count = 0
  target_edges = rows.length - 1

  edges.each do |weight, u, v|
    if dsu.union(u, v)
      mst_weight += weight
      edge_count += 1
      break if edge_count == target_edges
    end
  end

  total_weight - mst_weight
end

# Network data (replace with File.read('network.txt') for actual file)
network_data = File.read('../network.txt')

saving = max_saving(network_data)
puts "#{saving}"

