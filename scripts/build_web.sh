#!/usr/bin/env bash
set -eu
script_path=$(
  cd "$(dirname "${BASH_SOURCE[0]}")"
  pwd -P
)
cd "$script_path/.."

./scripts/setup_web.sh

CRATE_NAME="tcg-bound"

OPEN=false
OPTIMIZE=false
BUILD=debug
BUILD_FLAGS=""
WASM_OPT_FLAGS=(-O2 --fast-math)

while test $# -gt 0; do
  case "$1" in
  -h | --help)
    echo "build_demo_web.sh [--release] [--wgpu] [--open]"
    echo ""
    echo "  -g:        Keep debug symbols even with --release."
    echo "             These are useful profiling and size trimming."
    echo ""
    echo "  --open:    Open the result in a browser."
    echo ""
    echo "  --release: Build with --release, and then run wasm-opt."
    echo "             NOTE: --release also removes debug symbols, unless you also use -g."
    exit 0
    ;;

  -g)
    shift
    WASM_OPT_FLAGS="${WASM_OPT_FLAGS} -g"
    ;;

  --open)
    shift
    OPEN=true
    ;;

  --release)
    shift
    OPTIMIZE=true
    BUILD="release"
    BUILD_FLAGS="--release"
    ;;

  *)
    echo "Unknown option: $1"
    exit 1
    ;;
  esac
done

OUT_FILE_NAME="tcg_bound_app"
FINAL_WASM_PATH=tcg_bound_app/${OUT_FILE_NAME}_bg.wasm

# Clear output from old stuff:
rm -f "$FINAL_WASM_PATH"

echo "Building rust…"

cargo build \
  "$BUILD_FLAGS" \
  --quiet \
  --target wasm32-unknown-unknown

# Get the output directory (in the workspace it is in another location)
# TARGET=`cargo metadata --format-version=1 | jq --raw-output .target_directory`
TARGET="target"

echo "Generating JS bindings for wasm…"
TARGET_NAME="${CRATE_NAME}.wasm"
WASM_PATH="${TARGET}/wasm32-unknown-unknown/$BUILD/$TARGET_NAME"
wasm-bindgen "$WASM_PATH" --out-dir tcg_bound_app --out-name "$OUT_FILE_NAME" --no-modules --no-typescript

# if this fails with "error: cannot import from modules (`env`) with `--no-modules`", you can use:
# wasm2wat target/wasm32-unknown-unknown/release/egui_demo_app.wasm | rg env
# wasm2wat target/wasm32-unknown-unknown/release/egui_demo_app.wasm | rg "call .now\b" -B 20 # What calls `$now` (often a culprit)
# Or use https://rustwasm.github.io/twiggy/usage/command-line-interface/paths.html#twiggy-paths

# to get wasm-strip:  apt/brew/dnf install wabt
# wasm-strip ${FINAL_WASM_PATH}

if [[ "$OPTIMIZE" = true ]]; then
  echo "Optimizing wasm…"
  # to get wasm-opt:  apt/brew/dnf install binaryen
  wasm-opt "$FINAL_WASM_PATH" "$WASM_OPT_FLAGS" -o "$FINAL_WASM_PATH"
fi

echo "Finished ${FINAL_WASM_PATH}"

if [[ "$OPEN" == true ]]; then
  if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux, ex: Fedora
    xdg-open http://localhost:8765/index.html
  elif [[ "$OSTYPE" == "msys" ]]; then
    # Windows
    start http://localhost:8765/index.html
  else
    # Darwin/MacOS, or something else
    open http://localhost:8765/index.html
  fi
fi
