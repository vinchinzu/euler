#!/usr/bin/env python3
"""Tests for Project Euler Problem 750: Optimal Card Stacking."""

import pytest
import sys
import os

# Add parent directory to path to import the solution
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import importlib.util

# Get the full path to 750.py
script_dir = os.path.dirname(os.path.abspath(__file__))
solution_path = os.path.join(script_dir, "750.py")

spec = importlib.util.spec_from_file_location("p750", solution_path)
p750 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(p750)


class TestProblem750:
    """Test Problem 750 solution."""

    def test_g_6(self):
        """Test G(6) = 8."""
        assert p750.solve(6) == 8

    def test_g_16(self):
        """Test G(16) = 47."""
        assert p750.solve(16) == 47

    def test_g_976(self):
        """Test G(976) - the actual problem answer."""
        result = p750.solve(976)
        assert result == 160640
        assert isinstance(result, int)

    def test_solution_runs_quickly(self):
        """Test that the solution completes in reasonable time."""
        import time
        start = time.time()
        result = p750.solve(976)
        elapsed = time.time() - start
        assert elapsed < 5.0  # Should complete in under 5 seconds
        assert result == 160640


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
