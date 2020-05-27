defmodule Bob do
  def hey(input) do
    cond do
      Regex.match?(~r/^[A-Z][^a-z]*\?$/u, input) ->
        "Calm down, I know what I'm doing!"

      Regex.match?(~r/^[A-Z][^a-z]*$/u, input) ->
        "Whoa, chill out!"

      Regex.match?(~r/^.*[[:print:]]+.*\?$/u, input) ->
        "Sure."

      Regex.match?(~r/^[^[:graph:]]*$/, input) ->
        "Fine. Be that way!"

      true ->
        "Whatever."
    end
  end
end
