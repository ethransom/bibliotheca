defmodule Ferry do
  defstruct x: 0, y: 0, dir: 0

  def rotate(ferry, "L", amount) do
    Map.put(ferry, :dir, clampAngle(ferry.dir - amount))
  end

  def rotate(ferry, "R", amount) do
    Map.put(ferry, :dir, clampAngle(ferry.dir + amount))
  end

  def move(ferry, dist) do
    ferry
    |> Map.put(:x, ferry.x + units(ferry.dir).x * dist)
    |> Map.put(:y, ferry.y + units(ferry.dir).y * dist)
  end

  def move(ferry, "N", dist) do
    Map.put(ferry, :y, ferry.y - dist)
  end

  def move(ferry, "S", dist) do
    Map.put(ferry, :y, ferry.y + dist)
  end

  def move(ferry, "E", dist) do
    Map.put(ferry, :x, ferry.x + dist)
  end

  def move(ferry, "W", dist) do
    Map.put(ferry, :x, ferry.x - dist)
  end

  def offset(ferry) do
    abs(ferry.x) + abs(ferry.y)
  end

  defp units(degrees) do
    %{
      0 => %{x: 1, y: 0},
      90 => %{x: 0, y: 1},
      180 => %{x: -1, y: 0},
      270 => %{x: 0, y: -1},
    }[degrees]
  end

  defp clampAngle(degrees) do
    cond do
      degrees >= 360 ->
        degrees - 360
      degrees < 0 ->
        degrees + 360
      true ->
        degrees
    end
  end
end


defmodule DayTwelve do
  def partOne(input) do
    input
    |> String.split("\n")
    |> Enum.map(fn line ->
      parts = Regex.named_captures(~r/(?<dir>N|E|S|W|F|L|R)(?<arg>[\d]*)/, line)

      {parts["dir"], String.to_integer(parts["arg"])}
    end)
    |> Enum.reduce(%Ferry{}, fn {action, value}, ferry ->
      case action do
        "N" ->
          Ferry.move(ferry, "N", value)
        "S" ->
          Ferry.move(ferry, "S", value)
        "E" ->
          Ferry.move(ferry, "E", value)
        "W" ->
          Ferry.move(ferry, "W", value)
        "F" ->
          Ferry.move(ferry, value)
        "L" ->
          Ferry.rotate(ferry, "L", value)
        "R" ->
          Ferry.rotate(ferry, "R", value)
      end
    end)
    |> Ferry.offset()
  end
end

[
  "example.txt",
  "input.txt"
]
|> Enum.map(fn filename ->
  {:ok, input} = File.read(filename)

  DayTwelve.partOne(input)
  |> IO.inspect(label: filename)
end)
