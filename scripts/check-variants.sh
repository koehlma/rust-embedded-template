#!/usr/bin/env bash
# Generates every template variant and checks that it builds without warnings.
set -euo pipefail

repo_dir="$(cd "$(dirname "$0")/.." && pwd)"
out_dir="${1:-${TMPDIR:-/tmp}/rust-embedded-template-variants}"

rm -rf "$out_dir"
mkdir -p "$out_dir"

check_variant() {
    local name="$1"
    shift
    echo "::group::$name"
    cargo generate --path "$repo_dir" --name "$name" --destination "$out_dir" --vcs none --silent "$@"
    (
        cd "$out_dir/$name"
        cargo fmt --check
        cargo clippy --release -- -D warnings
        cargo build --release
    )
    echo "::endgroup::"
}

for chip in rp2040 rp2350; do
    check_variant "$chip-rp-hal" -d chip="$chip" -d wifi=false -d embassy=false
    check_variant "$chip-embassy" -d chip="$chip" -d wifi=false -d embassy=true
    check_variant "$chip-embassy-wifi" -d chip="$chip" -d wifi=true
done
