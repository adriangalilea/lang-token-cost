defmodule TodoCli.Models do
  @priorities ~w(low medium high)

  defstruct [:id, :text, done: false, priority: "medium", tags: [], created_at: nil, completed_at: nil]

  def priorities, do: @priorities

  def valid_priority?(p), do: p in @priorities

  def from_map(map) do
    %__MODULE__{
      id: map["id"],
      text: map["text"],
      done: map["done"] || false,
      priority: map["priority"] || "medium",
      tags: map["tags"] || [],
      created_at: map["created_at"],
      completed_at: map["completed_at"]
    }
  end

  def to_map(%__MODULE__{} = todo) do
    %{
      "id" => todo.id,
      "text" => todo.text,
      "done" => todo.done,
      "priority" => todo.priority,
      "tags" => todo.tags,
      "created_at" => todo.created_at,
      "completed_at" => todo.completed_at
    }
  end
end
