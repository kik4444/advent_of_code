defmodule Aoc.Year2015.Day02 do
  def part1(input) do
    parse(input) |> Enum.map(&area/1) |> Enum.sum()
  end

  def part2(input) do
    parse(input) |> Enum.map(&ribbons/1) |> Enum.sum()
  end

  def area([l, w, h]) do
    first = l * w
    second = w * h
    third = h * l

    min = min(first, min(second, third))

    2 * first + 2 * second + 2 * third + min
  end

  def ribbons([l, w, h] = dimensions) do
    [first, second, _] = Enum.sort(dimensions)

    ribbon = first * 2 + second * 2
    bow = l * w * h

    ribbon + bow
  end

  def parse(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn line -> String.split(line, "x") |> Enum.map(&String.to_integer/1) end)
  end
end
