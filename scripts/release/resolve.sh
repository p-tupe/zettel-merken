#!/usr/bin/env bash
# Resolve the version for this release.
# USAGE: resolve.sh <patch|minor|major|retry>
set -euo pipefail

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

usage() {
  cat >&2 <<'EOF'
usage: resolve.sh <patch|minor|major|retry>
  patch|minor|major  bump the version in Cargo.toml and release it
  retry              release the version already in Cargo.toml, as-is
EOF
  exit 2
}

[ $# -eq 1 ] || usage

bump="$1"
case "$bump" in
patch | minor | major | retry) ;;
*) die "unknown release type '$bump'" ;;
esac

read_version() {
  if command -v cargo >/dev/null 2>&1; then
    cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version'
  elif command -v python3 >/dev/null 2>&1; then
    python3 -c 'import tomllib,sys; print(tomllib.load(open(sys.argv[1],"rb"))["package"]["version"])' Cargo.toml
  else
    die "neither cargo nor python3 is available: cannot read the version"
  fi
}

current=$(read_version)
printf 'current=%s\n' "$current" >&2

if [ "$bump" != "retry" ]; then
  if last=$(git describe --tags --abbrev=0 2>/dev/null); then
    since=$(git rev-list --count "$last"..HEAD)
    # Catches the second click of the button: the previous run already
    # committed its bump, so there is nothing new to release.
    [ "$since" -gt 0 ] ||
      die "nothing has changed since $last, so there is nothing to release"
    # Manifest ahead of the tags means a previous run died between the commit
    # and the tag. Bumping now would silently skip that version.
    git rev-parse -q --verify "refs/tags/v$current" >/dev/null ||
      die "Cargo.toml says $current but v$current is untagged; a previous run probably died before tagging. Re-run with 'retry' to finish $current"
  else
    printf 'no existing tags: treating this as the first release\n' >&2
  fi
fi

case "$bump" in
retry)
  version="$current"
  ;;
*)
  IFS=. read -r maj min pat <<<"$current"
  case "$bump" in
  major) version="$((maj + 1)).0.0" ;;
  minor) version="${maj}.$((min + 1)).0" ;;
  patch) version="${maj}.${min}.$((pat + 1))" ;;
  esac
  ;;
esac

tag="v$version"
printf 'version=%s\n' "$version"
printf 'tag=%s\n' "$tag"
