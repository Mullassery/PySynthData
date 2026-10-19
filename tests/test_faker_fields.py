"""Faker enrichment tests."""

from __future__ import annotations

import pytest

pytest.importorskip("faker")

from pysynthdata.faker_fields import apply_faker_fields, parse_spec


class _World:
    def __init__(self):
        self.data = {"Customer": [{"name": ""}, {"name": ""}]}


def test_apply_faker_fields():
    world = apply_faker_fields(_World(), [parse_spec("Customer.name=name")], seed=7)
    assert all(row["name"] for row in world.data["Customer"])


def test_parse_spec_rejects_bad_value():
    with pytest.raises(ValueError, match="Entity.field=provider"):
        parse_spec("nope")
