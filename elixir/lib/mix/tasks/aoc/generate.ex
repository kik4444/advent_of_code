defmodule Mix.Tasks.Aoc.Generate do
  @moduledoc "Run with e.g. `mix aoc.generate -y 2024 -d 1`"
  @shortdoc @moduledoc

  use Mix.Task

  @impl true
  def run(args) do
    args = :argparse.run(Enum.map(args, &to_charlist/1), arguments(), %{progname: :aoc})

    day_string = args.day |> Integer.to_string() |> String.pad_leading(2, "0") |> then(&"day#{&1}")

    module = Module.concat([:Aoc, "Year#{args.year}", String.capitalize(day_string)])
    if Code.ensure_loaded?(module), do: raise("Module #{module} already exists")

    cwd = File.cwd!()
    input_dir = Path.join([cwd, "priv", "input", Integer.to_string(args.year)])

    ex_file = Path.join([cwd, "lib", "year#{args.year}", "#{day_string}.ex"])
    input_file = Path.join(input_dir, "#{day_string}.txt")
    example_input_file = Path.join(input_dir, "#{day_string}.example01.txt")

    File.write!(
      ex_file,
      """
      defmodule Aoc.Year#{args.year}.#{String.capitalize(day_string)} do
        def part1(input) do
          # TODO
        end

        def part2(input) do
          # TODO
        end

        def parse(input) do
          # TODO
        end
      end
      """,
      [:write]
    )

    for file <- [input_file, example_input_file], do: File.touch!(file)
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
        }
      ],
      handler: & &1,
      help: ~c"Generate an advent of code day"
    }
  end
end
