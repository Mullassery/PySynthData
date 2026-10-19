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


def _cmd_generate(args: argparse.Namespace) -> int:
    try:
        from pathlib import Path

        from pysynthdata.api import PrivacyBudget, WorldGenerator, load_schema
    except ImportError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1

    try:
        schema = load_schema(args.schema)
        generator = WorldGenerator(schema)
        if args.dp_epsilon is not None:
            budget = PrivacyBudget(epsilon=args.dp_epsilon, delta=args.dp_delta)
            world = generator.generate_private(
                args.rows, args.seed, budget, args.dp_epsilon
            )
        else:
            world = generator.generate(args.rows, args.seed)
    except Exception as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1

    if args.faker:
        try:
            from pysynthdata.faker_fields import apply_faker_fields, parse_spec

            apply_faker_fields(
                world,
                [parse_spec(value) for value in args.faker],
                seed=args.seed,
            )
        except Exception as exc:
            print(f"error: {exc}", file=sys.stderr)
            return 1

    out_dir = Path(args.out)
    out_dir.mkdir(parents=True, exist_ok=True)
    try:
        if args.format == "parquet":
            world.to_parquet(str(out_dir))
            written = {name: str(out_dir / f"{name}.parquet") for name in world.data}
        elif args.format == "csv":
            written = {}
            for name in world.data:
                path = out_dir / f"{name}.csv"
                world.to_pandas(name).to_csv(path, index=False)
                written[name] = str(path)
        else:
            written = {}
            for name in world.data:
                path = out_dir / f"{name}.json"
                path.write_text(
                    __import__("json").dumps(world.data[name], default=str, indent=2),
                    encoding="utf-8",
                )
                written[name] = str(path)
    except Exception as exc:
        print(f"error: writing output failed: {exc}", file=sys.stderr)
        return 1

    print(f"Generated {args.rows} rows/entity (seed={args.seed}) -> {out_dir}")
    for name, path in written.items():
        print(f"  {name}: {path}")
    print(f"fidelity_score={world.fidelity_score:.4f} violations={world.constraint_violations}")
    if world.privacy_report:
        print(f"privacy={world.privacy_report}")
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

    generate_parser = subparsers.add_parser(
        "generate",
        help="Generate synthetic rows from a schema and export them",
        description=(
            "Generate relational synthetic data from a pysynthdata YAML/JSON "
            "schema and write CSV, JSON or Parquet files per entity. Optional "
            "differential privacy uses the Rust Laplace mechanism."
        ),
    )
    generate_parser.add_argument("schema", help="Path to the YAML/JSON schema")
    generate_parser.add_argument("--out", required=True, help="Output directory")
    generate_parser.add_argument("--rows", type=int, default=1000, help="Rows per entity")
    generate_parser.add_argument("--seed", type=int, default=42, help="Deterministic seed")
    generate_parser.add_argument(
        "--format", choices=["csv", "json", "parquet"], default="parquet"
    )
    generate_parser.add_argument(
        "--dp-epsilon",
        type=float,
        default=None,
        help="Enable differential privacy with this epsilon (numeric fields only)",
    )
    generate_parser.add_argument(
        "--dp-delta", type=float, default=0.0, help="Delta for the privacy budget"
    )
    generate_parser.add_argument(
        "--faker",
        action="append",
        default=[],
        metavar="Entity.field=provider",
        help="Fill a field with a Faker provider, e.g. Customer.email=email",
    )
    generate_parser.set_defaults(func=_cmd_generate)

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
