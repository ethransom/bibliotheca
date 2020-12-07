defmodule DaySeven do
  def find(rules, searchColor) do
    rules[searchColor]
    |> Enum.map(fn {color, count} ->
      count * find(rules, color) + count
    end)
    |> Enum.sum()
  end
end

{:ok, input} = File.read("input.txt")

input
|> String.split("\n")
|> Enum.map(fn rules ->
  [color, contents] = String.split(rules, " bags contain ")

  contents =
    contents
    |> String.trim_trailing(".")
    |> case do
      "no other bags" ->
        %{}

      contents ->
        contents
        |> String.split(", ")
        |> Enum.map(fn rule ->
          {count, remainder} = Integer.parse(rule)

          color =
            remainder
            |> String.trim_trailing("bags")
            |> String.trim_trailing("bag")
            |> String.trim()

          {color, count}
        end)
        |> Map.new()
    end

  {color, contents}
end)
|> Map.new()
|> DaySeven.find("shiny gold")
|> IO.inspect()
