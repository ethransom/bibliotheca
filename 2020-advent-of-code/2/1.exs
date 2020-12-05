{:ok, input} = File.read("input.txt")

input
|> String.split("\n")
|> Enum.map(fn line ->
  [policy, password] = String.split(line, ": ")

  [policy, char] = String.split(policy, " ")

  [min, max] = String.split(policy, "-")

  {min, ""} = Integer.parse(min)
  {max, ""} = Integer.parse(max)

  {min, max, char, password}
end)
|> Enum.count(fn {min, max, char, password} ->
  count = password |> String.graphemes() |> Enum.count(& &1 == char)
  min <= count and count <= max
end)
|> IO.puts()
