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
|> Enum.filter(fn
  %{
    "byr" => _,
    "iyr" => _,
    "eyr" => _,
    "hgt" => _,
    "hcl" => _,
    "ecl" => _,
    "pid" => _
  } = record ->
    Enum.all?(
      record,
      fn
        {"byr", birthYear} ->
          birthYear = String.to_integer(birthYear)
          1920 <= birthYear and birthYear <= 2002

        {"iyr", issueYear} ->
          issueYear = String.to_integer(issueYear)
          2010 <= issueYear and issueYear <= 2020

        {"eyr", expYear} ->
          expYear = String.to_integer(expYear)

          2020 <= expYear and expYear <= 2030

        {"hgt", height} ->
          case Integer.parse(height) do
            {height, "cm"} ->
              150 <= height and height <= 193

            {height, "in"} ->
              59 <= height and height <= 76

            _ ->
              false
          end

        {"hcl", hairColor} ->
          case hairColor do
            "#" <> hex ->
              String.to_integer(hex, 16) <= String.to_integer("FFFFFF", 16)

            _ ->
              false
          end

        {"ecl", eyeColor} when eyeColor in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"] ->
          true

        {"pid", id} ->
          String.length(id) == 9 and String.to_integer(id) != nil

        {"cid", _} ->
          true

        _ ->
          false
      end
    )

  _ ->
    false
end)
|> Enum.count()
|> IO.puts()
