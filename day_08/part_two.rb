VALUES_TO_NUM = {
  %w{a b c   e f g} => 0,
  %w{    c     f  } => 1,
  %w{a   c d e   g} => 2,
  %w{a   c d   f g} => 3,
  %w{  b c d   f  } => 4,
  %w{a b   d   f g} => 5,
  %w{a b   d e f g} => 6,
  %w{a   c     f  } => 7,
  %w{a b c d e f g} => 8,
  %w{a b c d   f g} => 9,
}

training = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab"
data = "cdfeb fcadb cdfeb cdbaf"

# one   (2) =>     c,    f
# four  (4) =>   b,c,d,  f
# seven (3) => a,  c,    f
# eight (7) => a,b,c,d,e,f,g
#
# two   (5) => a,  c,d,e,  g
# three (5) => a,  c,d,  f,g
# five  (5) => a,b,  d,  f,g
#
# zero (6)  => a,b,c,  e,f,g
# six  (6)  => a,b,  d,e,f,g
# nine (6)  => a,b,c,d,  f,g


guesses = []
guesses << Maybe.new(value: %W{a}, is: seg[:seven] & seg[:eight] - seg[:one] - seg[:four])
guesses << Maybe.new(value: %W{e g}, is: seg[:eight] - seg[:seven] - seg[:one] - seg[:four])
guesses << Maybe.new(value: %W{b d}, is: seg[:four] & seg[:eight] - seg[:one] - seg[:seven])
guesses << Maybe.new(value: %W{c f}, is: seg[:one] & seg[:four] & seg[:eight] & seg[:seven])
guesses << Maybe.new(value: %W{a d g}, is: five_count[0] & five_count[1] & five_count[2])
guesses << Maybe.new(value: %W{a b f g}, is: six_count[0] & six_count[1] & six_count[2])

solved = []
while solved.length < 7
  guess = guesses.pop
  if guess.solved?
    solved << guess
    next
  end

  guesses.each do |g|
    guesses.unshift(guess - g) if g.intersect?(guess)
  end

  guesses.unshift(guess)
end

def map_from_training_line(input)
  count_to_sym = {}
  count_to_sym[2] = :one
  count_to_sym[4] = :four
  count_to_sym[3] = :seven
  count_to_sym[7] = :eight

  seg = {}
  maybe = {}
  mapping = {}

  five_count = []
  six_count = []
  input.split(" ").each do |group|
    chars = group.chars.sort
    count = chars.count
    known = count_to_sym[count]
    if known
      seg[known] = chars
    else
      five_count << chars if count == 5
      six_count << chars if count == 6
    end
  end

  mapping["a"] = seg[:seven] & seg[:eight] - seg[:one] - seg[:four]
  maybe["e_g"] = seg[:eight] - seg[:seven] - seg[:one] - seg[:four]
  maybe["b_d"] = seg[:four] & seg[:eight] - seg[:one] - seg[:seven]
  maybe["c_f"] = seg[:one] & seg[:four] & seg[:eight] & seg[:seven]

  maybe["a_d_g"] = five_count[0] & five_count[1] & five_count[2]
  maybe["a_b_f_g"] = six_count[0] & six_count[1] & six_count[2]

  maybe["d_g"] = maybe["a_d_g"] - mapping["a"]
  maybe["b_f_g"] = maybe["a_b_f_g"] - mapping["a"]

  mapping["g"] = maybe["e_g"] & maybe["d_g"]
  mapping["e"] = maybe["e_g"] - mapping["g"]
  mapping["d"] = maybe["d_g"] - mapping["g"]
  mapping["b"] = maybe["b_d"] - mapping["d"]

  mapping["f"] = maybe["b_f_g"] - mapping["b"] - mapping["g"]
  mapping["c"] = maybe["c_f"] - mapping["f"]
  mapping.each_with_object({}) { |(k, v), h| h[k] = v.first }.invert
end

class Line
  def initialize(string)
    training_str, data_str = string.split(" | ")
    @data = data_str.split(" ").map {|group| group.chars.sort }
    @training = training_str.split(" ").map {|group| group.chars.sort }
    @mapping = nil
  end

  private def mapping
    @mapping ||= Training.new(@training).call.mapping
  end
end

class Maybe
  attr_reader :answer, :maybe

  def initialize(value: [], is: [])
    @is = Array(is)
    @value = Array(value)
  end

  def -(other)
    Maybe.new(value: value - other.value, is: is - other.is)
  end

  def intersect?(other)
    (value && other).any?
  end
end

class Training
  def initialize(groups)
    @groups = groups
    @mapping = {}
    @maybe = {}

    @unique_count = {}
    [1,4,7,8].each do |i|
      @unique_count[i] = group.delete {|x| x.count == i }
    end
  end
end

def decode_line(data, mapping:)
  final = String.new
  data.split(" ").each do |group|
    value = digit_for_mapping(group, mapping: mapping)
    final << value.to_s
  end
  final.to_i
end

def digit_for_mapping(group, mapping: )
  key = group.chars.map {|x| mapping.fetch(x) }.sort
  VALUES_TO_NUM.fetch(key)
end

def all_of_it(input)
  count = 0
  input.strip.each_line do |line|
    training, data = line.split(" | ")
    mapping = map_from_training_line(training)
    count += decode_line(data, mapping: mapping)
  end
  puts count
end

all_of_it(File.read("input.txt"))
