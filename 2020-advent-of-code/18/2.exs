defmodule DayEighteen do
  defp tokens(""), do: []

  defp tokens(" " <> str), do: tokens(str)

  defp tokens("(" <> str), do: ["(" | tokens(str)]

  defp tokens(")" <> str), do: [")" | tokens(str)]

  defp tokens("+" <> str), do: ["+" | tokens(str)]

  defp tokens("*" <> str), do: ["*" | tokens(str)]

  defp tokens(str) do
    {num, str} = Integer.parse(str)

    [num | tokens(str)]
  end

  defp precedence(op) do
    case op do
      "+" ->
        1
      "*" ->
        0
    end
  end

  defp parseOp([op | tokens]) when op in ["+", "*"] do
    {op, tokens}
  end

  defp peekOp?([op | tokens]) when op in ["+", "*"] do
    op
  end

  defp peekOp?(_tokens) do
    false
  end

  defp parseValue([num | tokens]) when is_number(num) do
    {num, tokens}
  end

  defp parseValue(["(" | tokens]) do
    {val, [")" | tokens]} = parseExpr(tokens)

    {val, tokens}
  end

  defp parseExpr(tokens, lvalue \\ nil) do
    {left, tokens} =
      if lvalue do
        {lvalue, tokens}
      else
        parseValue(tokens)
      end

    {op, tokens} = parseOp(tokens)

    {right, tokens} = parseValue(tokens)

    if nextOp = peekOp?(tokens) do
      if precedence(nextOp) > precedence(op) do
        {next, tokens} = parseExpr(tokens, right)

        {{left, op, next}, tokens}
      else
        parseExpr(tokens, {left, op, right})
      end
    else
      {{left, op, right}, tokens}
    end
  end

  def parse(str) do
    tokens = tokens(str)

    {tree, []} = parseExpr(tokens)
    # {tree, []} = parseExpr(["("] ++ tokens ++ [")"])

    tree
  end

  def evaluate({left, "+", right}) do
    evaluate(left) + evaluate(right)
  end

  def evaluate({left, "*", right}) do
    evaluate(left) * evaluate(right)
  end

  def evaluate(val), do: val
end

[
  "example.txt",
  "input.txt"
]
|> Enum.map(fn filename ->
  {:ok, input} = File.read(filename)

  input
  |> String.split("\n")
  |> Enum.with_index()
  |> Enum.filter(fn
    {"//" <> _line, _num} -> false
    {_line, _num} -> true
  end)
  |> Enum.map(fn {line, num} ->
      try do
        line
        |> DayEighteen.parse()
        # |> IO.inspect()
        |> DayEighteen.evaluate()
        # |> IO.inspect(label: "-> ")
      rescue
        err ->
          IO.puts "Error: #{filename}:#{num+1}:"
          raise err
      end
  end)
  |> Enum.sum()
  |> IO.inspect(label: filename)
end)
