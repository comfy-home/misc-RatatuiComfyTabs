#!/usr/bin/env bash
# Copyright © 2026 ComfyHome™
# All rights reserved.
#
# Licensed under the ComfyGit SA-PS License
#
# For details, see the LICENSE file in the repository root.
#
############################################################
#
# General-purpose release script for source-only Rust crates (no compiled artifacts).
## with automated crates.io publishing.
## and fully compatible with ComfyGitFlow.
#
# ComfyGit repo: https://gitlab.com/comfyhome/dist/ComfyGit
#
# Flow:
#   1. Quality checks  : cargo fmt --check, cargo clippy, cargo test
#   2. Git push        : pushes commits + tags to the remote
#   3. Cargo publish   : publishes the crate to crates.io
#
# Usage:
#   ./scripts/releaseNOW.sh                  # Full release
#   ./scripts/releaseNOW.sh --no-checks      # Skip fmt/clippy/test
#   ./scripts/releaseNOW.sh --skip-test      # Skip tests only
#   ./scripts/releaseNOW.sh --test-only      # Run checks only, do not release
#   ./scripts/releaseNOW.sh --dry-run        # Everything except git push & cargo publish
#   ./scripts/releaseNOW.sh --skip-publish   # Push tag but skip cargo publish

set -euo pipefail

############################################################
#            DEFINE VARIABLES IN THE FOLLOWING SECTION     #
############################################################

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

# Crate / package identity
CRATE_NAME='ratatui-comfy-tabs'

# Git remote to push to
GIT_REMOTE='gitlab'

# Branch that holds release commits (used for the push)
GIT_BRANCH='main'

# Cargo publish flags (e.g. '--allow-dirty' during dev; empty for clean releases)
CARGO_PUBLISH_FLAGS=''

############################################################
#                END OF VARIABLE DEFINITIONS               #
############################################################

# ── Flags ──────────────────────────────────────────────────────────────────
NO_CHECKS=false
SKIP_TEST=false
TEST_ONLY=false
DRY_RUN=false
SKIP_PUBLISH=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --no-checks)    NO_CHECKS=true;     shift ;;
        --skip-test)    SKIP_TEST=true;     shift ;;
        --test-only)    TEST_ONLY=true;     shift ;;
        --dry-run)      DRY_RUN=true;       shift ;;
        --skip-publish) SKIP_PUBLISH=true;  shift ;;
        -h|--help)
            echo "Usage: $0 [options]"
            echo "  --no-checks      Skip fmt / clippy / test"
            echo "  --skip-test      Skip cargo test only"
            echo "  --test-only      Run checks only; do not tag, push, or publish"
            echo "  --dry-run        Run everything except git push and cargo publish"
            echo "  --skip-publish   Push tag but skip cargo publish"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# ── Colours ─────────────────────────────────────────────────────────────────
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_info()    { echo -e "${CYAN}$1${NC}"; }
log_success() { echo -e "${GREEN}$1${NC}"; }
log_warning() { echo -e "${YELLOW}$1${NC}"; }
log_error()   { echo -e "${RED}$1${NC}"; }

# ── Helpers ─────────────────────────────────────────────────────────────────
get_project_version() {
    local manifest="${PROJECT_ROOT}/Cargo.toml"
    if [[ ! -f "$manifest" ]]; then
        log_error "Cargo.toml not found at $manifest"
        exit 1
    fi
    local version
    version=$(grep -E '^version\s*=\s*"[^"]+"' "$manifest" | head -1 | sed -E 's/.*"([^"]+)".*/\1/')
    if [[ -z "$version" ]]; then
        log_error "Unable to parse version from $manifest"
        exit 1
    fi
    echo "$version"
}

require_command() {
    local cmd="$1"
    if ! command -v "$cmd" &>/dev/null; then
        log_error "Required command not found: $cmd"
        exit 1
    fi
}

# ── Step 1: Quality checks ───────────────────────────────────────────────────
run_checks() {
    if [[ "$NO_CHECKS" == true ]]; then
        log_warning "Skipping quality checks (--no-checks)."
        return
    fi

    log_info "── Checking formatting..."
    cargo fmt --manifest-path "${PROJECT_ROOT}/Cargo.toml" -- --check

    log_info "── Running clippy..."
    cargo clippy --manifest-path "${PROJECT_ROOT}/Cargo.toml" -- -D warnings

    if [[ "$SKIP_TEST" == true ]]; then
        log_warning "Skipping tests (--skip-test)."
    else
        log_info "── Running tests..."
        cargo test --manifest-path "${PROJECT_ROOT}/Cargo.toml"
    fi

    log_success "Quality checks passed."
}

# ── Step 2: Git push ─────────────────────────────────────────────────────────
run_push() {
    local version="$1"
    local tag="v${version}"
    if ! git -C "${PROJECT_ROOT}" rev-parse "${tag}" &>/dev/null; then
        log_info "── Tagging HEAD as ${tag}..."
        git -C "${PROJECT_ROOT}" tag "${tag}"
    fi
    log_info "── Pushing branch '${GIT_BRANCH}' and tag '${tag}' to ${GIT_REMOTE}..."
    git -C "${PROJECT_ROOT}" push "${GIT_REMOTE}" "${GIT_BRANCH}"
    git -C "${PROJECT_ROOT}" push "${GIT_REMOTE}" "${tag}"
    log_success "Pushed to ${GIT_REMOTE}."
}

# ── Step 3: Cargo publish ────────────────────────────────────────────────────
run_publish() {
    log_info "── Publishing ${CRATE_NAME} to crates.io..."
    # shellcheck disable=SC2086
    cargo publish --manifest-path "${PROJECT_ROOT}/Cargo.toml" $CARGO_PUBLISH_FLAGS
    log_success "${CRATE_NAME} published to crates.io."
}

# ── Main ─────────────────────────────────────────────────────────────────────
main() {
    log_info "=== ReleaseNOW: ${CRATE_NAME} ==="

    run_checks

    if [[ "$TEST_ONLY" == true ]]; then
        log_success "Test-only run complete. Nothing was released."
        exit 0
    fi

    local version
    version=$(get_project_version)
    log_info "Releasing version ${version}..."

    if [[ "$DRY_RUN" == true ]]; then
        log_warning "Dry-run: skipping git push and cargo publish."
        log_success "Dry-run complete for ${CRATE_NAME} v${version}."
        exit 0
    fi

    run_push "$version"

    if [[ "$SKIP_PUBLISH" == true ]]; then
        log_warning "Skipping cargo publish (--skip-publish)."
    else
        run_publish
    fi

    log_success "=== Released ${CRATE_NAME} v${version} ==="
}

main
