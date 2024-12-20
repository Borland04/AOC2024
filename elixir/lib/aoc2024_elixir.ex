defmodule Aoc2024Elixir do
  @moduledoc """
  Documentation for `Aoc2024Elixir`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Aoc2024Elixir.hello()
      :world

  """
  require Day01
  require Day02
  require Day07
  require Day09
  require Day11

  def start(_type, _args) do
    # IO.puts("## Day 01 ##")
    # Day01.run("../inputs/01.txt")
    # IO.puts("############")

    # IO.puts("## Day 02 ##")
    # Day02.run("../inputs/02.txt")
    # IO.puts("############")

    # IO.puts("## Day 07 ##")
    # Day07.run(File.stream!("../inputs/07.txt", :line))
    # IO.puts("############")

    # IO.puts("## Day 09 ##")
    # Day09.run(File.stream!("../inputs/09.txt", :line))
    # IO.puts("############")

    IO.puts("## Day 11 ##")
    Day11.run(File.stream!("../inputs/11.txt", :line))
    IO.puts("############")

    {:ok, self()}
  end
end
