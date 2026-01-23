#!/usr/bin/env bash
set -euo pipefail

if [[ -z "${DEVBOX_SHELL_ENABLED:-}" ]]; then
  echo "Please run inside devbox: devbox shell"
  exit 1
fi

ROOT_DIR=$(pwd)
TMPDIR=$(mktemp -d)
cd "$TMPDIR"

if [[ "${USE_LOCAL:-0}" == "1" ]]; then
  echo "Building local plugins..."
  (cd "$ROOT_DIR" && make build)
  cp "$ROOT_DIR/build/"*.wasm .
else
  echo "Downloading formatters from latest release..."
  RELEASE_URL="https://github.com/mridang/dprint-plugin-rusty/releases/latest/download"
  curl -sL "$RELEASE_URL/mago.wasm" -o mago.wasm
  curl -sL "$RELEASE_URL/stylua.wasm" -o stylua.wasm
  curl -sL "$RELEASE_URL/rumdl.wasm" -o rumdl.wasm
  curl -sL "$RELEASE_URL/alejandra.wasm" -o alejandra.wasm
fi

write_config() {
  local includes=( "**/*.php" "**/*.phtml" "**/*.lua" "**/*.md" "**/*.nix" )
  local plugins=( "./mago.wasm" "./stylua.wasm" "./rumdl.wasm" "./alejandra.wasm" )

  {
    echo '{'
    echo '  "includes": ['
    for i in "${!includes[@]}"; do
      local comma=","
      if [[ "$i" -eq "$((${#includes[@]} - 1))" ]]; then
        comma=""
      fi
      printf '    "%s"%s\n' "${includes[$i]}" "$comma"
    done
    echo '  ],'
    echo '  "plugins": ['
    for i in "${!plugins[@]}"; do
      local comma=","
      if [[ "$i" -eq "$((${#plugins[@]} - 1))" ]]; then
        comma=""
      fi
      printf '    "%s"%s\n' "${plugins[$i]}" "$comma"
    done
    echo '  ]'
    echo '}'
  } > dprint.json
}

write_config

cat > main.php << 'EOF'
<?php
class Sample{
function greet( $name ){
echo "Hi, $name";
}
}
EOF

cat > main.lua << 'EOF'
local function greet(  name )
print(  "hi "..name  )
end
greet("world")
EOF

cat > doc.md << 'EOF'
# Title

This  is  markdown with  bad spacing.
EOF

cat > main.nix << 'EOF'
{   pkgs   ?   import <nixpkgs> {}   }:
pkgs.mkShell {
buildInputs   =   [   pkgs.hello   ];
}
EOF

cp main.php main.php.bak
cp main.lua main.lua.bak
cp doc.md doc.md.bak
cp main.nix main.nix.bak

echo "Formatting..."
dprint fmt --log-level=warn || true

echo "=== main.php ==="
diff -y --width=150 main.php.bak main.php || true
echo -e "\n=== main.lua ==="
diff -y --width=150 main.lua.bak main.lua || true
echo -e "\n=== doc.md ==="
diff -y --width=150 doc.md.bak doc.md || true
echo -e "\n=== main.nix ==="
diff -y --width=150 main.nix.bak main.nix || true

echo -e "\nTest directory: $TMPDIR"
if [[ "${USE_LOCAL:-0}" == "1" ]]; then
  echo "Local plugins built from: $ROOT_DIR"
fi
