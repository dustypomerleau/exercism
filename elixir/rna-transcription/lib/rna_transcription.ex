defmodule RnaTranscription do
  @dna_bases [?A, ?C, ?G, ?T]
  @doc """
  Transcribes a character list representing DNA nucleotides to RNA

  ## Examples

  iex> RnaTranscription.to_rna('ACTG')
  'UGAC'
  """
  @spec to_rna([char]) :: [char]
  def to_rna(dna) when is_list(dna) do
    Enum.map(dna, &transcribe/1)
  end

  @spec transcribe(char) :: char
  defp transcribe(base) when base in @dna_bases do
    case base do
      ?A -> ?U
      ?C -> ?G
      ?G -> ?C
      ?T -> ?A
    end
  end
end
