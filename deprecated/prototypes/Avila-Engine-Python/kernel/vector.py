"""
Vector classes for 3D mathematics
"""

import glm
import numpy as np
from typing import Union, Tuple


class Vector3:
    """3D Vector wrapper around PyGLM vec3"""

    def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
        self._vec = glm.vec3(x, y, z)

    @property
    def x(self) -> float:
        return self._vec.x

    @x.setter
    def x(self, value: float):
        self._vec.x = value

    @property
    def y(self) -> float:
        return self._vec.y

    @y.setter
    def y(self, value: float):
        self._vec.y = value

    @property
    def z(self) -> float:
        return self._vec.z

    @z.setter
    def z(self, value: float):
        self._vec.z = value

    def __add__(self, other: "Vector3") -> "Vector3":
        result = Vector3()
        result._vec = self._vec + other._vec
        return result

    def __sub__(self, other: "Vector3") -> "Vector3":
        result = Vector3()
        result._vec = self._vec - other._vec
        return result

    def __mul__(self, scalar: float) -> "Vector3":
        result = Vector3()
        result._vec = self._vec * scalar
        return result

    def __rmul__(self, scalar: float) -> "Vector3":
        return self.__mul__(scalar)

    def __truediv__(self, scalar: float) -> "Vector3":
        result = Vector3()
        result._vec = self._vec / scalar
        return result

    def dot(self, other: "Vector3") -> float:
        """Dot product"""
        return glm.dot(self._vec, other._vec)

    def cross(self, other: "Vector3") -> "Vector3":
        """Cross product"""
        result = Vector3()
        result._vec = glm.cross(self._vec, other._vec)
        return result

    def length(self) -> float:
        """Vector magnitude"""
        return glm.length(self._vec)

    def length_squared(self) -> float:
        """Squared magnitude (faster than length)"""
        return glm.dot(self._vec, self._vec)

    def normalize(self) -> "Vector3":
        """Returns normalized copy"""
        result = Vector3()
        result._vec = glm.normalize(self._vec)
        return result

    def normalized(self) -> "Vector3":
        """Alias for normalize"""
        return self.normalize()

    def distance_to(self, other: "Vector3") -> float:
        """Distance to another vector"""
        return glm.distance(self._vec, other._vec)

    def lerp(self, other: "Vector3", t: float) -> "Vector3":
        """Linear interpolation"""
        result = Vector3()
        result._vec = glm.mix(self._vec, other._vec, t)
        return result

    def to_tuple(self) -> Tuple[float, float, float]:
        """Convert to tuple"""
        return (self.x, self.y, self.z)

    def to_array(self) -> np.ndarray:
        """Convert to numpy array"""
        return np.array([self.x, self.y, self.z], dtype=np.float32)

    @staticmethod
    def zero() -> "Vector3":
        """Zero vector (0, 0, 0)"""
        return Vector3(0, 0, 0)

    @staticmethod
    def one() -> "Vector3":
        """One vector (1, 1, 1)"""
        return Vector3(1, 1, 1)

    @staticmethod
    def up() -> "Vector3":
        """Up vector (0, 1, 0)"""
        return Vector3(0, 1, 0)

    @staticmethod
    def down() -> "Vector3":
        """Down vector (0, -1, 0)"""
        return Vector3(0, -1, 0)

    @staticmethod
    def left() -> "Vector3":
        """Left vector (-1, 0, 0)"""
        return Vector3(-1, 0, 0)

    @staticmethod
    def right() -> "Vector3":
        """Right vector (1, 0, 0)"""
        return Vector3(1, 0, 0)

    @staticmethod
    def forward() -> "Vector3":
        """Forward vector (0, 0, -1)"""
        return Vector3(0, 0, -1)

    @staticmethod
    def back() -> "Vector3":
        """Back vector (0, 0, 1)"""
        return Vector3(0, 0, 1)

    def __repr__(self) -> str:
        return f"Vector3({self.x}, {self.y}, {self.z})"

    def __str__(self) -> str:
        return f"({self.x:.3f}, {self.y:.3f}, {self.z:.3f})"


class Vector4:
    """4D Vector wrapper around PyGLM vec4"""

    def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0, w: float = 0.0):
        self._vec = glm.vec4(x, y, z, w)

    @property
    def x(self) -> float:
        return self._vec.x

    @x.setter
    def x(self, value: float):
        self._vec.x = value

    @property
    def y(self) -> float:
        return self._vec.y

    @y.setter
    def y(self, value: float):
        self._vec.y = value

    @property
    def z(self) -> float:
        return self._vec.z

    @z.setter
    def z(self, value: float):
        self._vec.z = value

    @property
    def w(self) -> float:
        return self._vec.w

    @w.setter
    def w(self, value: float):
        self._vec.w = value

    def __add__(self, other: "Vector4") -> "Vector4":
        result = Vector4()
        result._vec = self._vec + other._vec
        return result

    def __sub__(self, other: "Vector4") -> "Vector4":
        result = Vector4()
        result._vec = self._vec - other._vec
        return result

    def __mul__(self, scalar: float) -> "Vector4":
        result = Vector4()
        result._vec = self._vec * scalar
        return result

    def __rmul__(self, scalar: float) -> "Vector4":
        return self.__mul__(scalar)

    def __truediv__(self, scalar: float) -> "Vector4":
        result = Vector4()
        result._vec = self._vec / scalar
        return result

    def dot(self, other: "Vector4") -> float:
        """Dot product"""
        return glm.dot(self._vec, other._vec)

    def length(self) -> float:
        """Vector magnitude"""
        return glm.length(self._vec)

    def normalize(self) -> "Vector4":
        """Returns normalized copy"""
        result = Vector4()
        result._vec = glm.normalize(self._vec)
        return result

    def to_tuple(self) -> Tuple[float, float, float, float]:
        """Convert to tuple"""
        return (self.x, self.y, self.z, self.w)

    def to_array(self) -> np.ndarray:
        """Convert to numpy array"""
        return np.array([self.x, self.y, self.z, self.w], dtype=np.float32)

    def to_vector3(self) -> Vector3:
        """Convert to Vector3 (drops w component)"""
        return Vector3(self.x, self.y, self.z)

    @staticmethod
    def zero() -> "Vector4":
        """Zero vector (0, 0, 0, 0)"""
        return Vector4(0, 0, 0, 0)

    @staticmethod
    def one() -> "Vector4":
        """One vector (1, 1, 1, 1)"""
        return Vector4(1, 1, 1, 1)

    def __repr__(self) -> str:
        return f"Vector4({self.x}, {self.y}, {self.z}, {self.w})"

    def __str__(self) -> str:
        return f"({self.x:.3f}, {self.y:.3f}, {self.z:.3f}, {self.w:.3f})"
