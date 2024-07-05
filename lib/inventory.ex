defmodule Inventory do
  alias Inventory.Native

  @moduledoc """
  Documentation for `Inventory`.
  """
  defdelegate new(), to: Native

  defdelegate add(ref, token, type_id, cfg_id, amount), to: Native

  defdelegate get(ref, token), to: Native

  defdelegate get_by_type(ref, type), to: Native

  defdelegate get_by_cfg_id(ref, type), to: Native

  defdelegate to_list(ref), to: Native

  defdelegate cost(ref, token, amount), to: Native

  def enough(ref, cfg_id, amount) do
    case Native.enough(ref, cfg_id, amount) do
      {:ok, true} -> true
      _ -> false
    end
  end

  defdelegate test_ops(ref, ops), to: Native
end
