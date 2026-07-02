---
priority: high
usage: "/fix"
description: "Auto-fix linting, formatting, and common issues"
---

# Fix

Automatically fix as many issues as possible:

1. Run `task format` if available, otherwise run language-specific formatters; this excludes Alef formatting
2. Run `poly fmt --fix .` and `poly lint --fix .` to catch and fix remaining issues
3. Run `task alef:format` only when Alef-generated output needs formatting
4. Report what was fixed and what still needs manual attention
