defmodule Proxy.Application do
  use Application

  def start(_type, _args) do
    port = String.to_integer(System.get_env("PORT") || "25565")

    children = [
      {Task.Supervisor, name: :listener},
      {Task, fn -> Proxy.Client.accept(port) end},
      {Proxy.Connection, []}
    ]

    opts = [strategy: :one_for_one, name: Proxy.Supervisor]
    Supervisor.start_link(children, opts)
  end
end

