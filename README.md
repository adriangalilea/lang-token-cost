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
| rest_api    |            2487 |          3669 |            2918 | 1.5x      | 1.2x        |

**Total: Python 4322 | Rust 6064 | Elixir 5147 | Rust 1.4x | Elixir 1.2x**

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
CRUD REST API with two resources (users, posts), pagination, search, CORS, request logging, error handling, validation, cascade delete, computed post counts.

- **Python**: `FastAPI` + `pydantic` — 8 files, 2487 tokens
- **Rust**: `axum` + `tokio` + `serde` + `tower-http` + `tracing` + `thiserror` + `chrono` — 7 files, 3669 tokens
- **Elixir**: `Plug` + `Bandit` + `Jason` + `Agent` — 8 files, 2918 tokens

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

## Beyond token counts: an iteration test

Dashbit's article motivated a deeper question: what does it actually feel like to iterate on each language? Static token counts measure the final artifact, but LLM-assisted development is a loop — read, modify, verify, repeat. This experiment tests iteration cost directly.

This experiment was designed and executed by Claude Code (Opus 4.6).

### The task

> "When you delete a user, also delete all their posts. When listing/getting users, include how many posts each one has."

This is a good iteration test because it crosses resource boundaries (users ↔ posts), modifies response shapes, and touches store, routes, and models. The `post_count` must be computed at read time, not stored — a subtlety that forces each language to handle it differently.

### Token cost of the change

| Language | Before | After | Delta | Files changed |
|----------|--------|-------|-------|---------------|
| Python   |   2333 |  2487 |  +154 | 2 (models, store) |
| Rust     |   3450 |  3669 |  +219 | 3 (models, store, routes) |
| Elixir   |   2725 |  2918 |  +193 | 2 (store, routes) |

Rust's delta is 42% larger than Python's. Elixir is 25% larger.

### What happened in each language

**Python** — 2 files, no friction. Added `post_count: int = 0` to the existing `User` model. Pydantic's `model_copy(update=...)` made injecting the computed field trivial. Cascade delete was a one-liner dict comprehension. The routes didn't change at all — the store returns the enriched model transparently.

**Elixir** — 2 files, minor friction. Since `User.to_json` returns a plain map, adding `post_count` was just `Map.put`. Cascade delete required filtering inside the Agent callback. The routes needed updating because the `post_count` injection happens at the boundary (routes call `Store.count_user_posts`), not inside the store. Straightforward but more surface area than Python.

**Rust** — 3 files, real friction. The type system forced a new `UserResponse` struct because `User` (the storage type) can't carry a computed field. Every handler return type changed from `User` to `UserResponse`. The store needed a `user_to_response` builder that clones every field. `update_user` required reborrowing gymnastics — mutating the user, then re-reading it immutably to build the response — because you can't hold a mutable and immutable borrow simultaneously. The cascade delete itself was simple (`retain`), but the type plumbing dominated the effort.

### What's real and what's noise

**Real:** Rust's type system forces structural changes for cross-cutting features. `UserResponse` had to exist because the storage type can't carry a computed field. Every handler signature changed. This isn't an artifact of the experiment — it happens in every real Rust project. Python absorbed the change without touching routes. Elixir's `Map.put` made schema evolution free. These are genuine language properties.

**Artificial:** Claude Code (the author) had perfect context — read every file before editing, held the entire codebase in memory, didn't hit a single compilation error in any language. The experiment was supposed to test iteration friction, but there was almost none. The interesting costs (borrow checker fights, re-reading verbose error output, multiple failed attempts) never materialized. A human or a fresh LLM without prior context would have a very different experience, especially with Rust.

**Noisy:** The token deltas are confounded by formatting and lint fixes applied during the same pass. `mix format` expanded compact Elixir into multi-line code. An unused Python import got cleaned up. The pure feature delta is smaller and harder to isolate. And 65 tokens difference between Python and Rust is ~2 lines of code — not enough to draw quantitative conclusions from a single data point.

**What would actually be interesting:** Running the same task with a fresh LLM that doesn't know the codebase. Measure total tokens consumed (file reads + compiler errors + retries), not just diff size. That captures the real iteration cost — how much context you burn getting to a working change. Rust would almost certainly fare worse there because borrow checker errors are verbose and often require re-reading multiple files to resolve.

## What this doesn't capture (yet)

- **True iteration cost**: Total tokens consumed across reads, errors, and retries for a fresh LLM implementing a feature without prior context. The iteration test above used an LLM that already knew the codebase — the hard version of this experiment hasn't been run yet.
- **Compiler error token cost**: Rust borrow checker errors are multi-line novels. Python tracebacks are terse. Elixir sits in between.
- **Dependency token cost**: `Cargo.lock` vs `uv.lock` vs `mix.lock` — the transitive dependency graph each language pulls in.
- **Build system complexity**: Cargo workspaces, feature flags, build scripts — none of which Python needs. Elixir's Mix is lightweight but still more than Python's.
