defmodule Aoc2024.Day02 do
  def part1(path) do
    parse(path)
    |> Stream.map(fn [first_report | rest] ->
      Enum.reduce_while(rest, {first_report, nil}, &validate/2)
    end)
    |> Enum.count(&(&1 !== false))
  end

  def part2(path) do
    parse(path)
  end

  def parse(path) when is_binary(path) do
    File.read!(path)
    |> String.split("\n", trim: true)
    |> Enum.map(fn level -> String.split(level) |> Enum.map(&String.to_integer/1) end)
  end

  def validate(curr_report, {prev_report, nil}) do
    cond do
      not safe_distance?(prev_report, curr_report) -> {:halt, false}
      prev_report > curr_report -> {:cont, {curr_report, :dec}}
      prev_report < curr_report -> {:cont, {curr_report, :inc}}
    end
  end

  def validate(curr_report, {prev_report, :inc}) do
    cond do
      not safe_distance?(prev_report, curr_report) -> {:halt, false}
      prev_report > curr_report -> {:halt, false}
      prev_report < curr_report -> {:cont, {curr_report, :inc}}
    end
  end

  def validate(curr_report, {prev_report, :dec}) do
    cond do
      not safe_distance?(prev_report, curr_report) -> {:halt, false}
      prev_report < curr_report -> {:halt, false}
      prev_report > curr_report -> {:cont, {curr_report, :dec}}
    end
  end

  def safe_distance?(left, right), do: left !== right and abs(right - left) in 1..3
end
