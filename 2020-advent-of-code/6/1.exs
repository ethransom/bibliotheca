{:ok, input} = File.read("input.txt")

input
|> String.split("\n\n")
|> Enum.map(fn group ->
  group
  |> String.graphemes()
  |> Enum.filter(& &1 != "\n")
  |> Enum.uniq()
  |> Enum.count()
end)
|> Enum.sum()
|> IO.puts()
