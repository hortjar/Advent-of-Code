defmodule AoC2022.Day03 do
  def part01 do
    parse()
    |> Enum.map(&divide/1)
    |> Enum.map(&matches/1)
    |> Enum.flat_map(fn x -> x end)
    |> Enum.map(fn s -> :binary.first(s) end)
    |> Enum.map(&score/1)
    |> Enum.sum()
  end

  def part02 do
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
    first = elem(tuple, 0) |> String.codepoints()
    second = elem(tuple, 1) |> String.codepoints()

    Enum.filter(first, fn el -> Enum.member?(second, el) end)
    |> Enum.uniq()
  end

  def parse do
    __ENV__.file
    |> Path.dirname()
    |> File.cd!()

    File.read!('input.txt')
    |> String.split("\n", trim: true)
  end
end
