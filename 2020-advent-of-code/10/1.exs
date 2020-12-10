defmodule DayTen do
  def solve(filename) do
    {:ok, input} = File.read(filename)

    adapters =
      input
      |> String.split("\n")
      |> Enum.map(&String.to_integer/1)

    adapters = adapters ++ [Enum.max(adapters) + 3]

    {differences, _acc} =
      adapters
      |> Enum.sort()
      |> Enum.map_reduce(0, fn this, prev ->
        {this - prev, this}
      end)

    freqs = Enum.frequencies(differences)

    freqs[1] * freqs[3]
  end
end

["example1.txt", "example2.txt", "input.txt"]
|> Enum.each(fn filename ->
  IO.puts("#{filename}:")

  DayTen.solve(filename)
  |> IO.inspect(label: "\tSOLN")
end)
