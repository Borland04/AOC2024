defmodule Day09 do
  def run(input) do
    defragmented_disk =
      parse_input(input)
      |> unfold_disk()
      |> defragment

    result =
      Stream.zip([Stream.iterate(0, &(&1 + 1)), defragmented_disk])
      |> Enum.map(fn {idx, elem} ->
        case elem do
          {:file, file_id} -> idx * file_id
          _ -> 0
        end
      end)
      |> Enum.sum()

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
    current_block = {:file, next_file_id}

    List.duplicate(current_block, num) ++ unfold_disk(nums, next_block_type, next_file_id)
  end

  def unfold_disk([num | nums], :file, prev_file_id) do
    next_block_type = :empty
    next_file_id = prev_file_id
    current_block = {:empty}

    List.duplicate(current_block, num) ++ unfold_disk(nums, next_block_type, next_file_id)
  end

  def defragment([]) do
    []
  end

  def defragment(disk) do
    {prevs, nexts} = Enum.split_while(disk, &(&1 != {:empty}))

    cond do
      nexts == [] ->
        prevs

      true ->
        [empty_elem | nexts_tail] = nexts
        {last_file_block, new_tail} = remove_last_if(nexts_tail, &(&1 != {:empty}))

        case last_file_block do
          :none -> prevs ++ nexts
          _ -> prevs ++ [last_file_block | defragment(new_tail) ++ [empty_elem]]
        end
    end
  end

  def remove_last_if([], _) do
    {:none, []}
  end

  def remove_last_if([a | as], cond) do
    {removed_elem, tail} = remove_last_if(as, cond)

    if removed_elem != :none do
      {removed_elem, [a | tail]}
    else
      if cond.(a) do
        {a, as}
      else
        {:none, [a | as]}
      end
    end
  end
end
