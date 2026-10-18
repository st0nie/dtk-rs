#!/usr/bin/env bash
# Build and install the DTK6 release dtk-rs is generated against, from source.
#
# The generated bindings, GEN_*/OWNERSHIP_* reports and their reproducibility
# check in tools/check.sh depend on the exact DTK headers, so CI pins the tag
# instead of taking whatever version a distribution currently ships.
#
# Usage: tools/ci/build-dtk.sh [prefix]   (default /usr; needs Qt6 dev packages)
# Later DTK modules locate earlier ones (and dtkcore's DConfig JSON under
# /usr/share/dsg) at their install location, so each module is installed into
# the prefix directly. With DTK_STAGE set, every module is also installed under
# that DESTDIR, e.g. for caching the build.
set -euo pipefail

DTK_TAG=${DTK_TAG:-6.7.47}
TREELAND_PROTOCOLS_TAG=${TREELAND_PROTOCOLS_TAG:-0.5.10}
PREFIX=${1:-/usr}
WORK=${DTK_BUILD_DIR:-$(mktemp -d)}

fetch() {
    local repo=$1 tag=$2
    if [[ ! -d "$WORK/$repo" ]]; then
        git clone --quiet --depth 1 --branch "$tag" "https://github.com/linuxdeepin/$repo" "$WORK/$repo"
    fi
}

build() {
    local repo=$1
    shift
    cmake -S "$WORK/$repo" -B "$WORK/build/$repo" -G Ninja \
        -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX="$PREFIX" \
        -DBUILD_TESTING=OFF -DBUILD_DOCS=OFF -DBUILD_EXAMPLES=OFF "$@"
    cmake --build "$WORK/build/$repo"
    cmake --install "$WORK/build/$repo"
    if [[ -n "${DTK_STAGE:-}" ]]; then
        DESTDIR="$DTK_STAGE" cmake --install "$WORK/build/$repo"
    fi
}

fetch treeland-protocols "$TREELAND_PROTOCOLS_TAG"
for repo in dtkcommon dtklog dtkcore dtkgui dtkwidget; do
    fetch "$repo" "$DTK_TAG"
done

# dtkcore 6.7.47 only knows the Qt 6.8.0 and >= 6.9 QAbstractFileEngine
# iterator API. Qt 6.8.1 already removed the QDir::Filters overload, so
# select the newer API from 6.8.1 on (no-op on Qt 6.8.0, deepin's version).
sed -i 's/QT_VERSION_CHECK(6, 9, 0)/QT_VERSION_CHECK(6, 8, 1)/' \
    "$WORK/dtkcore/src/dci/private/ddcifileengine_p.h" \
    "$WORK/dtkcore/src/dci/private/ddcifileengine.cpp"

build treeland-protocols
for repo in dtkcommon dtklog dtkcore dtkgui dtkwidget; do
    build "$repo" -DDTK5=OFF
done
printf 'installed DTK %s into %s\n' "$DTK_TAG" "$PREFIX"
