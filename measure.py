from pathlib import Path

import tiktoken
from tabulate import tabulate

ENCODING = tiktoken.get_encoding("cl100k_base")

BENCHMARKS = ["todo_cli", "rest_api"]

LANGUAGES = {
    "Python": {
        "dir": "python",
        "exts": {".py", ".toml"},
    },
    "Rust": {
        "dir": "rust",
        "exts": {".rs", ".toml"},
    },
    "Elixir": {
        "dir": "elixir",
        "exts": {".ex", ".exs"},
    },
}


SKIP_DIRS = {"deps", "target", "_build", "__pycache__", ".elixir_ls"}


def count_tokens(directory: Path, extensions: set[str]) -> dict[str, int]:
    per_file: dict[str, int] = {}
    for f in sorted(directory.rglob("*")):
        if any(part in SKIP_DIRS for part in f.relative_to(directory).parts):
            continue
        if f.is_file() and f.suffix in extensions:
            text = f.read_text()
            tokens = len(ENCODING.encode(text))
            per_file[str(f.relative_to(directory))] = tokens
    return per_file


def main() -> None:
    root = Path(__file__).parent

    rows = []
    totals: dict[str, int] = {lang: 0 for lang in LANGUAGES}

    for bench in BENCHMARKS:
        counts = {}
        for lang, cfg in LANGUAGES.items():
            tokens = count_tokens(root / cfg["dir"] / bench, cfg["exts"])
            total = sum(tokens.values())
            assert total > 0, f"{lang}/{bench} has 0 tokens — missing implementation?"
            counts[lang] = total
            totals[lang] += total

        py = counts["Python"]
        row = [bench]
        for lang in LANGUAGES:
            row.append(counts[lang])
        row.append(f"{counts['Rust'] / py:.1f}x")
        row.append(f"{counts['Elixir'] / py:.1f}x")
        rows.append(row)

    headers = ["Benchmark"] + [f"{lang} tokens" for lang in LANGUAGES] + ["Rust/Py", "Elixir/Py"]

    print()
    print(tabulate(rows, headers=headers, tablefmt="github"))
    print()

    parts = " | ".join(f"{lang} {totals[lang]}" for lang in LANGUAGES)
    grand_ratio_rust = totals["Rust"] / totals["Python"]
    grand_ratio_elixir = totals["Elixir"] / totals["Python"]
    print(f"Total: {parts} | Rust {grand_ratio_rust:.1f}x | Elixir {grand_ratio_elixir:.1f}x")
    print()

    for bench in BENCHMARKS:
        for lang, cfg in LANGUAGES.items():
            print(f"--- {bench} ({lang.lower()}) ---")
            for f, t in count_tokens(root / cfg["dir"] / bench, cfg["exts"]).items():
                print(f"  {f}: {t} tokens")
        print()


if __name__ == "__main__":
    main()
