"""Regression test found via real-world benchmarking against a real dataset
(Palmer Penguins): Schema.add_field("...", "enum(Adelie,Gentoo,Chinstrap)")
silently lowercased every declared value ("Adelie" -> "adelie"), corrupting
fidelity to any real data the schema was modeling. Root cause: lib.rs's
parse_field_type matched the "enum(" keyword against a lowercased copy of
the type string, then extracted the enum values from that same lowercased
copy instead of the original.
"""
from pysynthdata import Schema, WorldGenerator


def test_add_field_preserves_declared_enum_casing():
    schema = Schema()
    schema.add_entity("penguins")
    schema.add_field("penguins", "species", "enum(Adelie,Gentoo,Chinstrap)")

    generator = WorldGenerator(schema)
    world = generator.generate(num_records=50, seed=1)
    values = set(world.to_pandas("penguins")["species"].unique())

    assert values <= {"Adelie", "Gentoo", "Chinstrap"}, (
        f"expected declared casing to be preserved, got {values}"
    )
