"""
Avila Engine - Math Examples
Demonstração das estruturas matemáticas do kernel
"""

import math
from kernel import Vector3, Vector4, Matrix4x4, Quaternion, AABB


def vector_examples():
    """Exemplos de uso de Vector3 e Vector4"""
    print("=" * 50)
    print("EXEMPLOS DE VETORES")
    print("=" * 50)

    # Criação de vetores
    v1 = Vector3(1, 2, 3)
    v2 = Vector3(4, 5, 6)
    print(f"v1 = {v1}")
    print(f"v2 = {v2}")

    # Operações básicas
    print(f"\nv1 + v2 = {v1 + v2}")
    print(f"v1 - v2 = {v1 - v2}")
    print(f"v1 * 2 = {v1 * 2}")
    print(f"v1 / 2 = {v1 / 2}")

    # Produtos
    print(f"\nDot product: {v1.dot(v2)}")
    print(f"Cross product: {v1.cross(v2)}")

    # Magnitude e normalização
    print(f"\nLength of v1: {v1.length():.3f}")
    print(f"Normalized v1: {v1.normalize()}")

    # Vetores constantes
    print(f"\nUp: {Vector3.up()}")
    print(f"Forward: {Vector3.forward()}")
    print(f"Right: {Vector3.right()}")

    # Vector4
    v4 = Vector4(1, 2, 3, 1)
    print(f"\nVector4: {v4}")
    print(f"To Vector3: {v4.to_vector3()}")


def matrix_examples():
    """Exemplos de uso de Matrix4x4"""
    print("\n" + "=" * 50)
    print("EXEMPLOS DE MATRIZES")
    print("=" * 50)

    # Transformações básicas
    translation = Matrix4x4.translate(Vector3(10, 5, 0))
    print("\nTranslation Matrix:")
    print(translation)

    rotation = Matrix4x4.rotate_y(math.radians(90))
    print("\nRotation Matrix (90° around Y):")
    print(rotation)

    scale = Matrix4x4.scale(Vector3(2, 2, 2))
    print("\nScale Matrix:")
    print(scale)

    # Combinação de transformações
    transform = translation * rotation * scale
    print("\nCombined Transform (T * R * S):")
    print(transform)

    # Transformar um ponto
    point = Vector3(1, 0, 0)
    transformed = transform.transform_point(point)
    print(f"\nTransform point {point} -> {transformed}")

    # Matrizes de projeção
    perspective = Matrix4x4.perspective(math.radians(60), 16 / 9, 0.1, 100)
    print("\nPerspective Matrix (60° FOV, 16:9):")
    print(perspective)

    # LookAt matrix
    eye = Vector3(0, 10, 10)
    target = Vector3.zero()
    up = Vector3.up()
    view = Matrix4x4.look_at(eye, target, up)
    print(f"\nView Matrix (eye={eye}, target={target}):")
    print(view)


def quaternion_examples():
    """Exemplos de uso de Quaternion"""
    print("\n" + "=" * 50)
    print("EXEMPLOS DE QUATERNIONS")
    print("=" * 50)

    # Criação a partir de eixo-ângulo
    quat1 = Quaternion.from_axis_angle(Vector3.up(), math.radians(90))
    print(f"\nQuaternion (90° around Y): {quat1}")

    # Criação a partir de Euler
    quat2 = Quaternion.from_euler_degrees(45, 30, 0)
    print(f"Quaternion from Euler(45, 30, 0): {quat2}")

    # Converter para Euler
    euler = quat2.to_euler_degrees()
    print(
        f"Back to Euler: pitch={euler[0]:.1f}°, yaw={euler[1]:.1f}°, roll={euler[2]:.1f}°"
    )

    # Rotacionar um vetor
    forward = Vector3.forward()
    rotated = quat1.rotate_vector(forward)
    print(f"\nRotate {forward} by 90° around Y: {rotated}")

    # Look rotation
    direction = Vector3(1, 0, -1).normalize()
    look_quat = Quaternion.look_rotation(direction)
    print(f"\nLook rotation towards {direction}: {look_quat}")

    # SLERP
    q1 = Quaternion.identity()
    q2 = Quaternion.from_euler_degrees(0, 90, 0)
    q_mid = q1.slerp(q2, 0.5)
    print(f"\nSLERP between identity and 90° rotation (t=0.5):")
    print(f"  Euler angles: {q_mid.to_euler_degrees()}")

    # Converter para matriz
    matrix = quat1.to_matrix()
    print(f"\nQuaternion to Matrix:")
    print(matrix)


