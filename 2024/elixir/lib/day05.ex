defmodule Aoc2024.Day05 do
  def part1(input) do
    {ordering, updates} = parse(input)

    updates
    |> Stream.filter(&valid_update?(&1, ordering))
    |> Stream.map(&middle/1)
    |> Enum.sum()
  end

  def part2(input) do
    {ordering, updates} = parse(input)

    updates
    |> Stream.reject(&valid_update?(&1, ordering))
    |> Stream.map(fn update -> Enum.sort(update, &left_before_right?(&1, &2, ordering)) end)
    |> Stream.map(&middle/1)
    |> Enum.sum()
  end

  def parse(input) do
    [ordering_list, updates_list] = String.split(input, "\n\n")

    parsed_ordering =
      ordering_list
      |> String.split("\n")
      |> Stream.map(&String.split(&1, "|"))
      |> Stream.map(fn [left, right] -> {String.to_integer(left), String.to_integer(right)} end)
      |> Enum.reduce(%{}, fn {left, right}, acc ->
        pages_following_left = acc[left] || []
        put_in(acc[left], [right | pages_following_left])
      end)

    parsed_updates =
      updates_list
      |> String.split("\n", trim: true)
      |> Enum.map(fn update -> String.split(update, ",") |> Enum.map(&String.to_integer/1) end)

    {parsed_ordering, parsed_updates}
  end

  def valid_update?([_last_page], _), do: true

  def valid_update?([head | tail], %{} = ordering) do
    if ordering[head] && Enum.all?(tail, &Enum.member?(ordering[head], &1)) do
      valid_update?(tail, ordering)
    else
      false
    end
  end

  def middle(list) when is_list(list), do: Enum.at(list, length(list) |> div(2))

  def left_before_right?(left, right, %{} = ordering) do
    (ordering[left] && Enum.member?(ordering[left], right)) || false
  end
end
