defmodule RestApi.MixProject do
  use Mix.Project

  def project do
    [
      app: :rest_api,
      version: "0.1.0",
      elixir: "~> 1.16",
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger],
      mod: {RestApi.Application, []}
    ]
  end

  defp deps do
    [
      {:bandit, "~> 1.0"},
      {:plug, "~> 1.16"},
      {:jason, "~> 1.4"}
    ]
  end
end
