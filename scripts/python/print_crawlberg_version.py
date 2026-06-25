#!/usr/bin/env python3
"""Print the installed crawlberg Python package version."""

from __future__ import annotations

import sys


def main() -> int:
    try:
        import crawlberg  # type: ignore[import-untyped]
    except ImportError as exc:  # pragma: no cover - runtime helper
        print(f"Failed to import crawlberg: {exc}", file=sys.stderr)
        return 1

    print(f"Crawlberg version: {getattr(crawlberg, '__version__', 'unknown')}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
