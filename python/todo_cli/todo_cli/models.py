from dataclasses import dataclass, field
from datetime import datetime
from enum import StrEnum


class Priority(StrEnum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"


@dataclass
class Todo:
    id: int
    text: str
    done: bool = False
    priority: Priority = Priority.MEDIUM
    tags: list[str] = field(default_factory=list)
    created_at: str = field(default_factory=lambda: datetime.now().isoformat())
    completed_at: str | None = None


@dataclass
class TodoStore:
    todos: list[Todo] = field(default_factory=list)
    next_id: int = 1
