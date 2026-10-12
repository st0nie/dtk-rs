#!/usr/bin/env bash
# Complete local regression: native Qt/DTK development packages are required.
set -euo pipefail
cd "$(dirname "$0")/.."

printf '\n== Toolchain and native versions ==\n'
rustc --version
cargo --version
pkg-config --modversion Qt6Widgets dtk6widget dtk6gui dtk6core

printf '\n== Generator tests and reproducibility ==\n'
uv run --no-project python -m unittest tools.test_gen
generated=(
    dtk/src/{widgets,core,gui}.rs
    dtk-sys/src/{gen,core,gui}_ffi.rs
    dtk-sys/cpp/dtk_{gen,core,gui}_shim.cpp
    dtk-sys/include/dtk_{gen,core,gui}_shim.h
    GEN_REPORT.md GEN_CORE_REPORT.md GEN_GUI_REPORT.md
)
manifest=$(mktemp)
trap 'rm -f "$manifest"' EXIT
sha256sum "${generated[@]}" > "$manifest"
uv run --no-project python tools/gen.py
sha256sum --check "$manifest"
cargo fmt --all -- --check

for profile in debug release; do
    flags=()
    if [[ "$profile" == release ]]; then flags+=(--release); fi
    printf '\n== %s check/lint/build/tests ==\n' "$profile"
    cargo check --workspace --all-targets "${flags[@]}"
    cargo clippy --workspace --all-targets "${flags[@]}" -- -D warnings
    cargo build --workspace --all-targets "${flags[@]}"
    cargo test --workspace --all-targets "${flags[@]}"
    # --all-targets does not run doctests, including the scope escape tests.
    cargo test --workspace --doc "${flags[@]}"

    printf '\n== Every %s example (GUI offscreen / Core headless) ==\n' "$profile"
    count=0
    for source in dtk/examples/*.rs dtk-sys/examples/*.rs; do
        example=$(basename "${source%.rs}")
        binary="target/$profile/examples/$example"
        args=()
        case "$example" in
            hello|controls|demo|paint_widget|mpsc_eventfd) args+=(--smoke) ;;
        esac
        printf '\n-- %s: %s --\n' "$profile" "$example"
        case "$example" in
            core_application_selfcheck|core_paths_selfcheck)
                env -u DISPLAY -u WAYLAND_DISPLAY -u QT_QPA_PLATFORM \
                    timeout 90 "$binary" "${args[@]}" ;;
            *)
                QT_QPA_PLATFORM=offscreen timeout 90 "$binary" "${args[@]}" ;;
        esac
        count=$((count + 1))
    done
    printf '\nPASS: %d %s examples\n' "$count" "$profile"
    # The Qt6/DTK configuration-thread teardown race was intermittent between
    # processes; one invocation or only repeated scopes in one process missed it.
    printf '\n== %s native shutdown stress: 32 fresh processes ==\n' "$profile"
    for attempt in {1..32}; do
        printf '\n-- %s shutdown stress %d/32 --\n' "$profile" "$attempt"
        QT_QPA_PLATFORM=offscreen timeout 90 \
            "target/$profile/examples/value_types_selfcheck"
    done
    printf '\nPASS: 32 %s fresh-process shutdown checks\n' "$profile"
done
printf '\nPASS: complete Debug/Release validation\n'
