from datetime import datetime

from rich.console import Console
from rich.table import Table

from .models import Priority, Todo, TodoStore
from .storage import load, save

console = Console()


def add(text: str, priority: Priority, tags: list[str]) -> None:
    store = load()
    todo = Todo(id=store.next_id, text=text, priority=priority, tags=tags)
    store.todos.append(todo)
    store.next_id += 1
    save(store)
    console.print(f"[green]Added #{todo.id}:[/green] {text}")


def list_todos(show_done: bool, tag: str | None, priority: Priority | None) -> None:
    store = load()
    todos = store.todos

    if not show_done:
        todos = [t for t in todos if not t.done]
    if tag:
        todos = [t for t in todos if tag in t.tags]
    if priority:
        todos = [t for t in todos if t.priority == priority]

    if not todos:
        console.print("[dim]No todos found.[/dim]")
        return

    table = Table(title="Todos")
    table.add_column("ID", style="cyan", width=4)
    table.add_column("Status", width=6)
    table.add_column("Pri", width=6)
    table.add_column("Text")
    table.add_column("Tags", style="dim")
    table.add_column("Created", style="dim")

    priority_colors = {Priority.HIGH: "red", Priority.MEDIUM: "yellow", Priority.LOW: "green"}

    for t in todos:
        status = "[green]done[/green]" if t.done else "[dim]open[/dim]"
        color = priority_colors[t.priority]
        pri = f"[{color}]{t.priority.value}[/{color}]"
        tags = ", ".join(t.tags) if t.tags else ""
        created = t.created_at[:10]
        table.add_row(str(t.id), status, pri, t.text, tags, created)

    console.print(table)


def done(todo_id: int) -> None:
    store = load()
    for t in store.todos:
        if t.id == todo_id:
            assert not t.done, f"Todo #{todo_id} is already done"
            t.done = True
            t.completed_at = datetime.now().isoformat()
            save(store)
            console.print(f"[green]Completed #{todo_id}:[/green] {t.text}")
            return
    console.print(f"[red]Not found: #{todo_id}[/red]")


def remove(todo_id: int) -> None:
    store = load()
    before = len(store.todos)
    store.todos = [t for t in store.todos if t.id != todo_id]
    assert len(store.todos) < before, f"Not found: #{todo_id}"
    save(store)
    console.print(f"[red]Removed #{todo_id}[/red]")


def edit(todo_id: int, text: str | None, priority: Priority | None, tags: list[str] | None) -> None:
    store = load()
    for t in store.todos:
        if t.id == todo_id:
            if text is not None:
                t.text = text
            if priority is not None:
                t.priority = priority
            if tags is not None:
                t.tags = tags
            save(store)
            console.print(f"[yellow]Updated #{todo_id}[/yellow]")
            return
    console.print(f"[red]Not found: #{todo_id}[/red]")


def stats() -> None:
    store = load()
    total = len(store.todos)
    completed = sum(1 for t in store.todos if t.done)
    pending = total - completed

    by_priority = {}
    for t in store.todos:
        if not t.done:
            by_priority[t.priority] = by_priority.get(t.priority, 0) + 1

    console.print(f"\n[bold]Stats[/bold]")
    console.print(f"  Total: {total}  |  Pending: {pending}  |  Done: {completed}")
    if by_priority:
        parts = [f"{p.value}: {c}" for p, c in sorted(by_priority.items())]
        console.print(f"  By priority: {', '.join(parts)}")
    console.print()
