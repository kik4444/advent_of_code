defmodule Aoc.Year2015.Day05 do
  def part1(input) do
    parse(input) |> Enum.count(fn str -> vowels?(str) and double_letter?(str) and no_bad_letters?(str) end)
  end

  def part2(input) do
    parse(input) |> Enum.count(fn str -> has_pair?(str) and has_inbetween_repeat?(str) end)
  end

  def vowels?(str), do: Regex.scan(~r/[aeiou]/, str) |> length() |> then(&(&1 >= 3))

  def double_letter?(str) do
    str
    |> String.graphemes()
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.any?(fn
      [x, x] -> true
      [_x, _y] -> false
    end)
  end

  def no_bad_letters?(str), do: not Regex.match?(~r/ab|cd|pq|xy/, str)

  def has_pair?(str) do
    str
    |> String.graphemes()
    |> Enum.chunk_every(2, 1)
    |> Enum.with_index()
    |> Enum.reduce_while(%{}, fn
      {[_last], _}, _ ->
        {:halt, false}

      # We have to ignore overlapping character pairs.
      # For example, "aaa" would be matched twice. First "(aa)a" with index 0, then "a(aa)" with index 1.
      # If the previous match's index plus 1 equals the current index, then we have an overlap and should ignore it.
      {[_, _] = pair, index}, seen ->
        with {:ok, prev_index} when prev_index + 1 != index <- Map.fetch(seen, pair) do
          {:halt, true}
        else
          _ -> {:cont, put_in(seen[pair], index)}
        end
    end)
  end

  def has_inbetween_repeat?(str) do
    str
    |> String.graphemes()
    |> Enum.chunk_every(3, 1, :discard)
    |> Enum.any?(fn
      [x, _y, x] -> true
      [_x, _y, _z] -> false
    end)
  end

  def parse(input), do: String.split(input, "\n", trim: true)
end
