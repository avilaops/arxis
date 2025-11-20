# Avila Engine

Engine 3D desenvolvido em Python com foco em performance e facilidade de uso.

## Kernel - Matemática 3D

O kernel da Avila Engine inclui todas as estruturas matemáticas fundamentais para desenvolvimento de aplicações 3D:

### Estruturas Disponíveis

- **Vector3** - Vetores 3D com operações completas
- **Vector4** - Vetores 4D para cálculos homogêneos
- **Matrix4x4** - Matrizes 4x4 para transformações
- **Quaternion** - Quaternions para rotações suaves
- **AABB** - Axis-Aligned Bounding Box para detecção de colisão

## Memory Management - Sistema de Gerenciamento de Memória

Sistema completo de allocators customizados para controle preciso de memória:

### Allocators Disponíveis

- **PoolAllocator** - Blocos de tamanho fixo para objetos uniformes (partículas, entidades)
- **ArenaAllocator** - Alocação linear/sequencial para memória temporária (per-frame)
- **StackAllocator** - Alocação LIFO para hierarquias (call stack, scopes)
- **MemoryManager** - Gerenciador central com estatísticas e profiling

### Características

- **O(1) allocation/deallocation** em todos os allocators
- **Zero fragmentação** com allocators especializados
- **Estatísticas detalhadas** (uso, pico, tempo de alocação)
- **Scoped allocations** com RAII pattern
- **Thread-safety ready** (base implementada)

## Instalação

### Dependências

```powershell
pip install numpy PyGLM scipy
```

## Uso Básico

```python
from kernel import Vector3, Matrix4x4, Quaternion, AABB

# Vetores
v1 = Vector3(1, 2, 3)
v2 = Vector3(4, 5, 6)
v3 = v1 + v2
dot_product = v1.dot(v2)
cross_product = v1.cross(v2)

# Matrizes
translation = Matrix4x4.translate(Vector3(10, 0, 0))
rotation = Matrix4x4.rotate_y(math.radians(45))
scale = Matrix4x4.scale_uniform(2.0)
transform = translation * rotation * scale

# Quaternions
quat = Quaternion.from_euler_degrees(45, 30, 0)
rotated_vector = quat.rotate_vector(Vector3.forward())

# AABB
box1 = AABB.from_center_size(Vector3.zero(), Vector3(2, 2, 2))
box2 = AABB.from_center_size(Vector3(1, 0, 0), Vector3(2, 2, 2))
if box1.intersects(box2):
    print("Colisão detectada!")
```

### Memory Management

```python
from memory import PoolAllocator, ArenaAllocator, StackAllocator, MemoryManager
from memory.arena import ScopedArena

# Pool para objetos do mesmo tamanho
particle_pool = PoolAllocator(block_size=64, block_count=1000, name="Particles")
particle = particle_pool.allocate(64)
particle_pool.free(particle)

# Arena para alocações temporárias por frame
frame_arena = ArenaAllocator(capacity=1024*1024, name="FrameMemory")
temp_data = frame_arena.allocate(512)
frame_arena.reset()  # Libera tudo de uma vez

# Stack para alocações hierárquicas
call_stack = StackAllocator(capacity=512*1024, name="CallStack")
local1 = call_stack.allocate(128)
local2 = call_stack.allocate(256)
call_stack.free(local2)  # Deve liberar em ordem LIFO
call_stack.free(local1)

# Memory Manager centralizado
manager = MemoryManager()
manager.create_pool("Entities", block_size=256, block_count=500)
manager.create_arena("LevelData", capacity=10*1024*1024)
manager.print_report()  # Relatório completo de uso

# Scoped allocations (RAII pattern)
with ScopedArena(frame_arena):
    temp = frame_arena.allocate(1024)
    # temp é automaticamente liberado ao sair do scope
```

## Estrutura do Projeto

```
Avila/
├── kernel/
│   ├── __init__.py
│   ├── vector.py       # Vector3 e Vector4
│   ├── matrix.py       # Matrix4x4
│   ├── quaternion.py   # Quaternion
│   └── aabb.py         # AABB
├── memory/
│   ├── __init__.py
│   ├── allocator.py    # Base allocator
│   ├── pool.py         # Pool allocator
│   ├── arena.py        # Arena/Linear allocator
│   ├── stack.py        # Stack allocator
│   └── manager.py      # Memory manager
├── examples/
│   ├── math_examples.py
│   └── memory_examples.py
└── Readme.md
```

## Features do Kernel

### Vector3
- Operações aritméticas (+, -, *, /)
- Produto escalar (dot) e vetorial (cross)
- Normalização e magnitude
- Distância e interpolação linear (lerp)
- Vetores constantes (zero, one, up, down, left, right, forward, back)

### Vector4
- Todas as operações do Vector3
- Conversão para Vector3
- Suporte para coordenadas homogêneas

### Matrix4x4
- Matriz identidade
- Transformações: translate, rotate, scale
- Projeções: perspective, orthographic
- View matrix: lookAt
- Operações: transpose, inverse, determinant
- Transform de pontos e direções

### Quaternion
- Criação a partir de eixo-ângulo ou Euler
- Conversão para matriz e Euler
- Interpolação esférica (slerp)
- Rotação de vetores
- Operações: conjugate, inverse, normalize

### AABB
- Criação a partir de pontos ou centro+tamanho
- Testes de interseção e contenção
- Merge e expand
- Cálculo de volume e área de superfície
- Ponto mais próximo e distância

## Memory Management Features

### PoolAllocator
- Fixed-size block allocation (O(1))
- Perfect para objetos uniformes
- Zero fragmentação
- Free list implementado
- Ideal para: partículas, bullets, entidades pooladas

### ArenaAllocator (Linear Allocator)
- Sequential bump allocation (O(1))
- Reset em massa (não há free individual)
- Perfeito para alocações temporárias
- Excelente cache locality
- Ideal para: per-frame memory, level loading, string operations

### StackAllocator
- LIFO allocation/deallocation (O(1))
- Must free in reverse order
- Markers para scoped allocations
- Zero fragmentação
- Ideal para: call stacks, recursive algorithms, nested scopes

### MemoryManager
- Registro centralizado de allocators
- Estatísticas combinadas
- Relatórios de uso detalhados
- Detecção de leaks
- Gerenciamento de budgets

## Exemplos de Uso

Execute os exemplos:

```powershell
# Exemplos de matemática 3D
python examples/math_examples.py

# Exemplos de memory management
python examples/memory_examples.py
```

## Próximos Passos

- [ ] Implementar Ray, Plane, Sphere
- [ ] Adicionar frustum culling
- [ ] Sistema de transformações hierárquicas
- [ ] Otimizações SIMD

## Licença

MIT License
