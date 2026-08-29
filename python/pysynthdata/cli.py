"""PySynthData command-line interface.

Installed as the `pysynthdata` console script (see `[project.scripts]` in
pyproject.toml) once the package is `pip install`ed.
"""

from __future__ import annotations

import argparse
import sys


def _cmd_infer_schema(args: argparse.Namespace) -> int:
    try:
        from pysynthdata.db_schema import infer_schema_to_file
    except ImportError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1

    try:
        yaml_text = infer_schema_to_file(
            args.db_url,
            args.output,
            table=args.table,
            all_tables=args.all_tables,
        )
    except Exception as exc:  # connection failures, missing table, bad URL, ...
        print(f"error: {exc}", file=sys.stderr)
        return 1

    print(f"Wrote schema to {args.output}")
    print(yaml_text)
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="pysynthdata", description="PySynthData command-line tools"
    )
    subparsers = parser.add_subparsers(dest="command")

    infer_parser = subparsers.add_parser(
        "infer-schema",
        help="Infer a pysynthdata YAML schema from a live Postgres/MySQL/SQLite database",
        description=(
            "Connects to a live database, reflects real table structure via "
            "SQLAlchemy's inspect() API, and writes a pysynthdata-compatible "
            "YAML schema (the same shape as examples/banking_schema.yaml). "
            "Requires the 'db' extra: pip install pysynthdata[db]"
        ),
    )
    infer_parser.add_argument(
        "--db-url",
        required=True,
        help=(
            "SQLAlchemy connection URL, e.g. "
            "postgresql+psycopg://user:pass@host/dbname, "
            "mysql+pymysql://user:pass@host/dbname, or "
            "sqlite:///path/to.db"
        ),
    )
    table_group = infer_parser.add_mutually_exclusive_group(required=True)
    table_group.add_argument("--table", help="Name of a single table to infer")
    table_group.add_argument(
        "--all-tables",
        action="store_true",
        help="Infer every table in the database",
    )
    infer_parser.add_argument(
        "--output", required=True, help="Path to write the inferred YAML schema to"
    )
    infer_parser.set_defaults(func=_cmd_infer_schema)

    return parser


def main(argv: "list[str] | None" = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    if not getattr(args, "command", None):
        parser.print_help()
        return 1
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
