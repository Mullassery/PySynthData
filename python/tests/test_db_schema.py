"""Tests for `pysynthdata.db_schema` -- live database schema inference.

These run against a real, file-based SQLite database (via a `tmp_path`
fixture, not an in-memory mock) rather than Postgres/MySQL, since no live
Postgres/MySQL server is available in this repo's test environment.
SQLAlchemy reflects SQLite through the exact same `inspect()` /
`get_columns()` / `get_pk_constraint()` / `get_foreign_keys()` /
`get_unique_constraints()` calls used for every other backend it supports
(see `db_schema.py`'s module docstring) -- connecting to Postgres or MySQL
only changes the `--db-url` scheme (`postgresql+psycopg://...` /
`mysql+pymysql://...`), not the reflection code path exercised here.
"""

import sqlalchemy as sa
import yaml
import pytest

from pysynthdata.db_schema import infer_schema, infer_schema_to_file, infer_schema_yaml


@pytest.fixture
def sqlite_db(tmp_path):
    """A real on-disk SQLite database with two tables (`customers`,
    `orders`) covering every field-type/nullable/unique/PK/FK case the
    inference logic needs to handle, linked by a real ForeignKey
    constraint."""
    db_path = tmp_path / "test.db"
    db_url = f"sqlite:///{db_path}"
    engine = sa.create_engine(db_url)
    metadata = sa.MetaData()

    sa.Table(
        "customers",
        metadata,
        sa.Column("id", sa.Integer, primary_key=True),
        sa.Column("name", sa.String(100), nullable=False),
        sa.Column("email", sa.String(255), nullable=False, unique=True),
        sa.Column("age", sa.Integer, nullable=True),
        sa.Column("is_active", sa.Boolean, nullable=False),
        sa.Column("balance", sa.Float, nullable=True),
        sa.Column("created_at", sa.DateTime, nullable=False),
    )
    sa.Table(
        "orders",
        metadata,
        sa.Column("id", sa.Integer, primary_key=True),
        sa.Column(
            "customer_id", sa.Integer, sa.ForeignKey("customers.id"), nullable=False
        ),
        sa.Column("amount", sa.Float, nullable=False),
        sa.Column("status", sa.String(20), nullable=True),
    )
    metadata.create_all(engine)
    engine.dispose()
    yield db_url


def test_infer_single_table_columns_types_pk_unique_nullable(sqlite_db):
    schema = infer_schema(sqlite_db, table="customers")

    assert set(schema["entities"]) == {"customers"}
    customer = schema["entities"]["customers"]
    assert customer["primary_key"] == "id"

    fields = customer["fields"]
    assert fields["id"] == {"type": "int", "unique": True}
    assert fields["name"] == {"type": "string"}
    assert fields["email"] == {"type": "string", "unique": True}
    assert fields["age"] == {"type": "int", "nullable": True}
    assert fields["is_active"] == {"type": "boolean"}
    assert fields["balance"] == {"type": "float", "nullable": True}
    assert fields["created_at"] == {"type": "datetime"}

    # Single-table inference must not fabricate relationships to tables
    # that weren't part of the request.
    assert "relationships" not in schema


def test_infer_all_tables_detects_foreign_key_relationship(sqlite_db):
    schema = infer_schema(sqlite_db, all_tables=True)

    assert set(schema["entities"]) == {"customers", "orders"}
    assert schema["entities"]["orders"]["primary_key"] == "id"
    assert schema["entities"]["orders"]["fields"]["customer_id"] == {"type": "int"}

    assert len(schema["relationships"]) == 1
    rel = schema["relationships"][0]
    assert rel == {
        "from_entity": "customers",
        "to_entity": "orders",
        "from_field": "id",
        "to_field": "customer_id",
        "cardinality": "1:n",
    }


def test_infer_schema_yaml_round_trips_through_real_yaml_parser(sqlite_db):
    yaml_text = infer_schema_yaml(sqlite_db, all_tables=True)
    parsed = yaml.safe_load(yaml_text)

    assert parsed == infer_schema(sqlite_db, all_tables=True)
    # Matches the on-disk shape examples/banking_schema.yaml uses.
    assert "fields" in parsed["entities"]["customers"]
    assert "primary_key" in parsed["entities"]["customers"]


def test_infer_schema_to_file_writes_yaml_to_disk(sqlite_db, tmp_path):
    out_path = tmp_path / "inferred_schema.yaml"

    yaml_text = infer_schema_to_file(sqlite_db, str(out_path), all_tables=True)

    assert out_path.exists()
    assert out_path.read_text() == yaml_text
    parsed = yaml.safe_load(out_path.read_text())
    assert set(parsed["entities"]) == {"customers", "orders"}


def test_missing_table_raises_clear_error(sqlite_db):
    with pytest.raises(ValueError, match="not found"):
        infer_schema(sqlite_db, table="does_not_exist")


def test_requires_exactly_one_of_table_or_all_tables(sqlite_db):
    with pytest.raises(ValueError):
        infer_schema(sqlite_db)
    with pytest.raises(ValueError):
        infer_schema(sqlite_db, table="customers", all_tables=True)


def test_inferred_schema_generates_real_rows_via_rust_core(sqlite_db):
    """End-to-end proof that this isn't YAML-shaped decoration: the schema
    inferred from a real SQLite database loads into the actual Rust-backed
    `pysynthdata.Schema` (via `Schema.from_yaml_str`, which goes through the
    same `SchemaParser` as any hand-written schema YAML) and produces real,
    foreign-key-consistent generated rows.
    """
    from pysynthdata import Schema, WorldGenerator

    yaml_text = infer_schema_yaml(sqlite_db, all_tables=True)
    schema = Schema.from_yaml_str(yaml_text)
    assert set(schema.entities) == {"customers", "orders"}

    world = WorldGenerator(schema).generate(num_records=10, seed=1)
    assert len(world.data["customers"]) == 10
    assert len(world.data["orders"]) == 10

    customer_ids = {row["id"] for row in world.data["customers"]}
    assert all(row["customer_id"] in customer_ids for row in world.data["orders"])


def test_cli_infer_schema_writes_expected_file(sqlite_db, tmp_path, capsys):
    """Exercises the actual `pysynthdata infer-schema` CLI command
    end-to-end (argument parsing through file output), not just the
    underlying `db_schema` functions."""
    from pysynthdata.cli import main

    out_path = tmp_path / "cli_schema.yaml"
    exit_code = main(
        [
            "infer-schema",
            "--db-url",
            sqlite_db,
            "--all-tables",
            "--output",
            str(out_path),
        ]
    )

    assert exit_code == 0
    assert out_path.exists()
    parsed = yaml.safe_load(out_path.read_text())
    assert set(parsed["entities"]) == {"customers", "orders"}

    captured = capsys.readouterr()
    assert "Wrote schema" in captured.out


def test_cli_infer_schema_reports_missing_table_error(sqlite_db, tmp_path, capsys):
    from pysynthdata.cli import main

    out_path = tmp_path / "cli_schema.yaml"
    exit_code = main(
        [
            "infer-schema",
            "--db-url",
            sqlite_db,
            "--table",
            "nope",
            "--output",
            str(out_path),
        ]
    )

    assert exit_code == 1
    assert not out_path.exists()
    captured = capsys.readouterr()
    assert "error" in captured.err
