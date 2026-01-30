@README.md

## Rules

- All benchmark code must be idiomatic, no comments, no unnecessary code
- Python >=3.12, uv, ruff
- measure.py is a living proof: it asserts rust tokens > python tokens, exits non-zero if thesis fails
- Both implementations must have the same features, same API surface, same module structure
