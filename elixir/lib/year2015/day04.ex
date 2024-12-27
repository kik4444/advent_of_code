defmodule Aoc.Year2015.Day04 do
  def part1(input) do
    find_number(parse(input), "00000", 1)
  end

  def part2(input) do
    find_number(parse(input), "000000", 1)
  end

  def find_number(input, pattern, number) do
    hex = :crypto.hash(:md5, input <> to_string(number)) |> Base.encode16()

    if String.starts_with?(hex, pattern) do
      number
    else
      find_number(input, pattern, number + 1)
    end
  end

  def parse(input), do: String.trim(input)
end
