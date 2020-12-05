{:ok, input} = File.read("input.txt")

input
|> String.split("\n")
|> Enum.map(fn pass ->
  pass
  |> String.graphemes()
  |> Enum.reduce({0, 127, 0, 7}, fn char, {rowStart, rowEnd, colStart, colEnd} ->
    case char do
      "F" -> {rowStart, div(rowEnd - rowStart, 2) + rowStart, colStart, colEnd}
      "B" -> {div(rowEnd - rowStart, 2) + rowStart + 1, rowEnd, colStart, colEnd}
      "L" -> {rowStart, rowEnd, colStart, div(colEnd - colStart, 2) + colStart}
      "R" -> {rowStart, rowEnd, div(colEnd - colStart, 2) + colStart + 1, colEnd}
    end
  end)
end)
|> Enum.map(fn {row, _, col, _} ->
  row * 8 + col
end)
|> Enum.max()
|> IO.puts()
