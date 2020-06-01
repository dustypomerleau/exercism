defmodule Bob do
  def hey(input) do
    input = String.trim(input)
    question = String.ends_with?(input, "?")
    caps = String.upcase(input) == input
    alphas = String.downcase(input) !== input
    empty = input == ""

    cond do
      question and caps and alphas ->
        "Calm down, I know what I'm doing!"

      caps and alphas ->
        "Whoa, chill out!"

      question ->
        "Sure."

      empty ->
        "Fine. Be that way!"

      true ->
        "Whatever."
    end
  end
end
