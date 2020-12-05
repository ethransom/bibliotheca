{:ok, input} = File.read("input.txt")

lines =
  input
  |> String.split("\n")
  |> Enum.map(&String.graphemes/1)

[
  # right, down
  {1, 1},
  {3, 1},
  {5, 1},
  {7, 1},
  {1, 2}
]
|> Enum.map(fn {right, down} ->
  lines
  |> Enum.with_index()
  |> Enum.filter(fn {_, index} -> rem(index, down) == 0 end)
  |> Enum.map(fn {line, _} -> line end)
  |> Enum.with_index()
  |> Enum.count(fn {line, index} ->
    pos = rem(right * index, length(line))

    Enum.at(line, pos) == "#"
  end)
end)
|> IO.inspect()
|> Enum.reduce(&*/2)
|> IO.puts()
