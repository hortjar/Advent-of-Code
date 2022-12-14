defmodule AoC2022.Day01 do
  @moduledoc """
  Documentation for `AoC2022.Day01`.
  """
  import AoC2022.Common, except: [parse: 0]

  def part01 do
    parse()
    |> Enum.max()
  end

  def part02 do
    parse()
    |> Enum.sort(:desc)
    |> Enum.chunk_every(3)
    |> Enum.at(0)
    |> Enum.sum()
  end

  def parse do
    __ENV__.file
    |> Path.dirname()
    |> File.cd!()

    File.read!('input.txt')
    |> String.split("\n\n")
    |> Enum.map(&String.split/1)
    |> Enum.map(fn x -> Enum.map(x, fn y -> String.to_integer(y) end) end)
    |> Enum.map(&Enum.sum/1)
  end
end
