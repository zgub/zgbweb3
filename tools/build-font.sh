#!/usr/bin/env bash
# Rebuild crates/app/assets/fonts/Nunito-Subset.ttf from the upstream variable font.
#   tools/build-font.sh path/to/Nunito[wght].ttf
# Upstream: https://github.com/google/fonts/tree/main/ofl/nunito (OFL 1.1, no reserved name)
set -euo pipefail
src="$1"
out="$(dirname "$0")/../crates/app/assets/fonts/Nunito-Subset.ttf"
tmp="$(mktemp -d)"
python3 -I "$(dirname "$0")/add_superscript_minus.py" "$src" "$tmp/with-minus.ttf"
# Latin + Latin Extended (Slovak), Greek, punctuation, super/subscripts, arrows, maths, shapes.
pyftsubset "$tmp/with-minus.ttf" \
  --unicodes="U+0000-024F,U+02B0-02FF,U+0300-036F,U+0370-03FF,U+1E00-1EFF,U+2000-206F,U+2070-209F,U+20A0-20CF,U+2100-214F,U+2150-218F,U+2190-21FF,U+2200-22FF,U+2300-23FF,U+25A0-25FF" \
  --layout-features='*' --output-file="$out"
rm -rf "$tmp"
echo "wrote $out"
