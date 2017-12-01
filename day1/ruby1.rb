input = ARGV[0].chars

total = 0

input.each_cons(2) do |a,b|
  if a == b
    total += a.to_i
  end
end

if input[0] == input[-1]
  total += input[0].to_i
end

puts total
