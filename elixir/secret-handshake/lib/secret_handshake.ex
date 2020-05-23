defmodule SecretHandshake do
  @shake_moves [
    "flip",
    "jump",
    "close your eyes",
    "double blink",
    "wink"
  ]

  @doc """
  Determine the actions of a secret handshake based on the binary
  representation of the given `code`.

  If the following bits are set, include the corresponding action in your list
  of commands, in order from lowest to highest.

  1 = wink
  10 = double blink
  100 = close your eyes
  1000 = jump

  10000 = Reverse the order of the operations in the secret handshake
  """
  @spec commands(code :: integer) :: list(String.t())
  def commands(code) when is_integer(code) do
    code
    |> Integer.to_string(2)
    |> String.pad_leading(5, "0")
    |> String.split("")
    |> Enum.drop(-1)
    |> Enum.take(-5)
    |> Enum.map(&String.to_integer/1)
    |> Enum.with_index()
    |> Enum.map(&get_shake/1)
    |> flip()
    |> Enum.filter(fn x -> !is_nil(x) end)
  end

  defp get_shake({code, index}) do
    if code == 1 do
      Enum.at(@shake_moves, index)
    end
  end

  defp flip(shake_list) do
    if Enum.at(shake_list, 0) == "flip" do
      {_, shake_list} = List.pop_at(shake_list, 0)
      shake_list
    else
      Enum.reverse(shake_list)
    end
  end
end
