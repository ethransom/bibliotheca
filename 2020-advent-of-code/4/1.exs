{:ok, input} = File.read("input.txt")

input
|> String.split("\n\n")
|> Enum.map(fn record ->
  record
  |> String.split([" ", "\n"])
  |> Enum.map(fn field ->
    [key, value] = String.split(field, ":")
    {key, value}
  end)
  |> Map.new()
end)
|> Enum.map(fn record ->
  match?(
    %{
      "byr" => _,
      "iyr" => _,
      "eyr" => _,
      "hgt" => _,
      "hcl" => _,
      "ecl" => _,
      "pid" => _
    },
    record
  )
end)
|> IO.inspect()
|> Enum.count(& &1)
|> IO.puts()
