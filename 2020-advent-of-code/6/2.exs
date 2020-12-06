questions =
  "abcdefghijklmnopqrstuvwxyz"
  |> String.graphemes()

{:ok, input} = File.read("input.txt")

input
|> String.split("\n\n")
|> Enum.map(fn group ->
  answers = String.split(group, "\n") |> Enum.map(&String.graphemes/1)

  questions
  |> Enum.count(fn letter ->
    Enum.all?(answers, fn answer ->
      Enum.member?(answer, letter)
    end)
  end)
end)
|> Enum.sum()
|> IO.inspect()
