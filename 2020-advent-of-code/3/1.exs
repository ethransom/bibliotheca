{:ok, input} = File.read("input.txt")

input
|> String.split("\n")
|> Enum.map(&String.graphemes/1)
|> Enum.with_index()
|> Enum.count(fn {line, index} ->
  cond do
    index == 0 -> false # don't count starting position
    true ->
      pos = rem(3 * index, length(line))
      Enum.at(line, pos) == "#"
  end
end)
|> IO.puts()
