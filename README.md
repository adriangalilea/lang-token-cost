# Token Cost of Programming Languages for LLM-Assisted Development

Higher-level languages consume fewer tokens to express equivalent functionality. This makes them strictly superior for LLM-assisted ("vibe") coding.

## Why it matters

1. **Context windows are finite** — tokens on boilerplate are tokens not spent reasoning
2. **Inference speed is token-bound** — more tokens = slower generation
3. **API cost scales with tokens** — even subscriptions have token budgets
4. **The gap compounds** — as projects grow, overhead multiplies across every file the LLM must read and write

## Results

| Benchmark   |   Python tokens |   Rust tokens |   Elixir tokens | Rust/Py   | Elixir/Py   |
|-------------|-----------------|---------------|-----------------|-----------|-------------|
| todo_cli    |            1835 |          2395 |            2229 | 1.3x      | 1.2x        |
| rest_api    |            2333 |          3450 |            2725 | 1.5x      | 1.2x        |

**Total: Python 4168 | Rust 5845 | Elixir 4954 | Rust 1.4x | Elixir 1.2x**

Python is the most token-efficient. Elixir sits between Python and Rust — closer to Python thanks to lightweight syntax and pattern matching, but its module boilerplate and explicit piping add overhead. Rust's type system, lifetime annotations, and trait implementations make it the most expensive.

## Why Elixir?

Dashbit's [Why Elixir Is the Best Language for AI](https://dashbit.co/blog/why-elixir-best-language-for-ai) argues Elixir's concurrency model and fault tolerance make it ideal for AI workloads. This project tests the token dimension of that argument: how much context does Elixir consume compared to Python and Rust?

## Benchmarks

### `todo_cli` — Interview-level
CLI todo manager with priorities, tags, persistent JSON storage, colored output, statistics.

- **Python**: `argparse` + `dataclass` + `json` + `rich` — 5 files, 1835 tokens
- **Rust**: `clap` + `serde` + `chrono` + `colored` + `thiserror` + `directories` — 6 files, 2395 tokens
- **Elixir**: `OptionParser` + `Jason` + `IO.ANSI` + `File` — 5 files, 2229 tokens

### `rest_api` — Production-level
CRUD REST API with two resources (users, posts), pagination, search, CORS, request logging, error handling, validation.

- **Python**: `FastAPI` + `pydantic` — 8 files, 2333 tokens
- **Rust**: `axum` + `tokio` + `serde` + `tower-http` + `tracing` + `thiserror` + `chrono` — 7 files, 3450 tokens
- **Elixir**: `Plug` + `Bandit` + `Jason` + `Agent` — 8 files, 2725 tokens

## Methodology

- All implementations are idiomatic, no comments, no unnecessary code
- Token counts use `tiktoken` with `cl100k_base` encoding (GPT-4 / Claude-class tokenizer)
- All project files are counted: source code + config (`Cargo.toml`, `pyproject.toml`, `mix.exs`)
- Each implementation has the same features, same module structure, same API surface

## Run it yourself

```bash
uv run python measure.py
```

The script asserts every language has non-zero tokens for every benchmark. If an implementation is missing, it crashes.

## What this doesn't capture (yet)

- **Compiler error token cost**: Rust borrow checker errors are multi-line novels. Python tracebacks are terse. Elixir sits in between.
- **Iteration cost**: How many LLM round-trips to get working code. Rust's type system means more failed attempts.
- **Dependency token cost**: `Cargo.lock` vs `uv.lock` vs `mix.lock` — the transitive dependency graph each language pulls in.
- **Build system complexity**: Cargo workspaces, feature flags, build scripts — none of which Python needs. Elixir's Mix is lightweight but still more than Python's.
