{:ok, input} = File.read("input.txt")

input
|> String.split("\n")
|> Enum.map(fn line ->
  [policy, password] = String.split(line, ": ")

  password = String.graphemes(password)

  [policy, char] = String.split(policy, " ")

  [position1, position2] = String.split(policy, "-")

  {position1, ""} = Integer.parse(position1)
  {position2, ""} = Integer.parse(position2)

  {position1, position2, char, password}
end)
|> Enum.count(fn input = {position1, position2, char, password} ->
  position1? = Enum.at(password, position1 - 1) == char
  position2? = Enum.at(password, position2 - 1) == char
  IO.inspect({input, position1?, position2?})
  (position1? or position2?) and not (position1? and position2?)
end)
|> IO.puts()
