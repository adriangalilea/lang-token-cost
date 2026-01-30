import sys
from pathlib import Path

import tiktoken
from tabulate import tabulate

ENCODING = tiktoken.get_encoding("cl100k_base")

BENCHMARKS = ["todo_cli", "rest_api"]

RUST_EXTS = {".rs", ".toml"}
PYTHON_EXTS = {".py", ".toml"}


def count_tokens(directory: Path, extensions: set[str]) -> dict[str, int]:
    per_file: dict[str, int] = {}
    for f in sorted(directory.rglob("*")):
        if f.is_file() and f.suffix in extensions:
            text = f.read_text()
            tokens = len(ENCODING.encode(text))
            per_file[str(f.relative_to(directory))] = tokens
    return per_file


def main() -> None:
    root = Path(__file__).parent
    rust_dir = root / "rust"
    python_dir = root / "python"

    rows = []
    all_pass = True

    for bench in BENCHMARKS:
        py_tokens = count_tokens(python_dir / bench, PYTHON_EXTS)
        rs_tokens = count_tokens(rust_dir / bench, RUST_EXTS)

        py_total = sum(py_tokens.values())
        rs_total = sum(rs_tokens.values())
        ratio = rs_total / py_total if py_total > 0 else float("inf")

        rows.append([bench, py_total, rs_total, f"{ratio:.1f}x", rs_total - py_total])

        if rs_total <= py_total:
            all_pass = False

    print()
    print(tabulate(
        rows,
        headers=["Benchmark", "Python tokens", "Rust tokens", "Ratio", "Delta"],
        tablefmt="github",
    ))
    print()

    py_grand = sum(r[1] for r in rows)
    rs_grand = sum(r[2] for r in rows)
    grand_ratio = rs_grand / py_grand if py_grand > 0 else float("inf")
    print(f"Total: Python {py_grand} | Rust {rs_grand} | {grand_ratio:.1f}x overhead")
    print()

    for bench in BENCHMARKS:
        print(f"--- {bench} (python) ---")
        for f, t in count_tokens(python_dir / bench, PYTHON_EXTS).items():
            print(f"  {f}: {t} tokens")
        print(f"--- {bench} (rust) ---")
        for f, t in count_tokens(rust_dir / bench, RUST_EXTS).items():
            print(f"  {f}: {t} tokens")
        print()

    assert all_pass, "THESIS FAILED: Rust did not exceed Python token count in all benchmarks"
    print("THESIS HOLDS: Rust requires more tokens than Python in every benchmark.")


if __name__ == "__main__":
    main()
