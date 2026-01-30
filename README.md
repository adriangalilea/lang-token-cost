# Token Cost of Programming Languages for LLM-Assisted Development

Higher-level languages consume fewer tokens to express equivalent functionality. This makes them strictly superior for LLM-assisted ("vibe") coding.

## Why it matters

1. **Context windows are finite** — tokens on boilerplate are tokens not spent reasoning
2. **Inference speed is token-bound** — more tokens = slower generation
3. **API cost scales with tokens** — even subscriptions have token budgets
4. **The gap compounds** — as projects grow, overhead multiplies across every file the LLM must read and write

## Results

| Benchmark   |   Python tokens |   Rust tokens | Ratio   |   Delta |
|-------------|-----------------|---------------|---------|---------|
| todo_cli    |            1835 |          2395 | 1.3x    |     560 |
| rest_api    |            2333 |          3450 | 1.5x    |    1117 |

**Total: Python 4168 | Rust 5845 | 1.4x overhead**

The gap widens with project complexity: 1.3x for a CLI app, 1.5x for a REST API. Real-world projects with async runtimes, error types, trait implementations, and lifetime annotations push this further.

## Benchmarks

### `todo_cli` — Interview-level
CLI todo manager with priorities, tags, persistent JSON storage, colored output, statistics.

- **Python**: `argparse` + `dataclass` + `json` + `rich` — 5 files, 1835 tokens
- **Rust**: `clap` + `serde` + `chrono` + `colored` + `thiserror` + `directories` — 6 files, 2395 tokens

### `rest_api` — Production-level
CRUD REST API with two resources (users, posts), pagination, search, CORS, request logging, error handling, validation.

- **Python**: `FastAPI` + `pydantic` — 8 files, 2333 tokens
- **Rust**: `axum` + `tokio` + `serde` + `tower-http` + `tracing` + `thiserror` + `chrono` — 7 files, 3450 tokens

## Methodology

- Both implementations are idiomatic, no comments, no unnecessary code
- Token counts use `tiktoken` with `cl100k_base` encoding (GPT-4 / Claude-class tokenizer)
- All project files are counted: source code + config (`Cargo.toml`, `pyproject.toml`)
- Each implementation has the same features, same module structure, same API surface

## Run it yourself

```bash
uv run python measure.py
```

The script asserts that Rust exceeds Python in token count for every benchmark. If the thesis fails, it crashes.

## What this doesn't capture (yet)

- **Compiler error token cost**: Rust borrow checker errors are multi-line novels. Python tracebacks are terse.
- **Iteration cost**: How many LLM round-trips to get working code. Rust's type system means more failed attempts.
- **Dependency token cost**: `Cargo.lock` vs `uv.lock` — the transitive dependency graph Rust pulls in.
- **Build system complexity**: Cargo workspaces, feature flags, build scripts — none of which Python needs.
