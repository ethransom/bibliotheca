defmodule DayEight do
  def evaluate(instructions, pointer \\ 0, accumulator \\ 0, seen \\ []) do
    {index, instr, arg} = Enum.at(instructions, pointer)

    if Enum.member?(seen, index) do
      accumulator
    else
      seen = [index | seen]

      case instr do
        "nop" ->
          evaluate(instructions, pointer + 1, accumulator, seen)

        "acc" ->
          evaluate(instructions, pointer + 1, accumulator + arg, seen)

        "jmp" ->
          evaluate(instructions, pointer + arg, accumulator, seen)
      end
    end
  end
end

{:ok, input} = File.read("input.txt")


instructions =
  input
  |> String.split("\n")
  |> Enum.with_index()
  |> Enum.map(fn {line, index} ->
    [cmd, arg] = String.split(line, " ")

    {arg, ""} = Integer.parse(arg)

    {index, cmd, arg}
  end)
  # |> IO.inspect()

DayEight.evaluate(instructions)
|> IO.puts()
