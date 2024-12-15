defmodule Aoc2024.Day03 do
  def part1(input) do
    parse_valid_muls(input)
  end

  def part2(input) do
    find_muls(input, :do, [])
    |> Stream.map(&parse_valid_muls/1)
    |> Enum.sum()
  end

  # Get all text up to the next "don't()",
  # or stop recursing if near the end.
  def find_muls(input, :do, acc) do
    case Regex.run(~r/mul\(\d+,\d+\).*?(?:don't\(\)|$)/s, input, return: :index) do
      [{index, length}] ->
        do_text = String.slice(input, index, length)

        {_, remaining_text} = String.split_at(input, index + length)

        find_muls(remaining_text, :dont, [do_text | acc])

      nil ->
        acc
    end
  end

  # Skip all text up to the next "do()".
  def find_muls(input, :dont, acc) do
    [{index, length}] = Regex.run(~r/.*?(?:do\(\)|$)/, input, return: :index)

    {_, remaining_text} = String.split_at(input, index + length)

    find_muls(remaining_text, :do, acc)
  end

  def parse_valid_muls(input) when is_binary(input) do
    Regex.scan(~r/mul\((\d+,\d+)\)/, input)
    |> Stream.map(fn [_mul, numbers] ->
      numbers
      |> String.split(",")
      |> Stream.map(&String.to_integer/1)
      |> Enum.product()
    end)
    |> Enum.sum()
  end
end
