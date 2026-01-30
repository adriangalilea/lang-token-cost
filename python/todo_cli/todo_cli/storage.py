import json
from dataclasses import asdict
from pathlib import Path

from .models import Todo, TodoStore


DEFAULT_PATH = Path.home() / ".todo.json"


def load(path: Path = DEFAULT_PATH) -> TodoStore:
    if not path.exists():
        return TodoStore()
    raw = json.loads(path.read_text())
    todos = [Todo(**t) for t in raw["todos"]]
    return TodoStore(todos=todos, next_id=raw["next_id"])


def save(store: TodoStore, path: Path = DEFAULT_PATH) -> None:
    data = {"todos": [asdict(t) for t in store.todos], "next_id": store.next_id}
    path.write_text(json.dumps(data, indent=2))
