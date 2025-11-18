#!/usr/bin/env python3
import argparse
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path


PROJECT_ROOT = Path(__file__).parent.resolve()
ROCKET_TOML = PROJECT_ROOT / "Rocket.toml"
MIGRATIONS_DIR = PROJECT_ROOT / "migrations"


def _print(msg: str) -> None:
    print(msg, flush=True)


def read_db_url() -> str:
    # Try Python 3.11+ tomllib first
    if ROCKET_TOML.exists():
        try:
            import tomllib  # type: ignore

            with ROCKET_TOML.open("rb") as f:
                data = tomllib.load(f)
            return (
                data.get("default", {})
                .get("databases", {})
                .get("sqlite_db", {})
                .get("url", "sqlite:./database/todos.db")
            )
        except Exception:
            pass

        # Fallback: simple regex scan for url = "sqlite:..."
        try:
            content = ROCKET_TOML.read_text(encoding="utf-8")
            m = re.search(r"^url\s*=\s*\"([^\"]+)\"", content, re.M)
            if m:
                return m.group(1)
        except Exception:
            pass

    return "sqlite:./database/todos.db"


def get_sqlite_path() -> Path:
    url = read_db_url()
    if not url.startswith("sqlite:"):
        raise SystemExit("Only sqlite URLs are supported (e.g. sqlite:./database/todos.db)")
    path_part = url.split(":", 1)[1]
    # Normalize relative to project root
    return (PROJECT_ROOT / path_part).resolve()


def run_cmd(cmd: list[str], env: dict | None = None, check: bool = True) -> int:
    _print("$ " + " ".join(cmd))
    result = subprocess.run(cmd, cwd=str(PROJECT_ROOT), env=env or os.environ.copy())
    if check and result.returncode != 0:
        raise SystemExit(result.returncode)
    return result.returncode


def cmd_run(_: argparse.Namespace) -> None:
    run_cmd(["cargo", "run"])


def cmd_watch(_: argparse.Namespace) -> None:
    # Requires cargo-watch installed: cargo install cargo-watch
    code = run_cmd(["cargo", "watch", "-x", "run"], check=False)
    if code != 0:
        _print("cargo-watch not found or failed. Install with: cargo install cargo-watch")
        raise SystemExit(code)


def cmd_test(_: argparse.Namespace) -> None:
    run_cmd(["cargo", "test"])


def cmd_fmt(_: argparse.Namespace) -> None:
    run_cmd(["cargo", "fmt"])


def cmd_clippy(_: argparse.Namespace) -> None:
    run_cmd(["cargo", "clippy", "--", "-D", "warnings"])


def cmd_check(_: argparse.Namespace) -> None:
    run_cmd(["cargo", "check"])


def cmd_migrate(_: argparse.Namespace) -> None:
    db_path = get_sqlite_path()
    db_path.parent.mkdir(parents=True, exist_ok=True)
    import sqlite3

    conn = sqlite3.connect(str(db_path))
    try:
        cur = conn.cursor()
        sql_files = sorted(p for p in MIGRATIONS_DIR.glob("*.sql"))
        if not sql_files:
            _print("No .sql files found in migrations/")
        for p in sql_files:
            _print(f"Applying migration: {p.name}")
            script = p.read_text(encoding="utf-8")
            cur.executescript(script)
        conn.commit()
        _print("Migrations applied successfully.")
    finally:
        conn.close()


def cmd_reset_db(_: argparse.Namespace) -> None:
    db_path = get_sqlite_path()
    # Remove DB and sidecar files
    removed = False
    for path in [db_path, db_path.with_suffix(db_path.suffix + "-shm"), db_path.with_suffix(db_path.suffix + "-wal")]:
        if path.exists():
            path.unlink()
            removed = True
            _print(f"Removed {path}")
    if not removed:
        _print("No existing database files to remove.")
    cmd_migrate(_)


def cmd_seed(_: argparse.Namespace) -> None:
    db_path = get_sqlite_path()
    import sqlite3

    conn = sqlite3.connect(str(db_path))
    try:
        cur = conn.cursor()
        rows = [
            ("Write docs", "Draft API docs and README", "in_progress", "high"),
            ("Implement update", "Add PUT /todos/:id", "pending", "medium"),
            ("Polish UI", "Tweak styles for list view", "completed", "low"),
        ]
        cur.executemany(
            """
            INSERT INTO todos (title, description, status, priority)
            VALUES (?, ?, ?, ?)
            """,
            rows,
        )
        conn.commit()
        _print(f"Seeded {len(rows)} todos into {db_path}")
    finally:
        conn.close()


def make_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Dev utilities for the Rocket app")
    sub = parser.add_subparsers(dest="cmd", required=True)

    sub.add_parser("run", help="Run the app (cargo run)").set_defaults(func=cmd_run)
    sub.add_parser("watch", help="Run with autoreload (cargo watch -x run)").set_defaults(func=cmd_watch)
    sub.add_parser("test", help="Run tests").set_defaults(func=cmd_test)
    sub.add_parser("fmt", help="Format code with rustfmt").set_defaults(func=cmd_fmt)
    sub.add_parser("clippy", help="Lint with clippy (deny warnings)").set_defaults(func=cmd_clippy)
    sub.add_parser("check", help="Type-check with cargo check").set_defaults(func=cmd_check)

    sub.add_parser("migrate", help="Apply SQL migrations to SQLite").set_defaults(func=cmd_migrate)
    sub.add_parser("reset-db", help="Delete DB and re-apply migrations").set_defaults(func=cmd_reset_db)
    sub.add_parser("seed", help="Seed sample data").set_defaults(func=cmd_seed)

    return parser


def main() -> None:
    parser = make_parser()
    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()


