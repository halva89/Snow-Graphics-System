#!/usr/bin/env bash
# Сборка main.frame под macOS (Apple Silicon arm64 и Intel x86_64).
# Запускать НА самом Mac: bash build-macos.sh
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$REPO_DIR"

ARCH="$(uname -m)"
case "$ARCH" in
  arm64)  TARGET="aarch64-apple-darwin" ;;
  x86_64) TARGET="x86_64-apple-darwin" ;;
  *)
    echo "Unsupported architecture: $ARCH" >&2
    exit 1
    ;;
esac
echo "==> Target: $TARGET (host arch: $ARCH)"

if ! command -v brew >/dev/null 2>&1; then
  echo "Нужен Homebrew: https://brew.sh" >&2
  exit 1
fi

# Vulkan-эмуляция через Metal (MoltenVK) + загрузчик Vulkan + GLFW.
brew install molten-vk vulkan-loader glfw

# Указываем драйвер MoltenVK (путь зависит от расположения Homebrew).
ICD="$(find /opt/homebrew/share/vulkan/icd.d "$HOME/homebrew/share/vulkan/icd.d" \
  -name 'MoltenVK_icd.json' -type f 2>/dev/null | head -1 || true)"
if [ -n "${ICD:-}" ]; then
  export VK_ICD_FILENAMES="$ICD"
  echo "==> VK_ICD_FILENAMES=$ICD"
else
  # Intel Mac / другой путь Homebrew: ищем пакетом
  export VK_ICD_FILENAMES="/opt/homebrew/share/vulkan/icd.d/MoltenVK_icd.json"
fi

echo "==> cargo build --release --target $TARGET"
cargo build --release --target "$TARGET"

BIN="target/$TARGET/release/main_frame"
DIST="dist/main_frame-macos-$ARCH"
mkdir -p "$DIST/shaders"
cp "$BIN" "$DIST/main_frame"
cp shaders/*.spv "$DIST/shaders/"

echo "==> Готово:"
echo "   $DIST/main_frame"
echo "   Запуск: cd $DIST && ./main_frame /путь/к/сцене.mff"