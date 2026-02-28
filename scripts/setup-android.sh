#!/usr/bin/env bash
# One-time setup for Android development.
# Run this after installing JDK 17+ and the Android SDK.
#
# Prerequisites:
#   1. JDK 17+  (e.g. brew install --cask temurin@17)
#   2. Android SDK  (via Android Studio or sdkman)
#   3. Set ANDROID_HOME (e.g. export ANDROID_HOME=$HOME/Library/Android/sdk)
#   4. NDK r26+  (install from Android Studio → SDK Manager → SDK Tools → NDK)
#   5. cargo-ndk  (cargo install cargo-ndk)

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

echo "==> Checking prerequisites..."
command -v java   >/dev/null || { echo "ERROR: JDK not found. Install JDK 17+."; exit 1; }
command -v gradle >/dev/null || { echo "ERROR: Gradle not found. Install via 'brew install gradle' or sdkman."; exit 1; }
[[ -n "${ANDROID_HOME:-}" ]]  || { echo "ERROR: ANDROID_HOME not set."; exit 1; }

echo "==> Generating Gradle wrapper..."
gradle wrapper --gradle-version=8.7 --distribution-type=bin

echo "==> Installing Android Rust targets..."
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android

echo "==> Building plinth-core .so files for all ABIs..."
NDK_HOME="${ANDROID_HOME}/ndk/$(ls "${ANDROID_HOME}/ndk" | sort -V | tail -1)"
ANDROID_NDK_HOME="$NDK_HOME" cargo ndk \
  -t arm64-v8a \
  -t armeabi-v7a \
  -t x86_64 \
  -o packages/plinth-android/src/main/jniLibs \
  build -p plinth-core --release

echo ""
echo "Setup complete. You can now run:"
echo "  ./gradlew :plinth-android:assembleRelease"
echo "  ./gradlew :plinth-media3:assembleRelease"
echo "  ./gradlew :android-sample:app:installDebug"
