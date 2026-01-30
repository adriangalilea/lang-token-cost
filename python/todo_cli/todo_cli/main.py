import argparse
import sys

from .commands import add, done, edit, list_todos, remove, stats
from .models import Priority


def parse_tags(value: str) -> list[str]:
    return [t.strip() for t in value.split(",") if t.strip()]


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="todo", description="A CLI todo manager")
    sub = parser.add_subparsers(dest="command", required=True)

    p_add = sub.add_parser("add", help="Add a new todo")
    p_add.add_argument("text")
    p_add.add_argument("-p", "--priority", type=Priority, default=Priority.MEDIUM, choices=list(Priority))
    p_add.add_argument("-t", "--tags", type=parse_tags, default=[])

    p_list = sub.add_parser("list", help="List todos")
    p_list.add_argument("-a", "--all", action="store_true", dest="show_done", help="Show completed")
    p_list.add_argument("-t", "--tag", help="Filter by tag")
    p_list.add_argument("-p", "--priority", type=Priority, choices=list(Priority))

    p_done = sub.add_parser("done", help="Mark a todo as done")
    p_done.add_argument("id", type=int)

    p_rm = sub.add_parser("remove", help="Remove a todo")
    p_rm.add_argument("id", type=int)

    p_edit = sub.add_parser("edit", help="Edit a todo")
    p_edit.add_argument("id", type=int)
    p_edit.add_argument("--text")
    p_edit.add_argument("-p", "--priority", type=Priority, choices=list(Priority))
    p_edit.add_argument("-t", "--tags", type=parse_tags)

    sub.add_parser("stats", help="Show statistics")

    return parser


def main() -> None:
    args = build_parser().parse_args()

    match args.command:
        case "add":
            add(args.text, args.priority, args.tags)
        case "list":
            list_todos(args.show_done, args.tag, args.priority)
        case "done":
            done(args.id)
        case "remove":
            remove(args.id)
        case "edit":
            edit(args.id, args.text, args.priority, args.tags)
        case "stats":
            stats()


if __name__ == "__main__":
    main()
