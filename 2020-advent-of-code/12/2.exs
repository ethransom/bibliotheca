defmodule Ferry do
  defstruct x: 0, y: 0, waypointX: 0, waypointY: 0, dir: 0

  def rotate(ferry, _dir, 0), do: ferry

  def rotate(ferry, "L", amount) do
    ferry
    |> Map.put(:waypointX, ferry.waypointY)
    |> Map.put(:waypointY, -ferry.waypointX)
    |> rotate("L", amount - 90)
  end

  def rotate(ferry, "R", amount) do
    ferry
    |> Map.put(:waypointX, -ferry.waypointY)
    |> Map.put(:waypointY, ferry.waypointX)
    |> rotate("R", amount - 90)
  end

  def move(ferry, 0), do: ferry

  def move(ferry, count) do
    ferry
    |> Map.put(:x, ferry.x + ferry.waypointX)
    |> Map.put(:y, ferry.y + ferry.waypointY)
    |> move(count - 1)
  end

  def move(ferry, "N", dist) do
    Map.put(ferry, :waypointY, ferry.waypointY - dist)
  end

  def move(ferry, "S", dist) do
    Map.put(ferry, :waypointY, ferry.waypointY + dist)
  end

  def move(ferry, "E", dist) do
    Map.put(ferry, :waypointX, ferry.waypointX + dist)
  end

  def move(ferry, "W", dist) do
    Map.put(ferry, :waypointX, ferry.waypointX - dist)
  end

  def offset(ferry) do
    abs(ferry.x) + abs(ferry.y)
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
    |> Enum.reduce(%Ferry{waypointX: 10, waypointY: -1}, fn {action, value}, ferry ->
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
