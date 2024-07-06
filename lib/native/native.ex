defmodule Inventory.Native do
  mix_config = Mix.Project.config()
  version = mix_config[:version]
  github_url = mix_config[:package][:links]["GitHub"]

  use RustlerPrecompiled,
    otp_app: :inventory,
    crate: "inventory",
    base_url: "#{github_url}/releases/download/v#{version}",
    force_build: System.get_env("FORCE_INVENTORY_BUILD") in ["1", "true"],
    targets: Enum.uniq(["aarch64-unknown-linux-musl" | RustlerPrecompiled.Config.default_targets()]),
    version: version

  def new(), do: :erlang.nif_error(:nif_not_loaded)

  def add(_ref, _token, _type_id, _cfg_id, _amount), do: :erlang.nif_error(:nif_not_loaded)

  def get(_ref, _token), do: :erlang.nif_error(:nif_not_loaded)

  def get_by_type(_ref, _type), do: :erlang.nif_error(:nif_not_loaded)

  def get_by_cfg_id(_ref, _cfg_id), do: :erlang.nif_error(:nif_not_loaded)

  def amount(_ref, _token), do: :erlang.nif_error(:nif_not_loaded)

  def amount_by_type(_ref, _type), do: :erlang.nif_error(:nif_not_loaded)

  def amount_by_cfg_id(_ref, _cfg_id), do: :erlang.nif_error(:nif_not_loaded)

  def to_list(_ref), do: :erlang.nif_error(:nif_not_loaded)

  def cost(_ref, _token, _amount), do: :erlang.nif_error(:nif_not_loaded)

  def verify_ops(_ref, _ops), do: :erlang.nif_error(:nif_not_loaded)

  def do_ops(_ref, _ops), do: :erlang.nif_error(:nif_not_loaded)
end
