defmodule TodoCli.Commands do
  alias TodoCli.{Models, Storage}

  def add(text, priority, tags) do
    {todos, next_id} = Storage.load()
    now = DateTime.utc_now() |> DateTime.to_iso8601()
    todo = %Models{id: next_id, text: text, priority: priority, tags: tags, created_at: now}
    Storage.save(todos ++ [todo], next_id + 1)
    IO.puts(IO.ANSI.green() <> "Added ##{next_id}: " <> IO.ANSI.reset() <> text)
  end

  def list_todos(show_done, tag, priority) do
    {todos, _} = Storage.load()
    todos = if show_done, do: todos, else: Enum.filter(todos, &(!&1.done))
    todos = if tag, do: Enum.filter(todos, &(tag in &1.tags)), else: todos
    todos = if priority, do: Enum.filter(todos, &(&1.priority == priority)), else: todos

    if todos == [] do
      IO.puts(IO.ANSI.faint() <> "No todos found." <> IO.ANSI.reset())
    else
      header = format_row("ID", "Status", "Pri", "Text", "Tags", "Created")
      IO.puts(IO.ANSI.bright() <> header <> IO.ANSI.reset())
      Enum.each(todos, &print_todo/1)
    end
  end

  defp format_row(id, status, pri, text, tags, created) do
    String.pad_trailing(id, 6) <>
      String.pad_trailing(status, 8) <>
      String.pad_trailing(pri, 8) <>
      String.pad_trailing(text, 30) <>
      String.pad_trailing(tags, 16) <>
      created
  end

  defp priority_color("high"), do: IO.ANSI.red()
  defp priority_color("medium"), do: IO.ANSI.yellow()
  defp priority_color("low"), do: IO.ANSI.green()

  defp print_todo(t) do
    status = if t.done, do: IO.ANSI.green() <> "done" <> IO.ANSI.reset(), else: IO.ANSI.faint() <> "open" <> IO.ANSI.reset()
    pri = priority_color(t.priority) <> t.priority <> IO.ANSI.reset()
    tags = Enum.join(t.tags, ", ")
    created = String.slice(t.created_at || "", 0, 10)
    IO.puts(format_row("#{t.id}", status, pri, t.text, tags, created))
  end

  def done(todo_id) do
    {todos, next_id} = Storage.load()
    {todo, rest} = pop_by_id(todos, todo_id)
    assert todo != nil, "Not found: ##{todo_id}"
    assert !todo.done, "Todo ##{todo_id} is already done"
    now = DateTime.utc_now() |> DateTime.to_iso8601()
    updated = %{todo | done: true, completed_at: now}
    Storage.save(reinsert(rest, updated, todos), next_id)
    IO.puts(IO.ANSI.green() <> "Completed ##{todo_id}: " <> IO.ANSI.reset() <> todo.text)
  end

  def remove(todo_id) do
    {todos, next_id} = Storage.load()
    filtered = Enum.reject(todos, &(&1.id == todo_id))
    assert length(filtered) < length(todos), "Not found: ##{todo_id}"
    Storage.save(filtered, next_id)
    IO.puts(IO.ANSI.red() <> "Removed ##{todo_id}" <> IO.ANSI.reset())
  end

  def edit(todo_id, text, priority, tags) do
    {todos, next_id} = Storage.load()
    {todo, rest} = pop_by_id(todos, todo_id)
    assert todo != nil, "Not found: ##{todo_id}"
    updated = todo
    updated = if text, do: %{updated | text: text}, else: updated
    updated = if priority, do: %{updated | priority: priority}, else: updated
    updated = if tags, do: %{updated | tags: tags}, else: updated
    Storage.save(reinsert(rest, updated, todos), next_id)
    IO.puts(IO.ANSI.yellow() <> "Updated ##{todo_id}" <> IO.ANSI.reset())
  end

  def stats do
    {todos, _} = Storage.load()
    total = length(todos)
    completed = Enum.count(todos, & &1.done)
    pending = total - completed

    by_priority =
      todos
      |> Enum.reject(& &1.done)
      |> Enum.frequencies_by(& &1.priority)
      |> Enum.sort()

    IO.puts("\n#{IO.ANSI.bright()}Stats#{IO.ANSI.reset()}")
    IO.puts("  Total: #{total}  |  Pending: #{pending}  |  Done: #{completed}")

    if by_priority != [] do
      parts = Enum.map_join(by_priority, ", ", fn {p, c} -> "#{p}: #{c}" end)
      IO.puts("  By priority: #{parts}")
    end

    IO.puts("")
  end

  defp pop_by_id(todos, id) do
    case Enum.find(todos, &(&1.id == id)) do
      nil -> {nil, todos}
      todo -> {todo, Enum.reject(todos, &(&1.id == id))}
    end
  end

  defp reinsert(rest, updated, original) do
    idx = Enum.find_index(original, &(&1.id == updated.id))
    List.insert_at(rest, idx, updated)
  end

  defp assert(true, _), do: :ok
  defp assert(false, msg), do: raise(msg)
end
