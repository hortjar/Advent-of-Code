defmodule AoC2022.Day04 do
  import AoC2022.Common

  def part01 do
    get_numbers()
    |> Enum.count(&does_fully_overlap?/1)
  end

  def part02 do
    get_numbers()
    |> Enum.count(&does_some_overlap?/1)
  end

  def get_numbers() do
    parse()
    |> Enum.map(&String.split(&1, ","))
    |> Enum.map(&parse_numbers/1)
  end

  def parse_numbers([l, r]) do
    l = l |> String.split("-") |> Enum.map(&String.to_integer/1)
    r = r |> String.split("-") |> Enum.map(&String.to_integer/1)
    [l, r]
  end

  def does_some_overlap?([[l1, l2], [r1, r2]]) do
    !MapSet.disjoint?(MapSet.new(l1..l2), MapSet.new(r1..r2))
  end

  def does_fully_overlap?([[l1, l2], [r1, r2]]) do
    cond do
      l1 >= r1 && l2 <= r2 ->
        true

      r1 >= l1 && r2 <= l2 ->
        true

      true ->
        false
    end
  end
end
