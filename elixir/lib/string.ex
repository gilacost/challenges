defmodule Challenges.String do
  @moduledoc """
  This module contains all challenges that have to do with strings manipulation.
  """

  @doc """

  Reverse takes the str parameter being passed and return the string in reversed
  order.

  ## examples:

      iex> Challenges.String.reverse("Hello World")
      "dlroW olleH"

      iex> Challenges.String.reverse("Te quiero Ellie")
      "eillE oreiuq eT"
  """
  def reverse(string), do: String.reverse(string)
end
