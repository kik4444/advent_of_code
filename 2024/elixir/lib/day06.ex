defmodule Aoc2024.Day06.Lab do
  defstruct grid: {}, width: 0, height: 0, x: 0, y: 0, direction: :up
end

defmodule Aoc2024.Day06 do
  alias Aoc2024.Day06.Lab

  def part1(input) do
    %Lab{} = lab = parse(input)
    uniq_pos = MapSet.new()

    walk(lab, uniq_pos) |> MapSet.size()
  end

  def walk(%Lab{} = lab, %MapSet{} = uniq_pos) do
    uniq_pos = MapSet.put(uniq_pos, {lab.x, lab.y})

    look_around(lab, uniq_pos)
  end

  def look_around(%Lab{} = lab, uniq_pos) do
    case lab.direction do
      :up when lab.y == 0 -> uniq_pos
      :right when lab.x == lab.width -> uniq_pos
      :down when lab.y == lab.height -> uniq_pos
      :left when lab.x == 0 -> uniq_pos
      _ -> move(lab, uniq_pos)
    end
  end

  def move(%Lab{direction: :up} = lab, uniq_pos) do
    next_cell = at(lab.grid, lab.x, lab.y - 1)
    if next_cell == "#", do: turn_right(lab, uniq_pos), else: walk(%Lab{lab | y: lab.y - 1}, uniq_pos)
  end

  def move(%Lab{direction: :right} = lab, uniq_pos) do
    next_cell = at(lab.grid, lab.x + 1, lab.y)
    if next_cell == "#", do: turn_right(lab, uniq_pos), else: walk(%Lab{lab | x: lab.x + 1}, uniq_pos)
  end

  def move(%Lab{direction: :down} = lab, uniq_pos) do
    next_cell = at(lab.grid, lab.x, lab.y + 1)
    if next_cell == "#", do: turn_right(lab, uniq_pos), else: walk(%Lab{lab | y: lab.y + 1}, uniq_pos)
  end

  def move(%Lab{direction: :left} = lab, uniq_pos) do
    next_cell = at(lab.grid, lab.x - 1, lab.y)
    if next_cell == "#", do: turn_right(lab, uniq_pos), else: walk(%Lab{lab | x: lab.x - 1}, uniq_pos)
  end

  def turn_right(%Lab{direction: :up} = lab, uniq_pos), do: walk(put_in(lab.direction, :right), uniq_pos)
  def turn_right(%Lab{direction: :right} = lab, uniq_pos), do: walk(put_in(lab.direction, :down), uniq_pos)
  def turn_right(%Lab{direction: :down} = lab, uniq_pos), do: walk(put_in(lab.direction, :left), uniq_pos)
  def turn_right(%Lab{direction: :left} = lab, uniq_pos), do: walk(put_in(lab.direction, :up), uniq_pos)

  def parse(input) do
    grid =
      input
      |> String.split("\n", trim: true)
      |> Enum.map(&String.graphemes/1)
      |> Enum.map(&List.to_tuple/1)
      |> List.to_tuple()

    height = tuple_size(grid) - 1
    width = tuple_size(elem(grid, 0)) - 1

    [{guard_x, guard_y}] = for x <- 0..width, y <- 0..height, at(grid, x, y) == "^", do: {x, y}

    %Lab{grid: grid, width: width, height: height, x: guard_x, y: guard_y}
  end

  def at(grid, x, y), do: grid |> elem(y) |> elem(x)
end
