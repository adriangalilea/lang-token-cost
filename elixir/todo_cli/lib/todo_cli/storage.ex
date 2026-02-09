defmodule TodoCli.Storage do
  alias TodoCli.Models

  @path Path.expand("~/.todo.json")

  def load do
    case File.read(@path) do
      {:ok, content} ->
        raw = Jason.decode!(content)
        todos = Enum.map(raw["todos"], &Models.from_map/1)
        {todos, raw["next_id"]}

      {:error, :enoent} ->
        {[], 1}
    end
  end

  def save(todos, next_id) do
    data = %{"todos" => Enum.map(todos, &Models.to_map/1), "next_id" => next_id}
    File.write!(@path, Jason.encode!(data, pretty: true))
  end
end
