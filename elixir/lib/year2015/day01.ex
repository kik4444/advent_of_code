defmodule Aoc.Year2015.Day01 do
  def part1(input) do
    parse(input)
    |> Enum.reduce(0, fn
      "(", acc -> acc + 1
      ")", acc -> acc - 1
    end)
  end

  def part2(input) do
    parse(input)
    |> Enum.reduce_while({0, 0}, fn char, {sum, index} ->
      index = index + 1

      sum = sum + if char == "(", do: 1, else: -1

      if sum == -1, do: {:halt, index}, else: {:cont, {sum, index}}
    end)
  end

  def parse(input) do
    input |> String.trim() |> String.graphemes()
  end
end
