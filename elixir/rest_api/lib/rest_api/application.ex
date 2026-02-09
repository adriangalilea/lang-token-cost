defmodule RestApi.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      RestApi.Store,
      {Bandit, plug: RestApi.Router, port: 8000}
    ]

    Supervisor.start_link(children, strategy: :one_for_one, name: RestApi.Supervisor)
  end
end
