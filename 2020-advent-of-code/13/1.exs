[
  "example.txt",
  "input.txt"
]
|> Enum.each(fn filename ->
  {:ok, input} = File.read(filename)

  [line1, line2] = String.split(input, "\n")

  timestamp = String.to_integer(line1) |> IO.inspect(label: "timestamp")

  ids =
    line2
    |> String.split(",")
    |> Enum.filter(& &1 != "x")
    |> Enum.map(&String.to_integer/1)
    |> IO.inspect(label: "buses")

  earliest =
    Stream.iterate(timestamp, & &1 + 1)
    |> Enum.find_value(fn time ->
      Enum.map(ids, fn id ->
        if rem(time, id) == 0 do
          id * (time - timestamp)
        end
      end)
      |> IO.inspect(label: time)
      |> Enum.find(& &1)
    end)

  IO.inspect(earliest, label: filename)
end)
