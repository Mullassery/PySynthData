#!/bin/bash
add_shortcuts() {
  if [ -f ~/.zshrc ]; then RC_FILE=~/.zshrc
  elif [ -f ~/.bashrc ]; then RC_FILE=~/.bashrc
  else echo "❌ No shell config found"; return 1; fi
  
  if grep -q "dash-pysynthdata" "$RC_FILE"; then
    echo "⚠️  Already installed"; return 0
  fi
  
  cat >> "$RC_FILE" << 'ALIASES'

# PySynthData shortcuts
alias dash-pysynthdata='pysynthdata dashboard --static'
alias dash-pysynthdata-live='pysynthdata dashboard'
alias dash-pysynthdata-export='pysynthdata dashboard --export /tmp/pysynthdata_metrics.json && echo ✓ Exported'
ALIASES
  
  echo "✅ Shortcuts added"; echo "   Run: source $RC_FILE"
}
remove_shortcuts() {
  sed -i '' '/# PySynthData shortcuts/,/alias dash-pysynthdata-export=/d' ~/.zshrc 2>/dev/null
  sed -i '' '/# PySynthData shortcuts/,/alias dash-pysynthdata-export=/d' ~/.bashrc 2>/dev/null
  echo "✅ Removed"
}
case "${1:-}" in --remove) remove_shortcuts ;; *) add_shortcuts ;; esac
