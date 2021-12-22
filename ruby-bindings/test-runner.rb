require_relative './foo'

def assert_eq(lhs, rhs)
  if lhs != rhs
    $stderr.puts "LHS != RHS\nLHS: #{lhs.inspect}\nRHS: #{rhs.inspect}"
    exit 1
  end
end

data = "abc😋中国"
puts data
chars = foo(data)

assert_eq(chars.length, 3)

puts "chars[0] = #{chars[0]}"
assert_eq(chars[0], "😋")

puts "chars[1] = #{chars[1]}"
assert_eq(chars[1], "中")

puts "chars[2] = #{chars[2]}"
assert_eq(chars[2], "国")

puts "All tests passed!"
