defmodule Day07 do
  def run(input) do
    equations = parse_input(input)

    result =
      equations
      |> Enum.filter(fn {val, args} -> solvable?(val, args) end)
      |> Enum.map(fn {val, _} -> val end)
      |> Enum.sum()

    IO.puts(result)
  end

  def parse_input(input) do
    input
    |> Stream.map(fn line ->
      [val_str, args_str] = String.split(line, ":")
      val = val_str |> Integer.parse() |> then(fn {num, _} -> num end)

      args =
        args_str
        |> String.trim()
        |> String.split()
        |> Enum.map(fn arg ->
          arg |> String.trim() |> Integer.parse() |> then(fn {num, _} -> num end)
        end)

      {val, args}
    end)
    |> Enum.to_list()
  end

  def solvable?(expected, args, acc \\ :empty)

  def solvable?(_, [], :empty) do
    false
  end

  def solvable?(expected, [a | as], :empty) do
    solvable?(expected, as, a)
  end

  def solvable?(expected, [], result) do
    expected == result
  end

  def solvable?(expected, _, acc) when acc > expected do
    false
  end

  def solvable?(expected, [a | as], acc) do
    ops = [
      fn a, b -> a + b end,
      fn a, b -> a * b end,
      fn a, b ->
        b_digits_count =
          b
          |> Integer.to_string()
          |> String.length()

        a_shifted =
          1..b_digits_count
          |> Enum.reduce(a, fn _, acc -> acc * 10 end)

        a_shifted + b
      end
    ]

    ops |> Enum.any?(fn op -> solvable?(expected, as, op.(acc, a)) end)
  end
end
