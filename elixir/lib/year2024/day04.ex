defmodule Aoc.Year2024.Day04 do
  def part1(input) do
    {grid, width, height} = parse(input)

    # "d" means delta.
    for row <- width,
        col <- height,
        drow <- -1..1,
        dcol <- -1..1,
        (row + drow * 3) in width,
        (col + dcol * 3) in height,
        at(grid, row, col) == "X",
        reduce: 0 do
      acc ->
        acc + if xmas?(grid, row, col, drow, dcol), do: 1, else: 0
    end
  end

  def part2(input) do
    {grid, width, height} = parse(input)

    # Not interested in "A"s at the start or end of a row or column
    # because they can't have any neighbors.
    for row <- width,
        col <- height,
        (row + 1) in width and (row - 1) in width,
        (col + 1) in height and (col - 1) in height,
        at(grid, row, col) == "A",
        reduce: 0 do
      acc ->
        acc + if x_mas?(grid, row, col), do: 1, else: 0
    end
  end

  def parse(input) do
    grid =
      input
      |> String.split("\n", trim: true)
      |> Enum.map(fn line -> String.graphemes(line) |> List.to_tuple() end)
      |> List.to_tuple()

    width = tuple_size(elem(grid, 0))
    height = tuple_size(grid)

    {grid, 0..(width - 1), 0..(height - 1)}
  end

  def at(grid, row, col), do: grid |> elem(col) |> elem(row)

  def xmas?(grid, row, col, drow, dcol) do
    Enum.map(1..3, &at(grid, row + &1 * drow, col + &1 * dcol)) == ~w[M A S]
  end

  def x_mas?(grid, row, col) do
    backward_diagonal =
      Enum.sort([
        at(grid, row + 1, col + 1),
        at(grid, row - 1, col - 1)
      ])

    forward_diagonal =
      Enum.sort([
        at(grid, row + 1, col - 1),
        at(grid, row - 1, col + 1)
      ])

    forward_diagonal == ~w[M S] and backward_diagonal == ~w[M S]
  end
end
