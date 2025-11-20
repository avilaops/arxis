"""
AABB (Axis-Aligned Bounding Box) class for collision detection
"""

from typing import List, Optional
from .vector import Vector3
import math


class AABB:
    """Axis-Aligned Bounding Box"""

    def __init__(self, min_point: Vector3 = None, max_point: Vector3 = None):
        """
        Create an AABB from min and max points
        If no points provided, creates an invalid AABB
        """
        if min_point is None:
            self.min = Vector3(math.inf, math.inf, math.inf)
            self.max = Vector3(-math.inf, -math.inf, -math.inf)
        else:
            self.min = min_point
            self.max = max_point if max_point is not None else min_point

    @staticmethod
    def from_center_size(center: Vector3, size: Vector3) -> "AABB":
        """Create AABB from center point and size"""
        half_size = size * 0.5
        return AABB(center - half_size, center + half_size)

    @staticmethod
    def from_points(points: List[Vector3]) -> "AABB":
        """Create AABB that encompasses all given points"""
        if not points:
            return AABB()

        min_x = min(p.x for p in points)
        min_y = min(p.y for p in points)
        min_z = min(p.z for p in points)

        max_x = max(p.x for p in points)
        max_y = max(p.y for p in points)
        max_z = max(p.z for p in points)

        return AABB(Vector3(min_x, min_y, min_z), Vector3(max_x, max_y, max_z))

    def center(self) -> Vector3:
        """Get center point of the AABB"""
        return (self.min + self.max) * 0.5

    def size(self) -> Vector3:
        """Get size (dimensions) of the AABB"""
        return self.max - self.min

    def half_size(self) -> Vector3:
        """Get half size (extents) of the AABB"""
        return self.size() * 0.5

    def volume(self) -> float:
        """Calculate volume of the AABB"""
        size = self.size()
        return size.x * size.y * size.z

    def surface_area(self) -> float:
        """Calculate surface area of the AABB"""
        size = self.size()
        return 2.0 * (size.x * size.y + size.y * size.z + size.z * size.x)

    def contains_point(self, point: Vector3) -> bool:
        """Check if point is inside the AABB"""
        return (
            self.min.x <= point.x <= self.max.x
            and self.min.y <= point.y <= self.max.y
            and self.min.z <= point.z <= self.max.z
        )

    def contains_aabb(self, other: "AABB") -> bool:
        """Check if another AABB is completely inside this one"""
        return (
            self.min.x <= other.min.x
            and self.max.x >= other.max.x
            and self.min.y <= other.min.y
            and self.max.y >= other.max.y
            and self.min.z <= other.min.z
            and self.max.z >= other.max.z
        )

    def intersects(self, other: "AABB") -> bool:
        """Check if this AABB intersects with another"""
        return (
            self.min.x <= other.max.x
            and self.max.x >= other.min.x
            and self.min.y <= other.max.y
            and self.max.y >= other.min.y
            and self.min.z <= other.max.z
            and self.max.z >= other.min.z
        )

    def intersection(self, other: "AABB") -> Optional["AABB"]:
        """Get intersection AABB with another, or None if no intersection"""
        if not self.intersects(other):
            return None

        min_point = Vector3(
            max(self.min.x, other.min.x),
            max(self.min.y, other.min.y),
            max(self.min.z, other.min.z),
        )

        max_point = Vector3(
            min(self.max.x, other.max.x),
            min(self.max.y, other.max.y),
            min(self.max.z, other.max.z),
        )

        return AABB(min_point, max_point)

    def merge(self, other: "AABB") -> "AABB":
        """Create AABB that contains both this and another AABB"""
        min_point = Vector3(
            min(self.min.x, other.min.x),
            min(self.min.y, other.min.y),
            min(self.min.z, other.min.z),
        )

        max_point = Vector3(
            max(self.max.x, other.max.x),
            max(self.max.y, other.max.y),
            max(self.max.z, other.max.z),
        )

        return AABB(min_point, max_point)

    def expand(self, amount: float) -> "AABB":
        """Expand AABB by amount in all directions"""
        expansion = Vector3(amount, amount, amount)
        return AABB(self.min - expansion, self.max + expansion)

    def expand_by_vector(self, vector: Vector3) -> "AABB":
        """Expand AABB to include a point"""
        min_point = Vector3(
            min(self.min.x, vector.x),
            min(self.min.y, vector.y),
            min(self.min.z, vector.z),
        )

        max_point = Vector3(
            max(self.max.x, vector.x),
            max(self.max.y, vector.y),
            max(self.max.z, vector.z),
        )

        return AABB(min_point, max_point)

    def closest_point(self, point: Vector3) -> Vector3:
        """Find closest point on AABB to given point"""
        return Vector3(
            max(self.min.x, min(point.x, self.max.x)),
            max(self.min.y, min(point.y, self.max.y)),
            max(self.min.z, min(point.z, self.max.z)),
        )

    def distance_to_point(self, point: Vector3) -> float:
        """Calculate distance from AABB to a point"""
        closest = self.closest_point(point)
        return closest.distance_to(point)

    def is_valid(self) -> bool:
        """Check if AABB is valid (min <= max)"""
        return (
            self.min.x <= self.max.x
            and self.min.y <= self.max.y
            and self.min.z <= self.max.z
        )

    def corners(self) -> List[Vector3]:
        """Get all 8 corner points of the AABB"""
        return [
            Vector3(self.min.x, self.min.y, self.min.z),
            Vector3(self.max.x, self.min.y, self.min.z),
            Vector3(self.min.x, self.max.y, self.min.z),
            Vector3(self.max.x, self.max.y, self.min.z),
            Vector3(self.min.x, self.min.y, self.max.z),
            Vector3(self.max.x, self.min.y, self.max.z),
            Vector3(self.min.x, self.max.y, self.max.z),
            Vector3(self.max.x, self.max.y, self.max.z),
        ]

    def __repr__(self) -> str:
        return f"AABB(min={self.min}, max={self.max})"

    def __str__(self) -> str:
        return f"AABB[min={self.min}, max={self.max}]"
