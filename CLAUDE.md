@README.md

## Rules

- All benchmark code must be idiomatic, no comments, no unnecessary code
- Python >=3.12, uv, ruff
- Elixir >=1.16, mix, idiomatic OTP patterns
- measure.py asserts every language has non-zero tokens per benchmark, exits non-zero if any implementation is missing
- All implementations must have the same features, same API surface, same module structure
