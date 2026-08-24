"""Tests for the new differential-privacy budget and streaming
generation/export API (PrivacyBudget, WorldGenerator.generate_private,
.generate_streaming, .to_parquet_streaming).

This is the first pytest suite in this repo -- previously all automated
coverage was the Rust `cargo test` suite (see README's "Development"
section). These tests exercise the Python-layer wrapper logic specifically
(threading/queue plumbing for streaming, PrivacyBudget's PyO3 boundary,
pyarrow ParquetWriter integration) that the Rust unit tests can't reach.
"""

import os
import tempfile

import pyarrow.parquet as pq
import pytest

from pysynthdata import PrivacyBudget, Schema, WorldGenerator


def _relational_schema() -> Schema:
    schema = Schema()
    schema.add_entity("customers")
    schema.add_field("customers", "id", "uuid", unique=True)
    schema.add_field("customers", "age", "int")
    schema.add_constraint("range", "customers", "18-90", field="age")

    schema.add_entity("orders")
    schema.add_field("orders", "id", "uuid", unique=True)
    schema.add_field("orders", "customer_id", "uuid")
    schema.add_field("orders", "amount", "float")
    schema.add_constraint("range", "orders", "1.0-5000.0", field="amount")
    schema.add_relationship("customers", "orders", "id", "customer_id", "1:n")
    return schema


class TestPrivacyBudget:
    def test_construction_and_defaults(self):
        budget = PrivacyBudget(epsilon=1.0, delta=1e-5)
        assert budget.total_epsilon == 1.0
        assert budget.total_delta == 1e-5
        assert budget.spent_epsilon == 0.0
        assert budget.remaining_epsilon == 1.0
        assert budget.is_exhausted is False

    def test_rejects_invalid_epsilon(self):
        with pytest.raises(ValueError):
            PrivacyBudget(epsilon=0.0, delta=0.0)
        with pytest.raises(ValueError):
            PrivacyBudget(epsilon=-1.0, delta=0.0)

    def test_rejects_invalid_delta(self):
        with pytest.raises(ValueError):
            PrivacyBudget(epsilon=1.0, delta=1.0)
        with pytest.raises(ValueError):
            PrivacyBudget(epsilon=1.0, delta=-0.1)


class TestGeneratePrivate:
    def test_returns_privacy_report(self):
        gen = WorldGenerator(_relational_schema())
        budget = PrivacyBudget(epsilon=1.0, delta=1e-5)

        world = gen.generate_private(50, seed=1, budget=budget, epsilon=0.4)

        report = world.privacy_report
        assert report is not None
        assert report["epsilon_spent"] == 0.4
        assert set(report["fields_privatized"]) == {"customers.age", "orders.amount"}
        assert report["values_perturbed"] == 100  # 50 customers.age + 50 orders.amount
        assert budget.spent_epsilon == 0.4
        assert budget.remaining_epsilon == pytest.approx(0.6)

    def test_plain_generate_has_no_privacy_report(self):
        gen = WorldGenerator(_relational_schema())
        world = gen.generate(10, seed=1)
        assert world.privacy_report is None

    def test_noised_values_still_respect_declared_range_constraint(self):
        gen = WorldGenerator(_relational_schema())
        budget = PrivacyBudget(epsilon=0.1, delta=0.0)  # small epsilon -> large noise

        world = gen.generate_private(200, seed=7, budget=budget, epsilon=0.1)

        ages = [row["age"] for row in world.data["customers"]]
        amounts = [row["amount"] for row in world.data["orders"]]
        assert all(18 <= a <= 90 for a in ages)
        assert all(1.0 <= a <= 5000.0 for a in amounts)

    def test_overspend_raises_and_leaves_budget_unchanged(self):
        gen = WorldGenerator(_relational_schema())
        budget = PrivacyBudget(epsilon=1.0, delta=0.0)
        gen.generate_private(10, seed=1, budget=budget, epsilon=0.7)

        with pytest.raises(ValueError, match="exhausted"):
            gen.generate_private(10, seed=1, budget=budget, epsilon=0.5)

        # Failed spend must not partially apply.
        assert budget.spent_epsilon == pytest.approx(0.7)

    def test_schema_with_no_numeric_fields_raises(self):
        schema = Schema()
        schema.add_entity("tags")
        schema.add_field("tags", "id", "uuid", unique=True)
        schema.add_field("tags", "label", "string")
        gen = WorldGenerator(schema)
        budget = PrivacyBudget(epsilon=1.0, delta=0.0)

        with pytest.raises(ValueError):
            gen.generate_private(10, seed=1, budget=budget, epsilon=0.5)


class TestGenerateStreaming:
    def test_yields_all_rows_in_bounded_chunks(self):
        gen = WorldGenerator(_relational_schema())

        chunks = list(gen.generate_streaming(237, seed=3, chunk_size=50, queue_size=2))

        by_entity = {}
        for entity_name, rows in chunks:
            assert len(rows) <= 50
            by_entity.setdefault(entity_name, 0)
            by_entity[entity_name] += len(rows)

        assert by_entity == {"customers": 237, "orders": 237}

    def test_matches_non_streaming_generate_for_same_seed(self):
        gen = WorldGenerator(_relational_schema())

        world = gen.generate(80, seed=11)
        streamed_customers = []
        for entity_name, rows in gen.generate_streaming(80, seed=11, chunk_size=16):
            if entity_name == "customers":
                streamed_customers.extend(rows)

        assert streamed_customers == world.data["customers"]

    def test_propagates_errors_from_worker_thread(self):
        gen = WorldGenerator(_relational_schema())

        with pytest.raises(ValueError):
            list(gen.generate_streaming(10, seed=1, chunk_size=0))  # chunk_size must be > 0


class TestToParquetStreaming:
    def test_writes_one_parquet_file_per_entity_with_correct_row_counts(self):
        gen = WorldGenerator(_relational_schema())

        with tempfile.TemporaryDirectory() as out_dir:
            counts = gen.to_parquet_streaming(120, seed=5, out_dir=out_dir, chunk_size=25)

            assert counts == {"customers": 120, "orders": 120}
            assert sorted(os.listdir(out_dir)) == ["customers.parquet", "orders.parquet"]

            customers_table = pq.read_table(os.path.join(out_dir, "customers.parquet"))
            orders_table = pq.read_table(os.path.join(out_dir, "orders.parquet"))
            assert customers_table.num_rows == 120
            assert orders_table.num_rows == 120

    def test_foreign_keys_survive_the_streaming_parquet_round_trip(self):
        gen = WorldGenerator(_relational_schema())

        with tempfile.TemporaryDirectory() as out_dir:
            gen.to_parquet_streaming(60, seed=9, out_dir=out_dir, chunk_size=10)

            customer_ids = set(
                pq.read_table(os.path.join(out_dir, "customers.parquet")).column("id").to_pylist()
            )
            order_customer_ids = (
                pq.read_table(os.path.join(out_dir, "orders.parquet")).column("customer_id").to_pylist()
            )

            assert set(order_customer_ids).issubset(customer_ids)
