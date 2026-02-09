defmodule TodoCli.MixProject do
  use Mix.Project

  def project do
    [
      app: :todo_cli,
      version: "0.1.0",
      elixir: "~> 1.16",
      deps: deps(),
      escript: [main_module: TodoCli.CLI]
    ]
  end

  def application do
    [extra_applications: [:logger]]
  end

  defp deps do
    [{:jason, "~> 1.4"}]
  end
end
