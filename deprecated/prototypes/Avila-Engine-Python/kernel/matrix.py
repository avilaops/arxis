"""
Matrix4x4 class for 3D transformations
"""

import glm
import numpy as np
from typing import Tuple
from .vector import Vector3, Vector4


class Matrix4x4:
    """4x4 Matrix wrapper around PyGLM mat4"""

    def __init__(self):
        self._mat = glm.mat4(1.0)  # Identity matrix by default

    def __mul__(self, other) -> "Matrix4x4":
        """Matrix multiplication"""
        if isinstance(other, Matrix4x4):
            result = Matrix4x4()
            result._mat = self._mat * other._mat
            return result
        elif isinstance(other, Vector4):
            vec_result = self._mat * other._vec
            result = Vector4()
            result._vec = vec_result
            return result
        else:
            raise TypeError(f"Cannot multiply Matrix4x4 with {type(other)}")

    @staticmethod
    def identity() -> "Matrix4x4":
        """Identity matrix"""
        return Matrix4x4()

    @staticmethod
    def translate(translation: Vector3) -> "Matrix4x4":
        """Create translation matrix"""
        result = Matrix4x4()
        result._mat = glm.translate(glm.mat4(1.0), translation._vec)
        return result

    @staticmethod
    def rotate(angle_radians: float, axis: Vector3) -> "Matrix4x4":
        """Create rotation matrix"""
        result = Matrix4x4()
        result._mat = glm.rotate(glm.mat4(1.0), angle_radians, axis._vec)
        return result

    @staticmethod
    def rotate_x(angle_radians: float) -> "Matrix4x4":
        """Rotate around X axis"""
        return Matrix4x4.rotate(angle_radians, Vector3(1, 0, 0))

    @staticmethod
    def rotate_y(angle_radians: float) -> "Matrix4x4":
        """Rotate around Y axis"""
        return Matrix4x4.rotate(angle_radians, Vector3(0, 1, 0))

    @staticmethod
    def rotate_z(angle_radians: float) -> "Matrix4x4":
        """Rotate around Z axis"""
        return Matrix4x4.rotate(angle_radians, Vector3(0, 0, 1))

    @staticmethod
    def scale(scale: Vector3) -> "Matrix4x4":
        """Create scale matrix"""
        result = Matrix4x4()
        result._mat = glm.scale(glm.mat4(1.0), scale._vec)
        return result

    @staticmethod
    def scale_uniform(scale: float) -> "Matrix4x4":
        """Create uniform scale matrix"""
        return Matrix4x4.scale(Vector3(scale, scale, scale))

    @staticmethod
    def look_at(eye: Vector3, center: Vector3, up: Vector3) -> "Matrix4x4":
        """Create view matrix (look at)"""
        result = Matrix4x4()
        result._mat = glm.lookAt(eye._vec, center._vec, up._vec)
        return result

    @staticmethod
    def perspective(
        fov_radians: float, aspect: float, near: float, far: float
    ) -> "Matrix4x4":
        """Create perspective projection matrix"""
        result = Matrix4x4()
        result._mat = glm.perspective(fov_radians, aspect, near, far)
        return result

    @staticmethod
    def orthographic(
        left: float, right: float, bottom: float, top: float, near: float, far: float
    ) -> "Matrix4x4":
        """Create orthographic projection matrix"""
        result = Matrix4x4()
        result._mat = glm.ortho(left, right, bottom, top, near, far)
        return result

    def transpose(self) -> "Matrix4x4":
        """Returns transposed matrix"""
        result = Matrix4x4()
        result._mat = glm.transpose(self._mat)
        return result

    def inverse(self) -> "Matrix4x4":
        """Returns inverse matrix"""
        result = Matrix4x4()
        result._mat = glm.inverse(self._mat)
        return result

    def determinant(self) -> float:
        """Calculate determinant"""
        return glm.determinant(self._mat)

    def transform_point(self, point: Vector3) -> Vector3:
        """Transform a point (w=1)"""
        vec4 = glm.vec4(point._vec, 1.0)
        result_vec4 = self._mat * vec4
        result = Vector3()
        result._vec = glm.vec3(result_vec4) / result_vec4.w
        return result

    def transform_direction(self, direction: Vector3) -> Vector3:
        """Transform a direction (w=0)"""
        vec4 = glm.vec4(direction._vec, 0.0)
        result_vec4 = self._mat * vec4
        result = Vector3()
        result._vec = glm.vec3(result_vec4)
        return result

    def to_array(self) -> np.ndarray:
        """Convert to numpy array (column-major)"""
        return np.array(self._mat, dtype=np.float32)

    def to_list(self) -> list:
        """Convert to list of lists"""
        return [[self._mat[i][j] for j in range(4)] for i in range(4)]

    def get(self, row: int, col: int) -> float:
        """Get element at (row, col)"""
        return self._mat[col][row]  # GLM uses column-major

    def set(self, row: int, col: int, value: float):
        """Set element at (row, col)"""
        self._mat[col][row] = value

    def __repr__(self) -> str:
        return f"Matrix4x4({self.to_list()})"

    def __str__(self) -> str:
        lines = []
        for row in range(4):
            values = [f"{self.get(row, col):8.3f}" for col in range(4)]
            lines.append("[" + " ".join(values) + "]")
        return "\n".join(lines)
