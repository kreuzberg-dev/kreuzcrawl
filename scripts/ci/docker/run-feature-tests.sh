#!/usr/bin/env bash
set -euo pipefail

variant="${1:?missing variant}"

echo "=== Running Docker feature tests (${variant}) ==="
python3 scripts/ci/docker/test_docker.py --image "crawlberg:${variant}" --variant "${variant}" --verbose
