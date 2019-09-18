defmodule Challenges.Path do
  @moduledoc """
  This module defines the functions to find a path in a 5x5 matrix.
  Expects an string that might have the next symbols: [u,d,r,l,?].
  The letters stand for up, down, right and left.

  The question marks are directions in the coordinates that need to
  be found taking into account that no the path needs to be unique.
  and that possitions veisted previously might not be visited again,
  """

  @doc """
  ## Examples:

      iex> Challenges.Path.correct("???rrurdr?")
      "dddrrurdrd"

      iex> Challenges.Path.correct("drdr??rrddd?")
      "drdruurrdddd"

  ## First example matrix:

      d 0 0 0 0
      d 0 0 0 0
      d 0 r d 0
      r r u r d
      0 0 0 0 0

  ## Second example matrix:

      d 0 r r d
      r d u 0 d
      0 r u 0 d
      0 0 0 0 d
      0 0 0 0 0
  """
  def correct(string) do
    string
    |> String.graphemes()
    |> directionsCount()
    |> pendingDriections()
  end

  def directionsCount(pathList) do
    directions = %{
      "d" => 0,
      "u" => 0,
      "l" => 0,
      "r" => 0,
      "?" => 0
    }

    pathList
    |> Enum.map_reduce(directions, fn d, acc ->
      acc = Map.update!(acc, "#{d}", &(&1 + 1))
      {pathList, acc}
    end)
  end

  def pendingDriections({pathList, %{"d" => d, "u" => u, "l" => l, "r" => r, "?" => unk}}) do
    IO.inspect(pathList)
    _currentPosition = {u - d, r - l} |> IO.inspect()
    IO.inspect(unk)
  end
end
