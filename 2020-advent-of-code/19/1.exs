defmodule DayNineteen do
  def parseRule("\"" <> line) do
    {:literal, String.trim_trailing(line, "\"")}
  end

  def parseRule(line) do
    case String.split(line, " | ") do
      [composite] ->
        {:composite, parseComposite(composite)}

      [left, right] ->
        {:conditional, parseComposite(left), parseComposite(right)}
    end
  end

  defp parseComposite(composite) do
    composite
    |> String.split(" ")
    |> Enum.map(&String.to_integer/1)
  end

  def matchRules?(rules = [], input = [], depth) do
    debug("#{List.duplicate('  ', depth)} STOP & STOP: #{inspect(rules)} #{inspect(input)}")
    []
  end

  def matchRules?(rules = [_rules | _rest], input = [], depth) do
    debug("#{List.duplicate('  ', depth)} EXTRA RULES: #{inspect(rules)} #{inspect(input)}")
    false
  end

  def matchRules?(rules = [], input = [_input | _rest], depth) do
    debug("#{List.duplicate('  ', depth)} EXTRA INPUT: #{inspect(rules)} #{inspect(input)}")
    input
  end

  def matchRules?(r = [firstRule | rules], i = [first | input], depth) when is_binary(firstRule) do
    debug("#{List.duplicate('  ', depth)} LITERAL: #{inspect(r)} #{inspect(i)}")
    if firstRule == first do
      matchRules?(rules, input, depth)
    else
      false
    end
  end

  def matchRules?(r = [{left, right} | rules], i = input, depth) do
    debug("#{List.duplicate('  ', depth)} CONDITION: #{inspect(r)} #{inspect(i)}")
    cond do
      input = matchRules?(left, input, depth + 1) ->
        matchRules?(rules, input, depth)

      input = matchRules?(right, input, depth + 1) ->
        matchRules?(rules, input, depth)

      true ->
        false
    end
  end

  defp debug(str) do
    if false do
      IO.puts(str)
    end
  end

  def collapse(rules) do
    collapse(rules, rules[0])
  end

  defp collapse(rules, {:composite, parts}) do
    parts
    |> Enum.map(&collapse(rules, rules[&1]))
    |> List.flatten()
  end

  defp collapse(rules, {:conditional, left, right}) do
    {collapse(rules, {:composite, left}), collapse(rules, {:composite, right})}
  end

  defp collapse(_rules, {:literal, val}) do
    [val]
  end
end

[
  "example.txt",
  "input.txt"
]
|> Enum.map(fn filename ->
  {:ok, input} = File.read(filename)

  [rules, messages] = String.split(input, "\n\n")

  rules =
    rules
    |> String.split("\n")
    |> Enum.map(fn line ->
      {name, line} = Integer.parse(line)

      line =
        line
        |> String.trim_leading(": ")
        |> DayNineteen.parseRule()

      {name, line}
    end)
    |> Map.new()
    |> DayNineteen.collapse()
    # |> IO.inspect(label: "rules")

  messages
  |> String.split("\n")
  |> Enum.filter(fn message ->
    match?([], DayNineteen.matchRules?(rules, String.graphemes(message), 0)) # |> IO.inspect(label: message)
  end)
  |> Enum.count()
  |> IO.inspect(label: filename)
end)
