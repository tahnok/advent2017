input = ARGV[0].chars

total = 0

offset = input.length / 2

input.each_with_index do |item,index|
  matching = input[(index + offset) % input.length]
  if item == matching
    total += item.to_i
  end
end

puts total
