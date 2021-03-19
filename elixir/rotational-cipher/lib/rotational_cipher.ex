defmodule RotationalCipher do
  @doc """
  Given a plaintext and amount to shift by, return a rotated string.

  Example:
  iex> RotationalCipher.rotate("Attack at dawn", 13)
  "Nggnpx ng qnja"
  """
  @spec rotate(text :: String.t(), shift :: integer) :: String.t()
  def rotate(text, shift) do
    alphas = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    alpha_list = String.split(alphas, "", trim: true)
    # with_index? probably not, map is better, try zipping with a range

    create a map of the alphabet
    ?
    %{"a" => "b"}
  end
end

# Integer.mod(dividend, divisor)
# computes the modulo remainder of an integer division
