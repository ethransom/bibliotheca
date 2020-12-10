defmodule DayTen do
  def solve(filename) do
    {:ok, input} = File.read(filename)

    adapters =
      input
      |> String.split("\n")
      |> Enum.map(&String.to_integer/1)
      |> Enum.sort()

    adapters = [0] ++ adapters ++ [Enum.max(adapters) + 3]

    count_combos(adapters)
    |> List.first()
  end

  defp count_combos([_one, _two]) do
    [1, 1]
  end

  defp count_combos(list) do
    [head | rest] = list

    combos_rest = count_combos(rest)

    combos =
      [0, 1, 2]
      |> Enum.map(fn pos ->
        if head + 3 >= Enum.at(rest, pos) do
          Enum.at(combos_rest, pos)
        else
          0
        end
      end)
      |> Enum.sum()

    [combos | combos_rest]
  end
end

[
  "example1.txt",
  "example2.txt",
  "input.txt"
]
|> Enum.each(fn filename ->
  DayTen.solve(filename)
  |> IO.inspect(label: filename)
end)
