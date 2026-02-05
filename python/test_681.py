#!/usr/bin/env python3
"""
Tests for Project Euler Problem 681 - Maximal Area
"""

import pytest
import subprocess
import os


def test_681_fast_helper_exists():
    """Test that the C helper source exists."""
    helper_c = os.path.join(os.path.dirname(__file__), '681_fast.c')
    assert os.path.exists(helper_c), f"C helper source not found: {helper_c}"


def test_681_produces_correct_answer():
    """Test that the solution produces the correct answer."""
    script_path = os.path.join(os.path.dirname(__file__), '681.py')
    result = subprocess.run(
        ['python', script_path],
        capture_output=True,
        text=True,
        timeout=60
    )

    assert result.returncode == 0, f"Script failed: {result.stderr}"

    answer = int(result.stdout.strip())
    expected = 2611227421428

    assert answer == expected, f"Expected {expected}, got {answer}"


def test_681_fast_helper_correctness():
    """Test that the C helper produces correct results for small test cases."""
    helper_path = os.path.join(os.path.dirname(__file__), '681_fast')

    # Compile if needed
    if not os.path.exists(helper_path):
        c_source = os.path.join(os.path.dirname(__file__), '681_fast.c')
        result = subprocess.run(
            ['gcc', '-O3', '-march=native', '-o', helper_path, c_source, '-lm'],
            capture_output=True,
            text=True
        )
        assert result.returncode == 0, f"Compilation failed: {result.stderr}"

    # Run the helper
    result = subprocess.run([helper_path], capture_output=True, text=True, timeout=60)
    assert result.returncode == 0, f"Execution failed: {result.stderr}"

    # Check that test cases pass (output to stderr)
    assert "SP(10) = 186 (expected 186)" in result.stderr
    assert "SP(100) = 23238 (expected 23238)" in result.stderr

    # Check final answer
    answer = int(result.stdout.strip())
    assert answer == 2611227421428


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
