defmodule Bob do
  def hey(input) do
    cond do
      Regex.match?(~r/^[^\p{Ll}]*[\p{Lu}][^\p{Ll}]*\?\s*$/u, input) ->
        "Calm down, I know what I'm doing!"

      Regex.match?(~r/^[^\p{Ll}]*[\p{Lu}][^\p{Ll}]*$/u, input) ->
        "Whoa, chill out!"

      Regex.match?(~r/^.*[[:print:]].*\?\s*$/u, input) ->
        "Sure."

      Regex.match?(~r/^[^[:graph:]]*$/u, input) ->
        "Fine. Be that way!"

      true ->
        "Whatever."
    end
  end
end
