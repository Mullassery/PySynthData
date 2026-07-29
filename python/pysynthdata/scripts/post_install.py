"""Post-install messaging for PySynthData"""

def post_install():
    print("""
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ PySynthData installed successfully!

📌 WHAT IS THIS?
   Synthetic data generation

🚀 GET STARTED:
   $ python3 -c "from pysynthdata import *; print('PySynthData ready')"
   $ python3 -c "import pysynthdata; print(f'v{pysynthdata.__version__ if hasattr(pysynthdata, \"__version__\") else \"latest\"}')"

📖 DOCUMENTATION:
   Repo:     https://github.com/Mullassery/PySynthData
   Tutorials: https://github.com/Mullassery/PySynthData#readme
   Issues:    https://github.com/Mullassery/PySynthData/issues

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    """)

if __name__ == "__main__":
    post_install()
