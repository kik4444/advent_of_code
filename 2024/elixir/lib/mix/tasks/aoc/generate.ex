defmodule Mix.Tasks.Aoc.Generate do
  @moduledoc "Generate a new AOC day with mix aoc.generate `num` where `num` is 01, 02, etc."
  @shortdoc "Run with mix aoc.generate 04"

  use Mix.Task

  @impl true
  def run([day_str]) do
    day_num = String.to_integer(day_str)
    if day_num not in 1..25, do: raise("day_num #{day_num} not in range 1..25")

    day = Integer.to_string(day_num) |> String.pad_leading(2, "0")

    module = Module.concat([:Aoc2024, "Day#{day}"])
    if Code.ensure_loaded?(module), do: raise("module #{module} already exists")

    cwd = File.cwd!()
    priv_dir = Path.join(cwd, "priv")
    input_txt = Path.join(priv_dir, "day#{day}.txt")
    example_txt = Path.join(priv_dir, "day#{day}.example01.txt")
    file = Path.join([cwd, "lib", "day#{day}.ex"])

    File.write!(
      file,
      """
      defmodule Aoc2024.Day#{day} do
        def part1(path) do
        # TODO
      end

      def part2(path) do
        # TODO
        end
      end
      """,
      [:write]
    )

    for input <- [input_txt, example_txt], do: File.touch!(input)
  end
end
