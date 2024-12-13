defmodule Mix.Tasks.Aoc.Run do
  @moduledoc "Run a given AOC2024 day and part with the real or example input"
  @shortdoc "Run with mix aoc.run 01/1/e or mix aoc.run 01/2/"

  use Mix.Task

  @impl true
  def run([<<day_num::binary-size(2), "/", part::binary-size(1), "/", input::binary>>]) do
    module = Module.concat([:Aoc2024, "Day#{day_num}"])

    file_suffix =
      case input do
        "" -> "txt"
        "e" <> rest when rest in ["", "01"] -> "example01.txt"
        "e02" -> "example02.txt"
      end

    input_path = :code.priv_dir(:aoc2024) |> Path.join("day#{day_num}.#{file_suffix}")

    IO.inspect(apply(module, :"part#{part}", [input_path]))
  end
end
