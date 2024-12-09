defmodule Aoc2024.Day01 do
  def part1(path) do
    {left_list, right_list} = parse(path)

    [Enum.sort(left_list), Enum.sort(right_list)]
    |> Enum.zip_reduce(0, fn [left, right], acc -> acc + abs(left - right) end)
  end

  def part2(path) do
    {left_list, right_list} = parse(path)

    freq = Enum.frequencies(right_list)

    Enum.reduce(left_list, 0, fn num, acc -> acc + num * (freq[num] || 0) end)
  end

  def parse(path) when is_binary(path) do
    File.read!(path)
    |> String.split("\n", trim: true)
    |> Stream.map(&Regex.split(~r/\s+/, &1))
    |> Stream.map(fn [left, right] ->
      {String.to_integer(left), String.to_integer(right)}
    end)
    |> Enum.unzip()
  end
end
