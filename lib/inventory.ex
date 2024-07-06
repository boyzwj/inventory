defmodule Inventory do
  alias Inventory.Native

  @moduledoc """
  Documentation for `Inventory`.
  """

  @doc """
  Creates a new inventory.
  """
  @spec new() :: any()
  def new(), do: Native.new()

  @doc """
  Adds an asset to the inventory.

  ## Parameters
  - `ref`: The reference to the inventory.
  - `token`: The asset ID, a string.
  - `type_id`: The type ID of the asset.
  - `cfg_id`: The configuration ID of the asset.
  - `amount`: The amount of the asset to add.
  """
  @spec add(
          ref :: any(),
          token :: String.t(),
          type_id :: any(),
          cfg_id :: any(),
          amount :: integer()
        ) :: any()
  def add(ref, token, type_id, cfg_id, amount) do
    Native.add(ref, token, type_id, cfg_id, amount)
  end

  @doc """
  Retrieves an asset from the inventory by its token.

  ## Parameters
  - `ref`: The reference to the inventory.
  - `token`: The asset ID, a string.
  """
  @spec get(ref :: any(), token :: String.t()) :: any()
  def get(ref, token) do
    Native.get(ref, token)
  end

  @doc """
  Retrieves assets from the inventory by their type.

  ## Parameters
  - `ref`: The reference to the inventory.
  - `type`: The type of the assets.
  """
  @spec get_by_type(ref :: any(), type :: any()) :: any()
  def get_by_type(ref, type) do
    Native.get_by_type(ref, type)
  end

  @doc """
  Retrieves assets from the inventory by their configuration ID.

  ## Parameters
  - `ref`: The reference to the inventory.
  - `type`: The configuration ID of the assets.
  """
  @spec get_by_cfg_id(ref :: any(), type :: any()) :: any()
  def get_by_cfg_id(ref, type) do
    Native.get_by_cfg_id(ref, type)
  end

  @doc """
  Get the amount of a token in the inventory.

  ## Parameters
  - `ref`: The reference to the inventory.
  - `token`: The asset ID, a string.

  ## Example:
        iex> {:ok, ref} = Inventory.new()
        iex> ref |> Inventory.add("a", 1, 1, 5)
        iex> Inventory.amount(ref, "a")
        5
  """
  @spec amount(ref :: any(), token :: String.t()) :: integer()
  def amount(ref, token) do
    Native.amount(ref, token)
  end

  @doc """
  Get the total amount of assets of a specific type in the inventory.

  ## Parameters
  - `ref`: The reference to the inventory.
  - `type`: The type of the assets.

  ## Example:
      iex> {:ok, ref} = Inventory.new()
      iex> ref |> Inventory.add("a", 1, 1, 5)
      iex> ref |> Inventory.add("b", 1, 2, 5)
      iex> ref |> Inventory.add("c", 2, 3, 4)
      iex> Inventory.amount_by_type(ref, 1)
      10
  """
  @spec amount_by_type(ref :: any(), type :: any()) :: integer()
  def amount_by_type(ref, type) do
    Native.amount_by_type(ref, type)
  end

  @doc """
  Get the total amount of assets of a specific configuration ID in the inventory.

  ## Parameters
  - `ref`: The reference to the inventory.
  - `type`: The configuration ID of the assets.

  ## Example:
    iex> {:ok, ref} = Inventory.new()
    iex> ref |> Inventory.add("a", 1, 1, 5)
    iex> ref |> Inventory.add("b", 1, 2, 5)
    iex> ref |> Inventory.add("c", 1, 2, 4)
    iex> Inventory.amount_by_cfg_id(ref, 2)
    9
  """
  @spec amount_by_cfg_id(ref :: any(), type :: any()) :: integer()
  def amount_by_cfg_id(ref, type) do
    Native.amount_by_cfg_id(ref, type)
  end

  @doc """
  Converts the inventory to a list.

  ## Parameters
  - `ref`: The reference to the inventory.
  """
  @spec to_list(ref :: any()) :: list()
  def to_list(ref) do
    Native.to_list(ref)
  end

  @doc """
  Calculates the cost of a token in the inventory.

  ## Parameters
  - `ref`: The reference to the inventory.
  - `token`: The asset ID, a string.
  - `amount`: The amount of the asset.
  """
  @spec cost(ref :: any(), token :: String.t(), amount :: integer()) :: any()
  def cost(ref, token, amount) do
    Native.cost(ref, token, amount)
  end

  @doc """
  Verifies the operations on the inventory.

  ## Parameters
  - `ref`: The reference to the inventory.
  - `ops`: The operations to verify.
  """
  @spec verify_ops(ref :: any(), ops :: any()) :: any()
  def verify_ops(ref, ops) do
    Native.verify_ops(ref, ops)
  end

  @doc """
  Performs the operations on the inventory.

  ## Parameters
  - `ref`: The reference to the inventory.
  - `ops`: The operations to perform.
    ## Example:
    iex> {:ok, ref} = Inventory.new()
    iex> ref |> Inventory.do_ops([{1,"a", 1, 1, 5}])
    {:ok,[{3,{"a", 1, 1, 5}}]}
  """
  @spec do_ops(ref :: any(), ops :: any()) :: any()
  def do_ops(ref, ops) do
    Native.do_ops(ref, ops)
  end
end
