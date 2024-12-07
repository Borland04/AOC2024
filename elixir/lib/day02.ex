defmodule Day02 do
  def run(input_filename) do
    reports =
      File.stream!(input_filename, :line)
      |> Stream.map(&parse_line/1)

    result =
      reports
      |> Enum.filter(&safe?/1)
      |> Enum.count()

    IO.puts(result)
  end

  defp safe?(report, graduation \\ :unknown, tolerance_num \\ 0)

  defp safe?(_, _, tolerance_num) when tolerance_num > 1 do
    false
  end

  defp safe?([], _, _) do
    true
  end

  defp safe?([_], _, _) do
    true
  end

  defp safe?([a, b | tail], :unknown, tolerance_num) do
    safe?(
      [a, b | tail],
      if a < b do
        :increasing
      else
        :decreasing
      end,
      tolerance_num
    ) or safe?([a | tail], :unknown, tolerance_num + 1) or
      safe?([b | tail], :unknown, tolerance_num + 1)
  end

  defp safe?([a, b | tail], graduation, tolerance_num) do
    diff =
      case graduation do
        :increasing -> b - a
        :decreasing -> a - b
      end

    in_range = diff in 1..3
    is_safe = safe?([b | tail], graduation, tolerance_num)
    (in_range and is_safe) or safe?([a | tail], graduation, tolerance_num + 1)
  end

  defp safe?(_, _, _) do
    false
  end

  defp parse_line(line) do
    String.split(line)
    |> Enum.map(fn num_str ->
      parse_result = Integer.parse(num_str)

      case parse_result do
        {num, ""} -> num
        _ -> raise "Expected a number, but got: " <> num_str
      end
    end)
  end
end
