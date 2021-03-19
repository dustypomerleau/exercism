defmodule RomanNumerals do
  @digits [
    {1000, "M"},
    {900, "CM"},
    {500, "D"},
    {400, "CD"},
    {100, "C"},
    {90, "XC"},
    {50, "L"},
    {40, "XL"},
    {10, "X"},
    {9, "IX"},
    {5, "V"},
    {4, "IV"},
    {1, "I"}
  ]
  @doc """
  Convert the number to a roman number.
  """
  @spec numeral(pos_integer) :: String.t()
  def numeral(n), do: do_numeral(n, [])

  defp do_numeral(0, list), do: List.to_string(list)

  for {num, roman} <- @digits do
    defp do_numeral(n, list) when n >= unquote(num),
      do: do_numeral(n - unquote(num), [list | unquote(roman)])
  end
end
