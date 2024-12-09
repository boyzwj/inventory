defmodule InventoryTest do
  use ExUnit.Case
  doctest Inventory

  setup do
    ref = Inventory.new()
    %{ref: ref}
  end

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
    {:ok, l} = Inventory.get_by_type(ref, 1)
    assert Enum.sort(l) == [{"a", 1, 1, 4}, {"b", 1, 1, 2}]
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
    {:ok, l} = Inventory.get_by_cfg_id(ref, 2)
    assert l |> Enum.sort() == [{"b", 1, 2, 2}, {"c", 1, 2, 3}]
  end

  test "verify_ops" do
    {:ok, ref} = Inventory.new()
    ref |> Inventory.add("a", 1, 1, 5)
    ref |> Inventory.add("b", 1, 1, 5)
    ref |> Inventory.add("c", 1, 1, 5)
    ref |> Inventory.add("d", 1, 1, 5)

    ops1 = [{2, "a", 1, 1, 5}, {2, "b", 1, 1, 5}, {2, "d", 1, 1, 5}]
    ops2 = [{2, "a", 1, 1, 7}]
    ops3 = [{2, "a", 1, 1, 5}, {2, "a", 1, 1, 5}]
    assert Inventory.verify_ops(ref, ops1) == true
    assert Inventory.verify_ops(ref, ops2) == false
    assert Inventory.verify_ops(ref, ops3) == false
  end

  test "do_ops" do
    {:ok, ref} = Inventory.new()
    ref |> Inventory.add("a", 1, 1, 5)
    ref |> Inventory.do_ops([{2, "a", 1, 1, 5}])
    assert Inventory.get_by_cfg_id(ref, 1) == {:ok, []}
    ref |> Inventory.do_ops([{1, "a", 1, 1, 5}])
    assert Inventory.get_by_cfg_id(ref, 1) == {:ok, [{"a", 1, 1, 5}]}
  end
end