def aabb_examples():
    """Exemplos de uso de AABB"""
    print("\n" + "=" * 50)
    print("EXEMPLOS DE AABB")
    print("=" * 50)

    # Criar AABB
    box1 = AABB.from_center_size(Vector3(0, 0, 0), Vector3(2, 2, 2))
    print(f"\nBox 1: {box1}")
    print(f"  Center: {box1.center()}")
    print(f"  Size: {box1.size()}")
    print(f"  Volume: {box1.volume()}")

    # Criar a partir de pontos
    points = [Vector3(0, 0, 0), Vector3(1, 2, 3), Vector3(-1, 1, -1), Vector3(2, -1, 1)]
    box2 = AABB.from_points(points)
    print(f"\nBox 2 (from points): {box2}")

    # Teste de interseção
    box3 = AABB.from_center_size(Vector3(1, 0, 0), Vector3(2, 2, 2))
    print(f"\nBox 3: {box3}")
    print(f"Box 1 intersects Box 3? {box1.intersects(box3)}")

    # Interseção
    if box1.intersects(box3):
        intersection = box1.intersection(box3)
        print(f"Intersection volume: {intersection.volume():.2f}")

    # Merge
    merged = box1.merge(box3)
    print(f"\nMerged box: {merged}")
    print(f"  Volume: {merged.volume()}")

    # Teste de ponto
    point = Vector3(0.5, 0.5, 0.5)
    print(f"\nPoint {point} inside Box 1? {box1.contains_point(point)}")

    point2 = Vector3(5, 5, 5)
    print(f"Point {point2} inside Box 1? {box1.contains_point(point2)}")

    # Ponto mais próximo
    closest = box1.closest_point(point2)
    distance = box1.distance_to_point(point2)
    print(f"\nClosest point on Box 1 to {point2}: {closest}")
    print(f"Distance: {distance:.3f}")

    # Expandir
    expanded = box1.expand(1.0)
    print(f"\nExpanded Box 1 by 1.0: {expanded}")

    # Corners
    corners = box1.corners()
    print(f"\nBox 1 corners:")
    for i, corner in enumerate(corners):
        print(f"  {i}: {corner}")


def transformation_pipeline_example():
    """Exemplo de pipeline de transformação completo"""
    print("\n" + "=" * 50)
    print("PIPELINE DE TRANSFORMAÇÃO 3D")
    print("=" * 50)

    # Objeto no espaço local
    local_vertex = Vector3(1, 0, 0)
    print(f"\nVertex em espaço local: {local_vertex}")

    # Model transform (objeto no mundo)
    model_translation = Matrix4x4.translate(Vector3(5, 2, -10))
    model_rotation = Matrix4x4.rotate_y(math.radians(45))
    model_scale = Matrix4x4.scale_uniform(2.0)
    model_matrix = model_translation * model_rotation * model_scale

    world_vertex = model_matrix.transform_point(local_vertex)
    print(f"Vertex em espaço mundial: {world_vertex}")

    # View transform (câmera)
    camera_pos = Vector3(0, 5, 10)
    camera_target = Vector3(0, 0, 0)
    view_matrix = Matrix4x4.look_at(camera_pos, camera_target, Vector3.up())

    view_vertex = view_matrix.transform_point(world_vertex)
    print(f"Vertex em espaço de visão: {view_vertex}")

    # Projection transform
    projection_matrix = Matrix4x4.perspective(
        math.radians(60),  # FOV
        16 / 9,  # Aspect ratio
        0.1,  # Near plane
        100.0,  # Far plane
    )

    # MVP matrix
    mvp = projection_matrix * view_matrix * model_matrix
    print(f"\nMVP Matrix calculada!")

    # Transformar usando quaternion
    print("\n--- Rotação com Quaternion ---")
    quat_rotation = Quaternion.from_euler_degrees(0, 45, 0)
    rotated_by_quat = quat_rotation.rotate_vector(local_vertex)
    print(f"Vertex rotacionado por quaternion: {rotated_by_quat}")


def main():
    """Executar todos os exemplos"""
    print("\n")
    print("╔" + "=" * 48 + "╗")
    print("║    AVILA ENGINE - KERNEL MATH EXAMPLES         ║")
    print("╚" + "=" * 48 + "╝")

    vector_examples()
    matrix_examples()
    quaternion_examples()
    aabb_examples()
    transformation_pipeline_example()

    print("\n" + "=" * 50)
    print("TODOS OS EXEMPLOS EXECUTADOS COM SUCESSO!")
    print("=" * 50 + "\n")


if __name__ == "__main__":
    main()
