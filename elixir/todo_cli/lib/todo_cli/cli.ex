defmodule TodoCli.CLI do
  alias TodoCli.{Commands, Models}

  def main(args) do
    case args do
      ["add" | rest] -> parse_add(rest)
      ["list" | rest] -> parse_list(rest)
      ["done", id] -> Commands.done(String.to_integer(id))
      ["remove", id] -> Commands.remove(String.to_integer(id))
      ["edit" | rest] -> parse_edit(rest)
      ["stats"] -> Commands.stats()
      _ -> IO.puts("Usage: todo <add|list|done|remove|edit|stats>")
    end
  end

  defp parse_add(args) do
    {opts, [text], _} =
      OptionParser.parse(args,
        aliases: [p: :priority, t: :tags],
        strict: [priority: :string, tags: :string]
      )

    priority = opts[:priority] || "medium"
    true = Models.valid_priority?(priority)
    tags = parse_tags(opts[:tags])
    Commands.add(text, priority, tags)
  end

  defp parse_list(args) do
    {opts, _, _} =
      OptionParser.parse(args,
        aliases: [a: :all, t: :tag, p: :priority],
        strict: [all: :boolean, tag: :string, priority: :string]
      )

    Commands.list_todos(opts[:all] || false, opts[:tag], opts[:priority])
  end

  defp parse_edit(args) do
    {opts, [id], _} =
      OptionParser.parse(args,
        aliases: [p: :priority, t: :tags],
        strict: [text: :string, priority: :string, tags: :string]
      )

    Commands.edit(
      String.to_integer(id),
      opts[:text],
      opts[:priority],
      if(opts[:tags], do: parse_tags(opts[:tags]))
    )
  end

  defp parse_tags(nil), do: []
  defp parse_tags(val), do: val |> String.split(",") |> Enum.map(&String.trim/1) |> Enum.filter(&(&1 != ""))
end
