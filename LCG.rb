#lcg

# You can create multiple instances of LCG::Berkeley or LCG::Microsoft. Each instance privately keeps the original seed in @seed, and the current state in @r. Each class resembles the core Random class, but with fewer features. The .new method takes a seed. The #rand method returns the next random number. The #seed method returns the original seed.

module LCG
  module Common
    # The original seed of this generator.
    attr_reader :seed
 
    # Creates a linear congruential generator with the given _seed_.
    def initialize(seed)
      @seed = @r = seed
    end
  end
 
  # LCG::Berkeley generates 31-bit integers using the same formula
  # as BSD rand().
  class Berkeley
    include Common
    def rand
      @r = (1103515245 * @r + 12345) & 0x7fff_ffff
    end
  end
 
  # LCG::Microsoft generates 15-bit integers using the same formula
  # as rand() from the Microsoft C Runtime.
  class Microsoft
    include Common
    def rand
      @r = (214013 * @r + 2531011) & 0x7fff_ffff
      @r >> 16
    end
  end
end