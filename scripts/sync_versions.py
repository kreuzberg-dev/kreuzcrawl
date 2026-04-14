#!/usr/bin/env python3
"""
Sync version from Cargo.toml workspace to all package manifests.

This script delegates to `alef sync-versions` which handles:
- All package manifests (pyproject.toml, package.json, gemspec, pom.xml, etc.)
- PEP 440 conversion for Python (e.g., 0.1.0-rc.1 → 0.1.0rc1)
- Vendored C header refresh

For manual version changes, use: alef sync-versions --set <version>
"""

import subprocess
import sys


def main() -> int:
    try:
        subprocess.run(["alef", "sync-versions"], check=True)
    except FileNotFoundError:
        print(
            "Error: alef not found. Install with: cargo install --git https://github.com/kreuzberg-dev/alef",
            file=sys.stderr,
        )
        return 1
    except subprocess.CalledProcessError as e:
        return e.returncode
    return 0


if __name__ == "__main__":
    sys.exit(main())
