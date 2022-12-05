defmodule AoC2022.Day02 do
  def scores_r do
    %{
      "A X" => 1 + 3,
      "A Y" => 2 + 6,
      "A Z" => 3 + 0,
      "B X" => 1 + 0,
      "B Y" => 2 + 3,
      "B Z" => 3 + 6,
      "C X" => 1 + 6,
      "C Y" => 2 + 0,
      "C Z" => 3 + 3
    }
  end

  def scores_l do
    %{
      "A X" => 3 + 0,
      "A Y" => 1 + 3,
      "A Z" => 2 + 6,
      "B X" => 1 + 0,
      "B Y" => 2 + 3,
      "B Z" => 3 + 6,
      "C X" => 2 + 0,
      "C Y" => 3 + 3,
      "C Z" => 1 + 6
    }
  end

  def part01 do
    solve(&scores_r/0)
  end

  def part02 do
    solve(&scores_l/0)
  end

  def solve(scores) do
    parse()
    |> Enum.map(fn s -> scores.()[s] end)
    |> Enum.sum()
  end

  def parse do
    __ENV__.file
    |> Path.dirname()
    |> File.cd!()

    File.read!('input.txt')
    |> String.split("\n", trim: true)
  end
end
