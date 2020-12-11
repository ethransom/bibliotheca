defmodule DayNine do
  def any_sum([_item], _sum), do: false

  def any_sum(list, sum) do
    [first | rest] = list

    with false <- Enum.any?(rest, & &1 + first == sum) do
      any_sum(rest, sum)
    end
  end
end

{:ok, input} = File.read("input.txt")

window = 25

input =
  input
  |> String.split("\n")
  |> Enum.map(&String.to_integer/1)

input
|> Enum.with_index()
|> Enum.drop(window)
|> Enum.find_value(fn {value, index} ->
  range =
    input
    |> Enum.slice(index - window, window)

  unless DayNine.any_sum(range, value) do
    value
  end
end)
|> IO.puts()
