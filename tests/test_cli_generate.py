"""CLI generate/export tests for the documented schema."""

from __future__ import annotations

from pathlib import Path

import pytest

pytest.importorskip("pysynthdata._core")

from pysynthdata.cli import main

SCHEMA = """
entities:
  Customer:
    fields:
      id: {type: uuid, unique: true}
      name: {type: string}
    primary_key: id
  Order:
    fields:
      id: {type: uuid, unique: true}
      customer_id: {type: uuid}
      amount: {type: float}
    primary_key: id
relationships:
  - from_entity: Customer
    to_entity: Order
    from_field: id
    to_field: customer_id
    cardinality: 1:n
"""


def _schema(tmp_path: Path) -> Path:
    path = tmp_path / "schema.yaml"
    path.write_text(SCHEMA, encoding="utf-8")
    return path


@pytest.mark.parametrize("fmt,ext", [("csv", ".csv"), ("json", ".json"), ("parquet", ".parquet")])
def test_generate_writes_each_entity(tmp_path, fmt, ext):
    schema = _schema(tmp_path)
    out = tmp_path / "out"
    assert main(["generate", str(schema), "--out", str(out), "--rows", "4", "--format", fmt]) == 0
    assert (out / f"Customer{ext}").is_file()
    assert (out / f"Order{ext}").is_file()


def test_generate_is_seed_deterministic(tmp_path):
    schema = _schema(tmp_path)
    a = tmp_path / "a"
    b = tmp_path / "b"
    assert (
        main(
            [
                "generate",
                str(schema),
                "--out",
                str(a),
                "--rows",
                "3",
                "--seed",
                "1",
                "--format",
                "json",
            ]
        )
        == 0
    )
    assert (
        main(
            [
                "generate",
                str(schema),
                "--out",
                str(b),
                "--rows",
                "3",
                "--seed",
                "1",
                "--format",
                "json",
            ]
        )
        == 0
    )
    assert (a / "Customer.json").read_text() == (b / "Customer.json").read_text()


def test_generate_with_privacy_budget(tmp_path):
    schema = _schema(tmp_path)
    out = tmp_path / "dp"
    assert (
        main(
            [
                "generate",
                str(schema),
                "--out",
                str(out),
                "--rows",
                "3",
                "--format",
                "json",
                "--dp-epsilon",
                "1.0",
            ]
        )
        == 0
    )
    assert (out / "Order.json").is_file()


def test_missing_schema_fails_cleanly(tmp_path, capsys):
    code = main(["generate", str(tmp_path / "missing.yaml"), "--out", str(tmp_path / "out")])
    assert code == 1
    assert "error:" in capsys.readouterr().err
