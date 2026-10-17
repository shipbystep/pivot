#!/usr/bin/env bash
# Checks the "Code shape" limits that clippy cannot see:
# lines per file and top-level items per file.
# Counts lines that start a top-level item after rustfmt, so it is a close
# estimate, not a parser.
set -euo pipefail

max_lines=${MAX_LINES:-150}
max_items=${MAX_ITEMS:-9}
item='^(pub(\([a-z]+\))? )?(const |async )?(fn|struct|enum|trait|type|const|static|mod|impl|macro_rules!) '
failed=0

for file in $(find src tests -name '*.rs' | sort); do
  lines=$(wc -l < "$file" | tr -d ' ')
  # Stop counting at the test module: tests are checked by their own size.
  items=$(sed '/^#\[cfg(test)\]/,$d' "$file" | grep -cE "$item" || true)
  if (( lines > max_lines )); then
    echo "$file: $lines lines (max $max_lines)"; failed=1
  fi
  if (( items > max_items )); then
    echo "$file: $items top-level items (max $max_items)"; failed=1
  fi
done

if (( failed == 0 )); then echo "shape: ok (files <= $max_lines lines, <= $max_items items)"; fi
exit $failed
