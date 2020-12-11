defmodule DayEleven do
  def simulate(cells, height, width) do
    # run an interation of the simulation
    # newCells =
    #   cells
    #   |> Enum.map(fn rows ->
    #     rows
    #     |> Enum.map(fn cell ->
    #       cell
    #     end)
    #   end)
    #   |> IO.inspect()

    newCells = iterate(cells, height, width)

    # compare new cells to old cells

    # if nothing has changed, return oldcells

    # else, return simulate(cells)
  end

  defp iterate(rows, height, width) do
    [first, rest] = rows

    [first | iterateRows(first, rest)]
  end

  defp iterateRows(_prev, [_this | [_next | []]]), do: []

  defp iterateRows(prev, [this | rest = [next | _rem]]) do
    IO.inspect(this)

    [this | rest]
  end
end

[
  "example.txt"
]
|> Enum.map(fn filename ->
  {:ok, input} = File.read(filename)

  cells =
    input
    |> String.split("\n")
    |> Enum.map(&String.graphemes/1)
    |> IO.inspect()

  height = Enum.count(cells)
  width = Enum.count(List.first(cells))

  # pad with floor
  cells =
    List.duplicate(".", 1 + height + 1) ++
      Enum.map(cells, &(["."] ++ &1 ++ ["."])) ++ List.duplicate(".", 1 + height + 1)

  DayEleven.simulate(cells, height, width)
  |> List.flatten()
  |> Enum.count(&(&1 == "#"))
  |> IO.inspect(label: filename)
end)
