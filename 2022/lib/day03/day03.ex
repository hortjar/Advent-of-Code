defmodule AoC2022.Day03 do
  import AoC2022.Common

  def part01 do
    parse()
    |> Enum.map(&divide/1)
    |> Enum.map(&matches/1)
    |> sum_scores()
  end

  def part02 do
    parse()
    |> Enum.chunk_every(3)
    |> Enum.map(&matches_group/1)
    |> sum_scores()
  end

  def sum_scores(lines) do
    lines
    |> Enum.flat_map(fn x -> x end)
    |> Enum.map(fn s -> :binary.first(s) end)
    |> Enum.map(&score/1)
    |> Enum.sum()
  end

  def score(ascii) do
    cond do
      ascii >= 65 && ascii <= 90 -> ascii - 38
      ascii >= 97 && ascii <= 122 -> ascii - 96
    end
  end

  def divide(string) do
    String.split_at(string, div(String.length(string), 2))
  end

  def matches(tuple) do
    first = tuple |> elem(0) |> String.codepoints()
    second = tuple |> elem(1) |> String.codepoints()

    first
    |> Enum.filter(fn el -> Enum.member?(second, el) end)
    |> Enum.uniq()
  end

  def matches_group(group) do
    first = group |> Enum.at(0) |> String.codepoints()
    second = group |> Enum.at(1) |> String.codepoints()
    third = group |> Enum.at(2) |> String.codepoints()

    first
    |> Enum.filter(fn el -> Enum.member?(second, el) && Enum.member?(third, el) end)
    |> Enum.uniq()
  end
end
