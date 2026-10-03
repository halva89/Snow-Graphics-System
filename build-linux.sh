#!/usr/bin/env bash
# Сборка main.frame под Linux (и x86_64, и ARM64).
# Запускать НА самой Linux-машине: bash build-linux.sh
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$REPO_DIR"

ARCH="$(uname -m)"
case "$ARCH" in
  x86_64)                     TARGET="x86_64-unknown-linux-gnu" ;;
  aarch64|arm64|aarch64_be)   TARGET="aarch64-unknown-linux-gnu" ;;
  *)
    echo "Unsupported architecture: $ARCH" >&2
    exit 1
    ;;
esac
echo "==> Target: $TARGET (host arch: $ARCH)"

# Системные зависимости для GLFW (X11/Wayland) и Vulkan.
if command -v apt-get >/dev/null 2>&1; then
  sudo apt-get update
  sudo apt-get install -y \
    build-essential pkg-config libvulkan-dev \
    libx11-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev \
    libxkbcommon-dev wayland-protocols libwayland-dev
elif command -v dnf >/dev/null 2>&1; then
  sudo dnf install -y \
    gcc-c++ pkgconf-pkg-config vulkan-loader-devel \
    libX11-devel libXrandr-devel libXinerama-devel libXcursor-devel libXi-devel \
    libxkbcommon-devel wayland-devel wayland-protocols-devel
else
  echo "Unsupported package manager — установите dev-библиотеки GLFW и Vulkan вручную." >&2
fi

echo "==> cargo build --release --target $TARGET"
cargo build --release --target "$TARGET"

BIN="target/$TARGET/release/main_frame"
DIST="dist/main_frame-linux-$ARCH"
mkdir -p "$DIST/shaders"
cp "$BIN" "$DIST/main_frame"
cp shaders/*.spv "$DIST/shaders/"

echo "==> Готово:"
echo "   $DIST/main_frame"
echo "   Запуск: cd $DIST && ./main_frame /путь/к/сцене.mff"
echo "   (добавьте права: chmod +x $DIST/main_frame, если файловая система их сбросила)"