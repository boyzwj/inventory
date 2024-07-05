defmodule InventoryTest do
  use ExUnit.Case
  doctest Inventory

  test "new" do
    {:ok, ref} = Inventory.new()
    assert ref != nil
  end

  test "add" do
    {:ok, ref} = Inventory.new()
    assert Inventory.add(ref, "a", 1, 1, 1) == :ok
  end

  test "multi add and get" do
    {:ok, ref} = Inventory.new()
    ref |> Inventory.add("a", 1, 1, 1)
    ref |> Inventory.add("b", 1, 1, 2)
    ref |> Inventory.add("a", 1, 1, 3)
    assert Inventory.get(ref, "a") == {:ok, {"a", 1, 1, 4}}
  end

  test "multi add and get_by_type" do
    {:ok, ref} = Inventory.new()
    ref |> Inventory.add("a", 1, 1, 1)
    ref |> Inventory.add("b", 1, 1, 2)
    ref |> Inventory.add("a", 1, 1, 3)
    assert Inventory.get_by_type(ref, 1) == {:ok, [{"a", 1, 1, 4}, {"b", 1, 1, 2}]}
  end

  test "to_list" do
    {:ok, ref} = Inventory.new()
    ref |> Inventory.add("a", 1, 1, 1)
    ref |> Inventory.add("b", 1, 2, 2)
    ref |> Inventory.add("c", 1, 1, 3)
    ref |> Inventory.add("d", 1, 3, 3)
    assert Inventory.to_list(ref) |> length == 4
  end

  test "get_by_cfg_id" do
    {:ok, ref} = Inventory.new()
    ref |> Inventory.add("a", 1, 1, 1)
    ref |> Inventory.add("b", 1, 2, 2)
    ref |> Inventory.add("a", 1, 1, 3)
    ref |> Inventory.add("c", 1, 2, 3)
    assert Inventory.get_by_cfg_id(ref, 2) == {:ok, [{"b", 1, 2, 2}, {"c", 1, 2, 3}]}
  end

  test "enough?" do
    {:ok, ref} = Inventory.new()
    ref |> Inventory.add("a", 1, 1, 5)
    assert Inventory.enough(ref, 1, 5) == true
    assert Inventory.enough(ref, 1, 6) == false
  end
end
