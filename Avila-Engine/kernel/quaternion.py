"""
Quaternion class for 3D rotations
"""

import glm
import numpy as np
import math
from typing import Tuple
from .vector import Vector3
from .matrix import Matrix4x4


class Quaternion:
    """Quaternion wrapper around PyGLM quat"""

    def __init__(self, w: float = 1.0, x: float = 0.0, y: float = 0.0, z: float = 0.0):
        """Create quaternion (w, x, y, z) - w is the scalar part"""
        self._quat = glm.quat(w, x, y, z)

    @property
    def w(self) -> float:
        return self._quat.w

    @w.setter
    def w(self, value: float):
        self._quat.w = value

    @property
    def x(self) -> float:
        return self._quat.x

    @x.setter
    def x(self, value: float):
        self._quat.x = value

    @property
    def y(self) -> float:
        return self._quat.y

    @y.setter
    def y(self, value: float):
        self._quat.y = value

    @property
    def z(self) -> float:
        return self._quat.z

    @z.setter
    def z(self, value: float):
        self._quat.z = value

    def __mul__(self, other: "Quaternion") -> "Quaternion":
        """Quaternion multiplication"""
        result = Quaternion()
        result._quat = self._quat * other._quat
        return result

    @staticmethod
    def identity() -> "Quaternion":
        """Identity quaternion (no rotation)"""
        return Quaternion(1, 0, 0, 0)

    @staticmethod
    def from_axis_angle(axis: Vector3, angle_radians: float) -> "Quaternion":
        """Create quaternion from axis and angle"""
        result = Quaternion()
        result._quat = glm.angleAxis(angle_radians, axis._vec)
        return result

    @staticmethod
    def from_euler(pitch: float, yaw: float, roll: float) -> "Quaternion":
        """Create quaternion from Euler angles (radians)"""
        result = Quaternion()
        result._quat = glm.quat(glm.vec3(pitch, yaw, roll))
        return result

    @staticmethod
    def from_euler_degrees(pitch: float, yaw: float, roll: float) -> "Quaternion":
        """Create quaternion from Euler angles (degrees)"""
        return Quaternion.from_euler(
            math.radians(pitch), math.radians(yaw), math.radians(roll)
        )

    @staticmethod
    def look_rotation(forward: Vector3, up: Vector3 = None) -> "Quaternion":
        """Create quaternion that looks in a direction"""
        if up is None:
            up = Vector3.up()

        result = Quaternion()
        result._quat = glm.quatLookAt(glm.normalize(forward._vec), up._vec)
        return result

    def to_euler(self) -> Tuple[float, float, float]:
        """Convert to Euler angles in radians (pitch, yaw, roll)"""
        euler = glm.eulerAngles(self._quat)
        return (euler.x, euler.y, euler.z)

    def to_euler_degrees(self) -> Tuple[float, float, float]:
        """Convert to Euler angles in degrees (pitch, yaw, roll)"""
        pitch, yaw, roll = self.to_euler()
        return (math.degrees(pitch), math.degrees(yaw), math.degrees(roll))

    def to_matrix(self) -> "Matrix4x4":
        """Convert to 4x4 rotation matrix"""
        result = Matrix4x4()
        result._mat = glm.mat4_cast(self._quat)
        return result

    def to_axis_angle(self) -> Tuple[Vector3, float]:
        """Convert to axis-angle representation"""
        angle = glm.angle(self._quat)
        axis_glm = glm.axis(self._quat)
        axis = Vector3()
        axis._vec = axis_glm
        return (axis, angle)

    def normalize(self) -> "Quaternion":
        """Returns normalized quaternion"""
        result = Quaternion()
        result._quat = glm.normalize(self._quat)
        return result

    def conjugate(self) -> "Quaternion":
        """Returns conjugate quaternion"""
        result = Quaternion()
        result._quat = glm.conjugate(self._quat)
        return result

    def inverse(self) -> "Quaternion":
        """Returns inverse quaternion"""
        result = Quaternion()
        result._quat = glm.inverse(self._quat)
        return result

    def dot(self, other: "Quaternion") -> float:
        """Dot product"""
        return glm.dot(self._quat, other._quat)

    def length(self) -> float:
        """Quaternion magnitude"""
        return glm.length(self._quat)

    def rotate_vector(self, vector: Vector3) -> Vector3:
        """Rotate a vector by this quaternion"""
        result = Vector3()
        result._vec = self._quat * vector._vec
        return result

    def slerp(self, other: "Quaternion", t: float) -> "Quaternion":
        """Spherical linear interpolation"""
        result = Quaternion()
        result._quat = glm.slerp(self._quat, other._quat, t)
        return result

    @staticmethod
    def lerp(a: "Quaternion", b: "Quaternion", t: float) -> "Quaternion":
        """Linear interpolation (not spherical)"""
        result = Quaternion()
        result._quat = glm.mix(a._quat, b._quat, t)
        return result.normalize()

    def to_tuple(self) -> Tuple[float, float, float, float]:
        """Convert to tuple (w, x, y, z)"""
        return (self.w, self.x, self.y, self.z)

    def to_array(self) -> np.ndarray:
        """Convert to numpy array [w, x, y, z]"""
        return np.array([self.w, self.x, self.y, self.z], dtype=np.float32)

    def __repr__(self) -> str:
        return f"Quaternion(w={self.w}, x={self.x}, y={self.y}, z={self.z})"

    def __str__(self) -> str:
        return f"Quat(w={self.w:.3f}, x={self.x:.3f}, y={self.y:.3f}, z={self.z:.3f})"
