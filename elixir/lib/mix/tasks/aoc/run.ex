defmodule Mix.Tasks.Aoc.Run do
  @moduledoc "Run with e.g. `mix aoc.run -y 2024 -d 1 -p 1`"
  @shortdoc @moduledoc

  use Mix.Task

  @impl true
  def run(args) do
    args = :argparse.run(Enum.map(args, &to_charlist/1), arguments(), %{progname: :aoc})

    input_suffix =
      case args.input do
        "" -> "txt"
        "e" <> rest when rest in ["", "1"] -> "example01.txt"
        "e2" -> "example02.txt"
        other -> raise "Unknown input #{other}"
      end

    day_string = args.day |> Integer.to_string() |> String.pad_leading(2, "0") |> then(&"day#{&1}")

    input =
      Path.join([:code.priv_dir(:aoc), "input", Integer.to_string(args.year), "#{day_string}.#{input_suffix}"])
      |> File.read!()

    module = Module.concat([:Aoc, "Year#{args.year}", String.capitalize(day_string)])

    IO.inspect(apply(module, :"part#{args.part}", [input]))
  end

  defp arguments do
    %{
      arguments: [
        %{
          name: :year,
          type: :integer,
          required: true,
          short: ?y,
          long: ~c"year",
          help: ~c"Year to run"
        },
        %{
          name: :day,
          type: {:integer, Enum.to_list(1..25)},
          required: true,
          short: ?d,
          long: ~c"day",
          help: ~c"Day in year to run."
        },
        %{
          name: :part,
          type: {:integer, [1, 2]},
          required: true,
          short: ?p,
          long: ~c"part",
          help: ~c"Part of day to run."
        },
        %{
          name: :input,
          type: :binary,
          default: "",
          help: ~c"""
            Which input to use.
            Use "e" or "e1" for "day{day}.example01.txt", "e2" for the second example if present,
            or leave empty to use the real input.
          """
        }
      ],
      handler: & &1,
      help: ~c"Run a given advent of code solution"
    }
  end
end
