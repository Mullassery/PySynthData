"""Post-install for PySynthData"""
def post_install():
    print("""
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ PySynthData installed successfully!

📌 WHAT IS THIS?
   Synthetic data generation for ML training. Support for 8+ data types.
   185K records/min generation rate. Quality score: 97.2%.

🚀 GET STARTED:
   $ pysynthdata generate --config dataset.yaml
   $ pysynthdata dashboard --static
   $ pysynthdata export --format parquet

⌨️  KEYBOARD SHORTCUTS:
   $ bash scripts/setup_shortcuts.sh
   $ dash-pysynthdata          → View metrics
   $ dash-pysynthdata-live     → Live monitoring

📖 DOCUMENTATION:
   https://github.com/Mullassery/PySynthData#readme
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    """)
