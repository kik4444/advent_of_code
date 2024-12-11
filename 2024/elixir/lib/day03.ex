defmodule Aoc2024.Day03 do
  def part1(path) do
    parse_valid_muls(File.read!(path))
  end

  def part2(path) do
    find_muls(File.read!(path), :do, [])
    |> Stream.map(&parse_valid_muls/1)
    |> Enum.sum()
  end

  # Get all text up to the next "don't()",
  # or stop recursing if near the end.
  def find_muls(input, :do, acc) do
    with [{index, length}] <- Regex.run(~r/mul\(\d+,\d+\).*?(?:don't\(\)|$)/s, input, return: :index) do
      do_text = String.slice(input, index, length)

      remaining_text = String.slice(input, (index + length)..-1//1)

      find_muls(remaining_text, :dont, [do_text | acc])
    else
      nil -> acc
    end
  end

  # Skip all text up to the next "do()".
  def find_muls(input, :dont, acc) do
    [{index, length}] = Regex.run(~r/.*?(?:do\(\)|$)/, input, return: :index)

    remaining_text = String.slice(input, (index + length)..-1//1)

    find_muls(remaining_text, :do, acc)
  end

  def parse_valid_muls(input) when is_binary(input) do
    Regex.scan(~r/mul\((\d+,\d+)\)/, input)
    |> Stream.map(fn [_mul, numbers] ->
      String.split(numbers, ",") |> Stream.map(&String.to_integer/1) |> Enum.product()
    end)
    |> Enum.sum()
  end
end
