defmodule Aoc.Year2015.Day01 do
  def part1(input) do
    Enum.reduce(parse(input), 0, fn
      "(", acc -> acc + 1
      ")", acc -> acc - 1
    end)
  end

  def part2(input) do
    Enum.reduce_while(parse(input), {0, 1}, fn char, {sum, index} ->
      sum = sum + if char == "(", do: 1, else: -1

      if sum == -1, do: {:halt, index}, else: {:cont, {sum, index + 1}}
    end)
  end

  def parse(input) do
    String.trim(input) |> String.graphemes()
  end
end
