defmodule Aoc2024.Day01 do
  def part1(input) do
    {left_list, right_list} = parse(input)

    [Enum.sort(left_list), Enum.sort(right_list)]
    |> Enum.zip_reduce(0, fn [left, right], acc -> acc + abs(left - right) end)
  end

  def part2(input) do
    {left_list, right_list} = parse(input)

    freq = Enum.frequencies(right_list)

    Enum.reduce(left_list, 0, fn num, acc -> acc + num * (freq[num] || 0) end)
  end

  def parse(input) do
    input
    |> String.split("\n", trim: true)
    |> Stream.map(&Regex.split(~r/\s+/, &1))
    |> Stream.map(fn [left, right] ->
      {String.to_integer(left), String.to_integer(right)}
    end)
    |> Enum.unzip()
  end
end
