defmodule Day11 do
  def run(input) do
    stones = parse_input(input)

    result = calculate_result(stones)

    IO.puts(result)
  end

  def parse_input(input) do
    input
    |> Stream.flat_map(&String.split/1)
    |> Stream.map(&String.trim/1)
    |> Enum.filter(&(&1 != ""))
  end

  def calculate_result(stones, step \\ 0)

  def calculate_result(stones, step) when step >= 25 do
    length(stones)
  end

  def calculate_result(stones, step) do
    stones
    |> Enum.flat_map(&blink/1)
    # |> IO.inspect()
    |> calculate_result(step + 1)
  end

  def blink(stone) do
    cond do
      stone == "0" ->
        ["1"]

      rem(String.length(stone), 2) == 0 ->
        half_length = div(String.length(stone), 2)
        {left_str, right_str} = String.split_at(stone, half_length)
        {left_num, ""} = Integer.parse(left_str)
        {right_num, ""} = Integer.parse(right_str)

        [Integer.to_string(left_num), Integer.to_string(right_num)]

      true ->
        {num, ""} = Integer.parse(stone)
        [Integer.to_string(num * 2024)]
    end
  end
end
