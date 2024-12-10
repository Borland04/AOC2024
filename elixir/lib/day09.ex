defmodule Day09 do
  def run(input) do
    defragmented_disk =
      parse_input(input)
      |> unfold_disk()
      |> defragment

    result = calculate_result(defragmented_disk, 0)

    IO.puts(result)
  end

  def parse_input(input) do
    input
    |> Stream.flat_map(fn line ->
      String.graphemes(line)
    end)
    |> Stream.map(&String.trim/1)
    |> Stream.filter(&(&1 != ""))
    |> Stream.map(&Integer.parse/1)
    |> Enum.map(fn {digit, _} -> digit end)
  end

  def unfold_disk(disk_data, prev_block_type \\ :empty, prev_file_id \\ -1)

  def unfold_disk([], _, _) do
    []
  end

  def unfold_disk([num | nums], :empty, prev_file_id) do
    next_block_type = :file
    next_file_id = prev_file_id + 1
    current_block = {:file, next_file_id, num}

    [current_block | unfold_disk(nums, next_block_type, next_file_id)]
  end

  def unfold_disk([num | nums], :file, prev_file_id) do
    next_block_type = :empty
    next_file_id = prev_file_id
    current_block = {:empty, num}

    [current_block | unfold_disk(nums, next_block_type, next_file_id)]
  end

  def defragment([]) do
    []
  end

  def defragment(disk) do
    {head_with_file_at_right, empty_tail} =
      split_while_right(disk, fn item ->
        case item do
          {:empty, _} -> true
          _ -> false
        end
      end)

    if head_with_file_at_right == [] do
      empty_tail
    else
      last_file = List.last(head_with_file_at_right)
      {:file, _, file_size} = last_file

      without_last_file =
        List.delete_at(head_with_file_at_right, length(head_with_file_at_right) - 1)

      {split_heads, split_tail} =
        Enum.split_while(without_last_file, fn elem ->
          case elem do
            {:empty, empty_size} when empty_size >= file_size -> false
            _ -> true
          end
        end)

      case split_tail do
        [] ->
          defragment(without_last_file) ++ [last_file | empty_tail]

        [{:empty, s} | others] ->
          rem_empty_size = s - file_size

          rem_empty =
            if rem_empty_size == 0 do
              []
            else
              [{:empty, rem_empty_size}]
            end

          defragment(
            split_heads ++
              [last_file] ++
              rem_empty ++
              others
          ) ++ [{:empty, file_size} | empty_tail]
      end
    end
  end

  def split_while_right([], _) do
    {[], []}
  end

  def split_while_right([a | as], cond) do
    {sub_left, sub_right} = split_while_right(as, cond)

    if sub_left == [] do
      if cond.(a) do
        {[], [a | sub_right]}
      else
        {[a], sub_right}
      end
    else
      {[a | sub_left], sub_right}
    end
  end

  def calculate_result([], _) do
    0
  end

  def calculate_result([{:file, file_idx, file_size} | tail], idx) do
    file_checksum =
      idx..(idx + (file_size - 1))
      |> Enum.map(&(&1 * file_idx))
      |> Enum.sum()

    file_checksum + calculate_result(tail, idx + file_size)
  end

  def calculate_result([{:empty, empty_size} | tail], idx) do
    calculate_result(tail, idx + empty_size)
  end
end
