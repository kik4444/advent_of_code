defmodule Aoc.Year2024.Day02 do
  def part1(input) do
    parse(input) |> Enum.count(&safe_level?/1)
  end

  def part2(input) do
    parse(input) |> Enum.count(fn level -> tolerated_permutations(level) |> Enum.any?(&safe_level?/1) end)
  end

  def parse(input) when is_binary(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn level -> String.split(level) |> Enum.map(&String.to_integer/1) end)
  end

  def safe_level?(level) when is_list(level), do: safe_level?(level, find_tolerance(level))

  def safe_level?([_last_report], _), do: true

  def safe_level?([report1 | [report2 | _] = rest], tolerance) do
    if (report2 - report1) in tolerance do
      safe_level?(rest, tolerance)
    else
      false
    end
  end

  def find_tolerance([report1, report2 | _]), do: if(report1 < report2, do: 1..3, else: -3..-1)

  def tolerated_permutations(level) when is_list(level), do: perms(level, [], [])

  def perms([], _, level_permutations), do: Enum.reverse(level_permutations)

  def perms([current_report | remaining_reports], used_reports, level_permutations) do
    level_without_current_report = Enum.reverse(used_reports) ++ remaining_reports

    perms(remaining_reports, [current_report | used_reports], [level_without_current_report | level_permutations])
  end
end
