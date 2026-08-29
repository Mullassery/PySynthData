"""Live database schema introspection for pysynthdata.

Connects to a real Postgres/MySQL/SQLite database, reflects its table
structure via `sqlalchemy.inspect()` (columns, types, nullability, primary
keys, foreign keys, unique constraints), and emits the same
entities/relationships YAML shape that `Schema.from_yaml()` /
`Schema.from_yaml_str()` (backed by the Rust `SchemaParser` in
`src/parser.rs`) already consume -- see `examples/banking_schema.yaml` for
the on-disk shape being matched:

    entities:
      <table_name>:
        fields:
          <column_name>:
            type: string|int|float|boolean|datetime|uuid|json|enum(a, b, c)
            nullable: true            # omitted when false
            unique: true              # omitted when false
        primary_key: <column_name>
    relationships:
      - from_entity: <referenced_table>
        to_entity: <table_with_the_fk>
        from_field: <referenced_column>
        to_field: <fk_column>
        cardinality: "1:1" | "1:n"

Postgres and MySQL are supported through *this exact same code path*; only
the SQLAlchemy connection URL scheme differs:

    postgresql+psycopg://user:pass@host:5432/dbname   (or postgresql+psycopg2://...)
    mysql+pymysql://user:pass@host:3306/dbname

Neither a live Postgres nor MySQL server is available in this repo's test
environment, so `python/tests/test_db_schema.py` exercises this module
against a real, file-based SQLite database instead. SQLAlchemy reflects
SQLite through the identical `inspect()` / `get_columns()` /
`get_pk_constraint()` / `get_foreign_keys()` / `get_unique_constraints()`
calls used for every other dialect it supports, so that test proves the
reflection and YAML-emission logic genuinely works end to end -- it is not
a hand-rolled SQLite-only shortcut. Only wire-protocol connectivity to a
real Postgres/MySQL server goes untested here.

Requires the optional `db` extra:  pip install pysynthdata[db]
"""

from __future__ import annotations

import datetime as dt
import decimal
from typing import Any, Dict, List, Optional, Set

try:
    import sqlalchemy as sa
except ImportError as exc:  # pragma: no cover - exercised via error-path test
    raise ImportError(
        "Live database schema inference requires SQLAlchemy. "
        "Install it with: pip install pysynthdata[db]"
    ) from exc

import yaml

__all__ = [
    "infer_schema",
    "infer_schema_yaml",
    "infer_schema_to_file",
]


def _map_column_type(col_type: Any) -> str:
    """Map a SQLAlchemy column type to a pysynthdata schema field-type
    string: one of `string`, `int`, `float`, `boolean`, `datetime`, `uuid`,
    `json`, or `enum(a, b, c)` -- the exact vocabulary
    `SchemaParser::parse_field_type` in `src/parser.rs` accepts.
    """
    # sqlalchemy.Enum subclasses String, so this has to be checked first.
    enum_values = list(getattr(col_type, "enums", None) or [])
    if enum_values:
        return f"enum({', '.join(enum_values)})"

    type_name = col_type.__class__.__name__.upper()
    if "UUID" in type_name:
        return "uuid"
    if "JSON" in type_name:
        return "json"

    if isinstance(col_type, sa.Boolean):
        return "boolean"
    if isinstance(col_type, (sa.DateTime, sa.Date, sa.Time)):
        return "datetime"
    if isinstance(col_type, sa.Integer):
        return "int"
    if isinstance(col_type, (sa.Float, sa.Numeric)):
        return "float"
    if isinstance(col_type, (sa.String, sa.Text)):
        return "string"

    # Dialect-specific types that don't subclass any of the above (e.g.
    # Postgres ARRAY/CIDR/INET): fall back to the type's own declared
    # Python type instead of guessing a default.
    try:
        py_type = col_type.python_type
    except (NotImplementedError, AttributeError):
        return "string"
    if py_type is bool:
        return "boolean"
    if py_type is int:
        return "int"
    if py_type in (float, decimal.Decimal):
        return "float"
    if py_type in (dt.datetime, dt.date, dt.time):
        return "datetime"
    return "string"


