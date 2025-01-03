defmodule Aoc.Year2015.Day03 do
  def part1(input) do
    {_, visited} = solve(parse(input))
    MapSet.size(visited)
  end

  def part2(input) do
    {santa_directions, robo_directions} =
      parse(input)
      |> Enum.chunk_every(2)
      |> Enum.map(&List.to_tuple/1)
      |> Enum.unzip()

    {_, santa_visited} = solve(santa_directions)
    {_, robo_visited} = solve(robo_directions)

    MapSet.union(santa_visited, robo_visited) |> MapSet.size()
  end

  def solve(directions) do
    position = %{x: 0, y: 0}
    visited = MapSet.new() |> MapSet.put(position)

    Enum.reduce(directions, {position, visited}, fn direction, {position, visited} ->
      new_pos =
        case direction do
          "^" -> put_in(position.y, position.y - 1)
          ">" -> put_in(position.x, position.x + 1)
          "v" -> put_in(position.y, position.y + 1)
          "<" -> put_in(position.x, position.x - 1)
        end

      {new_pos, MapSet.put(visited, new_pos)}
    end)
  end

  def parse(input), do: String.trim(input) |> String.graphemes()
end
