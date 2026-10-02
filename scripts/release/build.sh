#!/usr/bin/env bash
# Package one already-built binary into the tarball and checksum.
#
# USAGE: build.sh <target-triple>
#
# Assumes `cargo build --release --target <target-triple>` has already run.
# Writes dist/<name>-<version>-<target>.tar.gz and its .sha256.

set -euo pipefail

target="${1:?usage: build.sh <target-triple>}"

# bsdtar (macOS) embeds ._ AppleDouble members for files carrying
# extended attributes, which surprises anyone unpacking it on Linux. GNU tar
# ignores this.
export COPYFILE_DISABLE=1

meta=$(cargo metadata --no-deps --format-version 1)
name=$(jq -r '.packages[0].name' <<<"$meta")
version=$(jq -r '.packages[0].version' <<<"$meta")
bin=$(jq -r '.packages[0].targets[] | select(.kind[] == "bin") | .name' <<<"$meta" | head -n1)
binary="target/$target/release/$bin"
if [ ! -f "$binary" ]; then
  echo "error: no binary at $binary — build it first" >&2
  exit 1
fi

stem="$name-$version-$target"
stage="$(mktemp -d)/$stem"
mkdir -p "$stage"
cp "$binary" "$stage/"
for extra in README.md LICENSE.txt config.sample.json; do
  if [ -f "$extra" ]; then cp "$extra" "$stage/"; fi
done

mkdir -p dist
tar -czf "dist/$stem.tar.gz" -C "$(dirname "$stage")" "$stem"
if command -v sha256sum >/dev/null 2>&1; then
  (cd dist && sha256sum "$stem.tar.gz" >"$stem.tar.gz.sha256")
else
  (cd dist && shasum -a 256 "$stem.tar.gz" >"$stem.tar.gz.sha256")
fi
echo "dist/$stem.tar.gz"
echo "dist/$stem.tar.gz.sha256"
