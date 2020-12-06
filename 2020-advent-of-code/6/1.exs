{:ok, input} = File.read("input.txt")

input
|> String.split("\n\n")
|> Enum.map(fn group ->
  String.split(group, "\n")
  |> Enum.map(fn line ->
    line |> String.graphemes() |> Enum.map(& {&1, 0})
  end)
  |> List.flatten()
  |> Map.new()
  |> Enum.count()
end)
|> Enum.sum()
|> IO.puts()
