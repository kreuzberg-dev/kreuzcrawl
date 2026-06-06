"""Smoke test: import the published package."""

import importlib


def test_imports_published_package():
    module = importlib.import_module("kreuzcrawl")
    assert module is not None
