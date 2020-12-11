defmodule DayEight do
  def evaluate(instructions, pointer \\ 0, accumulator \\ 0, seen \\ []) do
    case Enum.at(instructions, pointer) do
      {index, instr, arg} ->
        if Enum.member?(seen, index) do
          {false, accumulator}
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
      nil ->
        {true, accumulator}
    end
  end

  def decorrupt(instructions) do
    instructions
    |> Enum.filter(fn {_index, instr, _arg} -> instr != "acc" end)
    |> Enum.map(fn {index, instr, arg} ->
      instr = %{"jmp" => "nop", "nop" => "jmp"}[instr]

      program = List.replace_at(instructions, index, {index, instr, arg})

      {exited?, accumulator} = evaluate(program)

      {exited?, accumulator}
    end)
    |> Enum.find(fn {exited?, _acc} -> exited? end)
  end

  # def decorrupt([]), do: [[]] |> IO.inspect(label: "base")

  # def decorrupt(instructions) do
  #   [{index, instr, arg} | instructions] = instructions

  #   IO.inspect({index, instr, arg}, label: "decorrupting")

  #   fixes =
  #     case instr do
  #       "acc" ->
  #         [
  #           {index, "acc", arg}
  #         ]

  #       "nop" ->
  #         [
  #           {index, "jmp", arg},
  #           {index, "nop", arg}
  #         ]

  #       "jmp" ->
  #         [
  #           {index, "jmp", arg},
  #           {index, "nop", arg}
  #         ]
  #     end

  #   IO.inspect(fixes, label: "fixes")

  #   rest = decorrupt(instructions) |> IO.inspect(label: "rest")

  #   Enum.map(fixes, fn fix ->
  #     Enum.map(rest, fn program ->
  #       IO.inspect([fix | program], label: "\t\tfix | program")
  #       [fix | program]
  #     end)
  #   end)
  #   # |> List.flatten()
  #   |> IO.inspect(rest, label: "\tperms")
  # end
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

{true, accumulator} = DayEight.decorrupt(instructions)

IO.puts(accumulator)
