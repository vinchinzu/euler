"""Project Euler Problem 395 - Pythagorean Tree (Python 3.12)

This module provides an idiomatic Python implementation of the Pythagorean tree
construction used to solve Project Euler Problem 395.

Key features:
- Lightweight 2D Vector and Matrix types (no external dependencies).
- Affine Transform composition utilities.
- OrientedSquare representation with cached bounding box.
- PythagoreanTree generator and bounding-rectangle area computation with
  convergence checking.

The original Ruby version computed an approximate minimal bounding rectangle
area of about 28.2453753155.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import atan2, cos, sin, sqrt
from typing import Iterable, List, Tuple


@dataclass(frozen=True)
class Vector:
    """Immutable 2D vector with basic operations.

    This is intentionally minimal and tailored to this problem; it should not be
    considered a drop-in replacement for numpy or similar libraries.
    """

    x: float
    y: float

    def __getitem__(self, index: int) -> float:
        if index == 0:
            return self.x
        if index == 1:
            return self.y
        msg = "Vector index out of range"
        raise IndexError(msg)

    def __add__(self, other: "Vector") -> "Vector":
        return Vector(self.x + other.x, self.y + other.y)

    def __sub__(self, other: "Vector") -> "Vector":
        return Vector(self.x - other.x, self.y - other.y)

    def __mul__(self, scalar: float) -> "Vector":  # scalar multiplication
        return Vector(self.x * scalar, self.y * scalar)

    def __rmul__(self, scalar: float) -> "Vector":
        return self.__mul__(scalar)

    def __truediv__(self, scalar: float) -> "Vector":
        return Vector(self.x / scalar, self.y / scalar)

    def dot(self, other: "Vector") -> float:
        return self.x * other.x + self.y * other.y

    def magnitude(self) -> float:
        return sqrt(self.x * self.x + self.y * self.y)

    def normalize(self) -> "Vector":
        mag = self.magnitude()
        if mag == 0.0:
            msg = "Cannot normalize zero vector"
            raise ValueError(msg)
        return Vector(self.x / mag, self.y / mag)


@dataclass(frozen=True)
class Matrix:
    """Simple 2x2 matrix supporting multiplication and addition.

    Only the operations required by this problem are implemented.
    """

    a11: float
    a12: float
    a21: float
    a22: float

    @classmethod
    def from_rows(cls, rows: Iterable[Iterable[float]]) -> "Matrix":
        """Create a 2x2 matrix from row data.

        Raises ValueError if the shape is not exactly 2x2.
        """

        data = [list(r) for r in rows]
        if len(data) != 2 or any(len(r) != 2 for r in data):
            msg = "Matrix.from_rows requires a 2x2 iterable"
            raise ValueError(msg)
        return cls(data[0][0], data[0][1], data[1][0], data[1][1])

    def __mul__(self, other: object) -> object:
        if isinstance(other, Vector):
            x = self.a11 * other.x + self.a12 * other.y
            y = self.a21 * other.x + self.a22 * other.y
            return Vector(x, y)

        if isinstance(other, Matrix):
            a11 = self.a11 * other.a11 + self.a12 * other.a21
            a12 = self.a11 * other.a12 + self.a12 * other.a22
            a21 = self.a21 * other.a11 + self.a22 * other.a21
            a22 = self.a21 * other.a12 + self.a22 * other.a22
            return Matrix(a11, a12, a21, a22)

        msg = "Unsupported multiplication"
        raise TypeError(msg)

    def __add__(self, other: "Matrix") -> "Matrix":
        return Matrix(
            self.a11 + other.a11,
            self.a12 + other.a12,
            self.a21 + other.a21,
            self.a22 + other.a22,
        )


IDENTITY_MATRIX = Matrix(1.0, 0.0, 0.0, 1.0)
ZERO_VECTOR = Vector(0.0, 0.0)


@dataclass(frozen=True)
class Transform:
    """Affine transform represented as (matrix, vector).

    Applies: T(p) = matrix * p + vector.
    """

    matrix: Matrix
    vector: Vector

    def apply(self, point: Vector) -> Vector:
        return self.matrix * point + self.vector

    def compose(self, other: "Transform") -> "Transform":
        """Return composition self ∘ other (apply other, then self)."""

        # new(p) = M1 * (M2 * p + v2) + v1 = (M1*M2) * p + (M1 * v2 + v1)
        new_matrix = self.matrix * other.matrix
        new_vector = self.matrix * other.vector + self.vector
        return Transform(new_matrix, new_vector)


@dataclass
class OrientedSquare:
    """Square in the plane with arbitrary orientation.

    The transform acts on local coordinates centered at (0, 0). The square is
    defined by its center and side length in world coordinates.
    """

    center: Vector
    side_length: float
    transform: Transform
    corners: Tuple[Vector, Vector, Vector, Vector]
    min_x: float
    max_x: float
    min_y: float
    max_y: float

    def __init__(
        self,
        center: Tuple[float, float] | Vector,
        side_length: float,
        transform: Transform | None = None,
    ) -> None:
        if side_length <= 0.0:
            msg = "Side length must be positive"
            raise ValueError(msg)

        if not isinstance(center, Vector):
            cx, cy = center
            center_vec = Vector(float(cx), float(cy))
        else:
            center_vec = center

        # Default transform: identity at given center.
        if transform is None:
            transform = Transform(IDENTITY_MATRIX, center_vec)

        object.__setattr__(self, "center", center_vec)
        object.__setattr__(self, "side_length", float(side_length))
        object.__setattr__(self, "transform", transform)

        half_side = side_length / 2.0
        local_corners = (
            Vector(-half_side, -half_side),
            Vector(half_side, -half_side),
            Vector(half_side, half_side),
            Vector(-half_side, half_side),
        )

        corners = tuple(transform.apply(c) for c in local_corners)
        object.__setattr__(self, "corners", corners)  # type: ignore[arg-type]

        xs = [c.x for c in corners]
        ys = [c.y for c in corners]

        object.__setattr__(self, "min_x", min(xs))
        object.__setattr__(self, "max_x", max(xs))
        object.__setattr__(self, "min_y", min(ys))
        object.__setattr__(self, "max_y", max(ys))

    def side_endpoints(self, side_index: int) -> Tuple[Vector, Vector]:
        """Return endpoints of side by index: 0=bottom,1=right,2=top,3=left."""

        if side_index == 0:  # bottom
            return self.corners[0], self.corners[1]
        if side_index == 1:  # right
            return self.corners[1], self.corners[2]
        if side_index == 2:  # top
            return self.corners[2], self.corners[3]
        if side_index == 3:  # left
            return self.corners[3], self.corners[0]
        msg = f"Invalid side index: {side_index}"
        raise ValueError(msg)

    @staticmethod
    def create_leg_transform(
        p1: Vector,
        p2: Vector,
        leg_direction: str = "outward",
    ) -> Transform:
        """Create transform for a child square built on leg p1->p2.

        This matches the geometric intent of the Ruby version. The
        ``leg_direction`` parameter is accepted for compatibility; only
        "outward" is currently meaningful.
        """

        leg_vector = p2 - p1
        leg_length = leg_vector.magnitude()
        if leg_length == 0.0:
            msg = "Cannot create leg transform for zero-length segment"
            raise ValueError(msg)

        unit_leg = leg_vector.normalize()

        # Perpendicular vector (90° CCW); direction may be flipped later by
        # caller based on tree orientation logic.
        perp_vector = Vector(-unit_leg.y, unit_leg.x)

        if leg_direction != "outward":
            # For now we treat any non-outward hint as using the same
            # orientation. If more nuanced behavior is required, adjust here.
            pass

        midpoint = (p1 + p2) / 2.0
        perp_offset = perp_vector * (leg_length / 2.0)
        new_center = midpoint + perp_offset

        angle = atan2(unit_leg.y, unit_leg.x)
        cos_a = cos(angle)
        sin_a = sin(angle)
        rot_matrix = Matrix(cos_a, -sin_a, sin_a, cos_a)

        return Transform(rot_matrix, new_center)


class PythagoreanTree:
    """Generate and analyze the Pythagorean tree.

    Public methods:
    - generate(depth_limit): build squares up to depth_limit.
    - bounding_rectangle(): bounding box of current squares.
    - compute_converged_area(...): iterate until area converges.
    - run_tests(): basic self-checks.
    """

    TRIANGLE_RATIOS = {
        "short_leg": 3.0 / 5.0,
        "long_leg": 4.0 / 5.0,
        "height_factor": 12.0 / 25.0,
    }

    def __init__(self) -> None:
        self._squares: List[OrientedSquare] = []
        self._max_iterations: int = 25

    # ---- Tree generation -------------------------------------------------

    def generate(self, depth_limit: int | None = None) -> None:
        """Generate squares up to the given recursion depth (inclusive)."""

        if depth_limit is None:
            depth_limit = self._max_iterations

        initial_transform = Transform(IDENTITY_MATRIX, ZERO_VECTOR)
        root_square = OrientedSquare((0.0, 0.0), 1.0, initial_transform)

        self._squares.clear()
        # Start with top side as attachment (index 2), matching Ruby logic.
        self._generate_recursive(root_square, 0, depth_limit, 2)

    def _generate_recursive(
        self,
        square: OrientedSquare,
        depth: int,
        max_depth: int,
        attachment_side: int,
    ) -> None:
        if depth > max_depth:
            return

        self._squares.append(square)
        if depth == max_depth:
            return

        p1, p2 = square.side_endpoints(attachment_side)

        leg_vec = p2 - p1
        hypotenuse_length = leg_vec.magnitude()

        short_leg = hypotenuse_length * self.TRIANGLE_RATIOS["short_leg"]
        long_leg = hypotenuse_length * self.TRIANGLE_RATIOS["long_leg"]

        unit_hyp = leg_vec.normalize()
        projection_distance = hypotenuse_length * (9.0 / 25.0)  # (3/5)^2
        projection_point = p1 + unit_hyp * projection_distance

        # Perpendicular pointing initially CCW; may be flipped to face parent.
        perp_unit = Vector(-unit_hyp.y, unit_hyp.x)
        triangle_height = (
            hypotenuse_length * self.TRIANGLE_RATIOS["height_factor"]
        )

        # Decide correct perpendicular orientation so that triangle grows
        # outward from the current square.
        square_center = sum(square.corners, start=ZERO_VECTOR) / 4.0
        mid_segment = (p1 + p2) / 2.0
        vector_to_center = square_center - mid_segment
        if vector_to_center.dot(perp_unit) < 0.0:
            perp_unit = perp_unit * -1.0

        apex = projection_point + perp_unit * triangle_height

        eps = 1e-10
        if short_leg > eps:
            t1 = OrientedSquare.create_leg_transform(p1, apex, "outward")
            child1 = OrientedSquare((0.0, 0.0), short_leg, t1)
            new_side1 = (attachment_side + 2) % 4
            self._generate_recursive(child1, depth + 1, max_depth, new_side1)

        if long_leg > eps:
            t2 = OrientedSquare.create_leg_transform(apex, p2, "outward")
            child2 = OrientedSquare((0.0, 0.0), long_leg, t2)
            new_side2 = (attachment_side + 2) % 4
            self._generate_recursive(child2, depth + 1, max_depth, new_side2)

    # ---- Bounding rectangle ---------------------------------------------

    def bounding_rectangle(self) -> dict:
        """Return bounding rectangle for current squares.

        Dict keys: min_x, max_x, min_y, max_y, width, height, area.
        """

        if not self._squares:
            return {
                "min_x": 0.0,
                "max_x": 0.0,
                "min_y": 0.0,
                "max_y": 0.0,
                "width": 0.0,
                "height": 0.0,
                "area": 0.0,
            }

        min_x = min(s.min_x for s in self._squares)
        max_x = max(s.max_x for s in self._squares)
        min_y = min(s.min_y for s in self._squares)
        max_y = max(s.max_y for s in self._squares)

        width = max_x - min_x
        height = max_y - min_y
        area = width * height

        return {
            "min_x": min_x,
            "max_x": max_x,
            "min_y": min_y,
            "max_y": max_y,
            "width": width,
            "height": height,
            "area": area,
        }

    # ---- Convergence computation ----------------------------------------

    def compute_converged_area(
        self,
        max_iter: int = 25,
        precision: float = 1e-12,
    ) -> float:
        """Iteratively generate deeper trees until bounding area stabilizes."""

        prev_area = 0.0
        current_area = 0.0

        print(
            "Computing Pythagorean tree bounding area with convergence "
            "checking...",
        )
        print(f"Target precision: {precision}")
        print("-" * 60)

        for iteration in range(1, max_iter + 1):
            self.generate(iteration)
            bounds = self.bounding_rectangle()
            current_area = bounds["area"]

            print(
                f"Iteration {iteration}, "
                f"Height={bounds['height']:.6f}, "
                f"Area={current_area:.12f}",
            )

            if current_area > 0.0:
                delta = abs(current_area - prev_area)
                relative_delta = delta / current_area
            else:
                delta = 0.0
                relative_delta = 0.0

            if iteration > 3 and (
                delta < precision or relative_delta < precision
            ):
                print(f"\nConverged at iteration {iteration}")
                print(
                    "Final delta: "
                    f"{delta:.15f}, Relative delta: {relative_delta:.15f}",
                )
                break

            prev_area = current_area
        else:
            # Loop completed without break: we still return current_area.
            pass

        print("-" * 60)
        print(
            "Final result (rounded to 10 decimal places): "
            f"{current_area:.10f}",
        )
        return current_area

    # ---- Lightweight self-tests ----------------------------------------

    @staticmethod
    def _assert_in_delta(
        expected: float,
        actual: float,
        delta: float,
        message: str,
    ) -> None:
        if abs(expected - actual) <= delta:
            print(f"\u2713 {message}")
        else:
            print(f"\u2717 {message}, got {actual}")

    @staticmethod
    def _assert_equal(expected: object, actual: object, message: str) -> None:
        if expected == actual:
            print(f"\u2713 {message}")
        else:
            print(f"\u2717 {message}, got {actual}")

    @classmethod
    def run_tests(cls) -> None:
        """Run basic unit checks mirroring the Ruby version."""

        print("Running unit tests...")
        print("-" * 40)

        tree = cls()
        tree.generate(0)
        bounds = tree.bounding_rectangle()

        cls._assert_in_delta(1.0, bounds["width"], 1e-10, "Root square width")
        cls._assert_in_delta(1.0, bounds["height"], 1e-10, "Root square height")
        cls._assert_in_delta(1.0, bounds["area"], 1e-10, "Root square area")
        print("\u2713 Root square test passed")

        tree.generate(1)
        cls._assert_equal(3, len(tree._squares), "Depth 1 square count")

        hypotenuse = 5.0
        short_leg = hypotenuse * cls.TRIANGLE_RATIOS["short_leg"]
        long_leg = hypotenuse * cls.TRIANGLE_RATIOS["long_leg"]
        cls._assert_in_delta(3.0, short_leg, 1e-10, "Short leg ratio")
        cls._assert_in_delta(4.0, long_leg, 1e-10, "Long leg ratio")
        print("\u2713 Triangle ratio test passed")

        try:
            OrientedSquare((0.0, 0.0), 0.0)
            print(
                "\u2717 Zero side length test failed (should raise error)",
            )
        except ValueError:
            print("\u2713 Zero side length test passed")

        print("All unit tests passed!")
        print()


def _main() -> None:
    """Entry point for command-line execution."""

    PythagoreanTree.run_tests()

    tree = PythagoreanTree()
    final_area = tree.compute_converged_area(20, 1e-12)

    print("\n" + "=" * 60)
    print("PROJECT EULER PROBLEM 395 SOLUTION")
    print("=" * 60)
    print(
        "The smallest area of the bounding rectangle for the Pythagorean "
        f"tree is:\n{final_area:.10f}",
    )
    print("=" * 60)

    # Print only final answer for test harness
    print()
    print(f"{final_area:.10f}")


if __name__ == "__main__":
    _main()
