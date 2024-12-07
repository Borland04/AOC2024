defmodule Day01 do
  def run(input_filename) do
    {list_l, list_r} =
      File.stream!(input_filename, :line)
      |> Stream.map(&parse_line/1)
      |> Enum.reduce({[], []}, fn [l, r], {acc_l, acc_r} -> {acc_l ++ [l], acc_r ++ [r]} end)

    sorted_l = Enum.sort(list_l)
    sorted_r = Enum.sort(list_r)

    result =
      Enum.zip([sorted_l, sorted_r])
      |> Enum.map(fn {l, r} -> abs(l - r) end)
      |> Enum.reduce(&+/2)

    IO.puts(result)
  end

  defp parse_line(line) do
    String.split(line)
    |> Enum.filter(&(&1 != ""))
    |> Enum.map(fn num_str ->
      parse_result = Integer.parse(num_str)

      case parse_result do
        {num, ""} -> num
        _ -> raise "Expected a number"
      end
    end)
  end
end
