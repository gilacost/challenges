defmodule Challenges.Path do
  @moduledoc """
  This module defines the functions to find a path in a 5x5 matrix.
  Expects an string that might have the next characters: [u,d,r,l,?].
  The letters stand for up, down, right and left.

  The question marks are directions in the coordinates that need to
  be found taking into account that the path needs to be unique and that
  should never go outside the 5X5 matrix.
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
  def correct(inputString) do
    inputString
    |> String.graphemes()
    |> Enum.map(&randomDirection(&1))
    |> Enum.join()
    |> checkPath(inputString)
  end

  def checkPath(testString, inputString) do
    {history, last_position} =
      testString
      |> String.graphemes()
      |> Enum.reduce_while({{}, [0, 0]}, fn d, {history, acc} ->
        [x, y] = forward(d, acc)

        acc = {
          Tuple.append(history, [x, y]),
          [x, y]
        }

        if x < 0 or x > 4 or y > 0 or y < -4 do
          {:halt, acc}
        else
          {:cont, acc}
        end
      end)

    history_list = Tuple.to_list(history)
    history_unique = Enum.uniq(history_list)

    if length(history_list) == length(history_unique) and last_position == [4, -4] do
      testString
    else
      correct(inputString)
    end
  end

  def randomDirection("?"),
    do:
      Enum.random([
        "d",
        "r",
        "u",
        "l"
      ])

  def randomDirection(d), do: d

  def forward(direction, [x, y]) do
    case direction do
      "d" -> [x, y - 1]
      "r" -> [x + 1, y]
      "u" -> [x, y + 1]
      "l" -> [x - 1, y]
      _ -> 0
    end
  end
end
