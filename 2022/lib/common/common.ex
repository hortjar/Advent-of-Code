defmodule AoC2022.Common do
  defmacro parse do
    quote do
      __ENV__.file
      |> Path.dirname()
      |> File.cd!()

      File.read!('input.txt')
      |> String.split("\n", trim: true)
    end
  end
end
