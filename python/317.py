"""Project Euler Problem 317 (improved)

This module computes the volume in m^3 of the region swept out by fragments
from a firecracker exploding at height H0 with initial speed V0, neglecting
air resistance and assuming a uniform gravitational field.

Public API:
- solve() -> float: compute the volume rounded to 4 decimal places.

Run this file directly to execute basic tests and print the result.
"""
from __future__ import annotations

from dataclasses import dataclass
from math import pi, sqrt
from typing import Callable


G: float = 9.81
H0: float = 100.0
V0: float = 20.0


@dataclass(frozen=True)
class EnvelopeCalculator:
    """Compute properties of the trajectory envelope.

    All methods treat G, H0, and V0 as global physical parameters.
    """

    @staticmethod
    def z_max(rho: float) -> float:
        """Return maximal height z at radial distance rho.

        If rho is not reachable or parameters are invalid, 0.0 is returned.
        """

        if rho < 0 or H0 <= 0.0 or V0 <= 0.0:
            return 0.0

        # Envelope equation for projectile motion from height H0 with speed V0
        # Derived from eliminating launch angle from trajectory equations:
        # z = H0 + V0^2/(2g) - g*rho^2/(2*V0^2)
        z_top: float = H0 + (V0**2) / (2.0 * G)
        z: float = z_top - (G * rho**2) / (2.0 * V0**2)

        # Return 0 if the point is beyond reachable range
        return max(0.0, z)

    @staticmethod
    def is_reachable(rho: float, z: float) -> bool:
        """Return True if point (rho, z) lies within the reachable envelope.

        This is based on z_max(rho) with a small tolerance for numerical issues.
        """

        if z < 0.0:
            return False

        z_top: float = H0 + (V0**2) / (2.0 * G)
        if z > z_top:
            return False

        if V0 <= 0.0:
            return False

        if rho < 0.0:
            return False

        # Check if rho is within maximum range (where envelope reaches z=0)
        # From z = z_top - G*rho^2/(2*V0^2) = 0, solve for rho_max
        rho_max: float = sqrt(2.0 * V0**2 * z_top / G)
        if rho > rho_max:
            return False

        z_max_val: float = EnvelopeCalculator.z_max(rho)
        return z <= z_max_val + 1e-6


@dataclass(frozen=True)
class VolumeIntegrator:
    """Numerical utilities to integrate the envelope and compute the volume."""

    @staticmethod
    def integrate(func: Callable[[float], float],
                  a: float,
                  b: float,
                  n: int = 1000) -> float:
        """Approximate integral of func from a to b via Simpson's rule.

        Returns 0.0 if the interval or n is invalid.
        """

        if a >= b or n <= 0:
            return 0.0

        # Ensure n is even as required by Simpson's rule.
        if n % 2 == 1:
            n += 1

        h: float = (b - a) / float(n)
        integral: float = func(a) + func(b)

        for i in range(1, n):
            x: float = a + i * h
            weight: float = 4.0 if i % 2 == 1 else 2.0
            integral += weight * func(x)

        return integral * h / 3.0

    @staticmethod
    def estimate_rho_max() -> float:
        """Estimate maximal radial distance reached by any fragment."""

        return (V0**2) / G + V0 * sqrt(2.0 * H0 / G)

    @staticmethod
    def compute_volume(n_rho: int = 10_000,
                       tolerance: float = 1e-6) -> float:
        """Compute swept volume via radial integration of the envelope.

        Uses Simpson's rule and doubles n_rho until convergence within
        the given tolerance.
        """

        if V0 <= 0.0 or H0 <= 0.0:
            return 0.0

        rho_max: float = VolumeIntegrator.estimate_rho_max()

        def z_max_func(rho: float) -> float:
            return 2.0 * pi * rho * EnvelopeCalculator.z_max(rho)

        volume: float = VolumeIntegrator.integrate(z_max_func, 0.0, rho_max,
                                                   n_rho)

        while True:
            n_rho *= 2
            new_volume: float = VolumeIntegrator.integrate(
                z_max_func, 0.0, rho_max, n_rho
            )
            if abs(new_volume - volume) < tolerance:
                return new_volume
            volume = new_volume


def solve() -> float:
    """Return the volume (m^3) rounded to 4 decimal places.

    Uses global constants G, H0, and V0.
    """

    if V0 <= 0.0 or H0 <= 0.0:
        return 0.0

    volume: float = VolumeIntegrator.compute_volume()
    return round(volume, 4)


def _run_tests() -> None:
    """Run a minimal self-test suite.

    This replaces the original Ruby Minitest tests with straightforward
    assertions to keep the module self-contained and dependency-free.
    """

    # z_max at rho=0 should give the apex height
    z_top: float = H0 + (V0**2) / (2.0 * G)
    assert abs(EnvelopeCalculator.z_max(0.0) - z_top) < 1e-6

    # Far away rho should be essentially unreachable
    assert abs(EnvelopeCalculator.z_max(1_000.0) - 0.0) < 1e-6

    z_at_10: float = EnvelopeCalculator.z_max(10.0)
    assert 0.0 < z_at_10 < z_top

    # Reachability checks
    assert EnvelopeCalculator.is_reachable(0.0, H0)
    assert EnvelopeCalculator.is_reachable(30.0, 0.0)
    assert not EnvelopeCalculator.is_reachable(0.0, z_top + 1.0)
    assert not EnvelopeCalculator.is_reachable(1_000.0, 0.0)

    # Volume should be positive and within a reasonable upper bound
    vol: float = VolumeIntegrator.compute_volume(1_000)
    assert 0.0 < vol < 5_000_000.0

    # Solver should match known Project Euler solution closely
    result: float = solve()
    assert 1_856_500.0 < result < 1_856_600.0


def main() -> None:
    """Execute tests then print the computed volume."""

    _run_tests()
    volume: float = solve()
    print(f"{volume:.4f}")


if __name__ == "__main__":  # pragma: no cover - CLI entry point
    main()