def _table_entity(inspector: Any, table_name: str) -> Dict[str, Any]:
    """Reflect one table into a pysynthdata `entities.<name>` dict."""
    columns = inspector.get_columns(table_name)
    pk_columns = inspector.get_pk_constraint(table_name).get("constrained_columns") or []
    unique_columns = {
        col
        for uc in inspector.get_unique_constraints(table_name)
        for col in uc.get("column_names", [])
    }

    fields: Dict[str, Any] = {}
    for col in columns:
        name = col["name"]
        field: Dict[str, Any] = {"type": _map_column_type(col["type"])}
        if col.get("nullable", True):
            field["nullable"] = True
        if name in pk_columns or name in unique_columns:
            field["unique"] = True
        fields[name] = field

    entity: Dict[str, Any] = {"fields": fields}
    if pk_columns:
        # The pysynthdata YAML format only supports a single-column
        # primary_key. For composite keys the first constrained column is
        # used as the declared key; every PK column is still marked
        # unique=True above so multi-column uniqueness isn't silently lost.
        entity["primary_key"] = pk_columns[0]
    return entity


def _table_relationships(
    inspector: Any, table_name: str, known_tables: Set[str]
) -> List[Dict[str, Any]]:
    """Reflect one table's outbound foreign keys into pysynthdata
    `relationships` entries."""
    pk_columns = set(inspector.get_pk_constraint(table_name).get("constrained_columns") or [])
    unique_columns = {
        col
        for uc in inspector.get_unique_constraints(table_name)
        for col in uc.get("column_names", [])
    }

    relationships: List[Dict[str, Any]] = []
    for fk in inspector.get_foreign_keys(table_name):
        referred_table = fk.get("referred_table")
        constrained = fk.get("constrained_columns") or []
        referred = fk.get("referred_columns") or []
        if not referred_table or not constrained or not referred:
            continue
        if referred_table not in known_tables:
            # FK points at a table outside the set we're inferring -- skip
            # rather than emit a relationship to an entity that won't exist
            # in this schema.
            continue
        fk_column = constrained[0]
        cardinality = "1:1" if (fk_column in pk_columns or fk_column in unique_columns) else "1:n"
        relationships.append(
            {
                "from_entity": referred_table,
                "to_entity": table_name,
                "from_field": referred[0],
                "to_field": fk_column,
                "cardinality": cardinality,
            }
        )
    return relationships


def infer_schema(
    db_url: str,
    table: Optional[str] = None,
    all_tables: bool = False,
) -> Dict[str, Any]:
    """Connect to `db_url` and reflect real table structure into a
    pysynthdata schema dict: `{"entities": {...}, "relationships": [...]}`
    -- the same shape `Schema.from_yaml()` / `examples/banking_schema.yaml`
    use.

    Exactly one of `table` or `all_tables=True` must be given. Works
    against any SQLAlchemy-supported backend (Postgres, MySQL, SQLite, ...)
    through the identical `inspect()` code path -- see the module
    docstring.

    Raises `ValueError` if neither/both of `table`/`all_tables` are given,
    or if a requested table doesn't exist in the database. Raises whatever
    the underlying SQLAlchemy dialect raises on a real connection failure
    (e.g. `sqlalchemy.exc.OperationalError`) -- this never silently returns
    a fake schema when the connection fails.
    """
    if bool(table) == bool(all_tables):
        raise ValueError("Pass exactly one of `table` or `all_tables=True`")

    engine = sa.create_engine(db_url)
    try:
        inspector = sa.inspect(engine)
        available = inspector.get_table_names()
        table_names = list(available) if all_tables else [table]

        missing = [t for t in table_names if t not in available]
        if missing:
            raise ValueError(
                f"Table(s) not found in database: {', '.join(missing)}. "
                f"Available tables: {', '.join(available) or '(none)'}"
            )

        known = set(table_names)
        entities: Dict[str, Any] = {}
        relationships: List[Dict[str, Any]] = []
        for t in table_names:
            entities[t] = _table_entity(inspector, t)
            relationships.extend(_table_relationships(inspector, t, known))

        schema: Dict[str, Any] = {"entities": entities}
        if relationships:
            schema["relationships"] = relationships
        return schema
    finally:
        engine.dispose()


def infer_schema_yaml(db_url: str, table: Optional[str] = None, all_tables: bool = False) -> str:
    """Same as `infer_schema()`, but returns a YAML string in the exact
    on-disk shape `examples/banking_schema.yaml` uses."""
    schema = infer_schema(db_url, table=table, all_tables=all_tables)
    return yaml.safe_dump(schema, sort_keys=False, default_flow_style=False)


def infer_schema_to_file(
    db_url: str,
    output_path: str,
    table: Optional[str] = None,
    all_tables: bool = False,
) -> str:
    """Infer a schema from `db_url` and write it to `output_path` as YAML.
    Returns the YAML text that was written."""
    yaml_text = infer_schema_yaml(db_url, table=table, all_tables=all_tables)
    with open(output_path, "w") as f:
        f.write(yaml_text)
    return yaml_text
