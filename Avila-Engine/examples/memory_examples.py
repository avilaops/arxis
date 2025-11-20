"""
Avila Engine - Memory Management Examples
Demonstração dos allocators customizados
"""

import time
from memory import (
    PoolAllocator,
    ArenaAllocator,
    StackAllocator,
    MemoryManager,
    get_memory_manager,
)
from memory.arena import ScopedArena
from memory.stack import ScopedStack


def pool_allocator_example():
    """Exemplo de Pool Allocator - ideal para objetos do mesmo tamanho"""
    print("\n" + "=" * 70)
    print("POOL ALLOCATOR - Fixed-Size Block Allocation")
    print("=" * 70)

    # Criar pool para partículas (64 bytes cada, 100 partículas)
    particle_pool = PoolAllocator(block_size=64, block_count=100, name="ParticlePool")

    print(f"\nCriado: {particle_pool}")

    # Simular sistema de partículas
    particles = []

    print("\n--- Alocando 10 partículas ---")
    for i in range(10):
        particle = particle_pool.allocate(64)
        if particle:
            particles.append(particle)
            # Escrever dados na partícula
            particle.data[0:4] = i.to_bytes(4, "little")

    print(
        f"Partículas ativas: {particle_pool.get_used_blocks()}/{particle_pool.block_count}"
    )
    print(f"Memória usada: {particle_pool.get_used():,} bytes")
    print(f"Utilização: {particle_pool.get_utilization()*100:.1f}%")

    # Liberar algumas partículas
    print("\n--- Liberando 5 partículas ---")
    for _ in range(5):
        if particles:
            particle = particles.pop(0)
            particle_pool.free(particle)

    print(
        f"Partículas ativas: {particle_pool.get_used_blocks()}/{particle_pool.block_count}"
    )
    print(f"Blocos livres: {particle_pool.get_free_blocks()}")

    # Realocar partículas
    print("\n--- Realocando 3 novas partículas ---")
    for i in range(3):
        particle = particle_pool.allocate(64)
        if particle:
            particles.append(particle)

    print(
        f"Partículas ativas: {particle_pool.get_used_blocks()}/{particle_pool.block_count}"
    )

    # Estatísticas
    print("\n--- Estatísticas ---")
    print(particle_pool.get_stats())

    # Reset do pool
    print("\n--- Reset do pool ---")
    particle_pool.reset()
    print(f"Partículas ativas após reset: {particle_pool.get_used_blocks()}")


def arena_allocator_example():
    """Exemplo de Arena Allocator - ideal para alocações temporárias por frame"""
    print("\n" + "=" * 70)
    print("ARENA ALLOCATOR - Linear/Sequential Allocation")
    print("=" * 70)

    # Criar arena de 1KB para alocações temporárias por frame
    frame_arena = ArenaAllocator(capacity=1024, name="FrameArena")

    print(f"\nCriado: {frame_arena}")

    # Simular alocações de um frame
    print("\n--- Frame 1: Alocações temporárias ---")

    temp_string = frame_arena.allocate(64)
    if temp_string:
        temp_string.write(b"Temporary string data for frame 1")
        print(f"String temporária alocada: {temp_string}")

    temp_buffer = frame_arena.allocate(256)
    print(f"Buffer temporário alocado: {temp_buffer}")

    temp_data = frame_arena.allocate(128)
    print(f"Dados temporários alocados: {temp_data}")

    print(
        f"\nMemória usada no frame: {frame_arena.get_used()}/{frame_arena.get_capacity()}"
    )
    print(f"Utilização: {frame_arena.get_utilization()*100:.1f}%")

    # Fim do frame - resetar arena
    print("\n--- Fim do frame - Reset ---")
    frame_arena.reset()
    print(f"Memória usada após reset: {frame_arena.get_used()}")

    # Próximo frame
    print("\n--- Frame 2: Novas alocações ---")
    for i in range(5):
        alloc = frame_arena.allocate(100)
        if alloc:
            print(f"  Alocação {i+1}: {alloc}")

    print(f"Memória usada: {frame_arena.get_used()}/{frame_arena.get_capacity()}")

    # Usar scoped arena para alocações temporárias dentro de um frame
    print("\n--- Usando ScopedArena ---")
    print(f"Antes do scope: {frame_arena.get_used()} bytes usados")

    with ScopedArena(frame_arena):
        temp1 = frame_arena.allocate(50)
        temp2 = frame_arena.allocate(75)
        print(f"Dentro do scope: {frame_arena.get_used()} bytes usados")

    print(
        f"Depois do scope: {frame_arena.get_used()} bytes usados (restaurado automaticamente)"
    )

    # Estatísticas
    print("\n--- Estatísticas ---")
    print(f"Peak usage: {frame_arena.get_peak_usage()} bytes")
    print(frame_arena.get_stats())


def stack_allocator_example():
    """Exemplo de Stack Allocator - ideal para alocações hierárquicas/aninhadas"""
    print("\n" + "=" * 70)
    print("STACK ALLOCATOR - LIFO Hierarchical Allocation")
    print("=" * 70)

    # Criar stack de 2KB
    call_stack = StackAllocator(capacity=2048, name="CallStack")

    print(f"\nCriado: {call_stack}")

    # Simular pilha de chamadas com alocações
    print("\n--- Simulando pilha de chamadas ---")

    # Função A aloca memória
    print("\n1. Entrando em função A")
    func_a_local = call_stack.allocate(128)
    print(f"   Alocado: {func_a_local}")
    print(f"   Stack depth: {call_stack.get_allocation_count()}")

    # Função B aloca memória
    print("\n2. Entrando em função B (chamada por A)")
    func_b_local = call_stack.allocate(256)
    print(f"   Alocado: {func_b_local}")
    print(f"   Stack depth: {call_stack.get_allocation_count()}")

    # Função C aloca memória
    print("\n3. Entrando em função C (chamada por B)")
    func_c_local1 = call_stack.allocate(64)
    func_c_local2 = call_stack.allocate(32)
    print(f"   Alocado: {func_c_local1}, {func_c_local2}")
    print(f"   Stack depth: {call_stack.get_allocation_count()}")
    print(f"   Uso atual: {call_stack.get_used()} bytes")

    # Retornar de C (liberar em ordem LIFO)
    print("\n4. Saindo de função C")
    call_stack.free(func_c_local2)
    call_stack.free(func_c_local1)
    print(f"   Stack depth: {call_stack.get_allocation_count()}")

    # Retornar de B
    print("\n5. Saindo de função B")
    call_stack.free(func_b_local)
    print(f"   Stack depth: {call_stack.get_allocation_count()}")

    # Retornar de A
    print("\n6. Saindo de função A")
    call_stack.free(func_a_local)
    print(f"   Stack depth: {call_stack.get_allocation_count()}")
    print(f"   Uso atual: {call_stack.get_used()} bytes")

    # Usar marker para liberação em lote
    print("\n--- Usando markers para scope-based allocation ---")

    marker = call_stack.get_marker()
    print(f"Marker salvo: {marker}")

    # Alocar vários objetos
    for i in range(5):
        alloc = call_stack.allocate(50)
        print(f"  Alocação {i+1}: {alloc}")

    print(f"Uso antes de free_to_marker: {call_stack.get_used()} bytes")

    # Liberar todos de uma vez
    call_stack.free_to_marker(marker)
    print(f"Uso após free_to_marker: {call_stack.get_used()} bytes")

    # Usar ScopedStack
    print("\n--- Usando ScopedStack ---")
    print(f"Antes do scope: {call_stack.get_used()} bytes")

    with ScopedStack(call_stack):
        temp1 = call_stack.allocate(100)
        temp2 = call_stack.allocate(150)
        print(f"Dentro do scope: {call_stack.get_used()} bytes")

    print(f"Depois do scope: {call_stack.get_used()} bytes (liberado automaticamente)")

    # Estatísticas
    print("\n--- Estatísticas ---")
    print(f"Peak usage: {call_stack.get_peak_usage()} bytes")
    print(call_stack.get_stats())


def memory_manager_example():
    """Exemplo de Memory Manager - gerenciamento centralizado"""
    print("\n" + "=" * 70)
    print("MEMORY MANAGER - Centralized Memory Management")
    print("=" * 70)

    # Obter memory manager global
    manager = get_memory_manager()

    # Criar múltiplos allocators
    print("\n--- Criando allocators ---")

    particle_pool = manager.create_pool(
        name="ParticleSystem", block_size=128, block_count=500, set_as_default=True
    )

    frame_arena = manager.create_arena(name="FrameMemory", capacity=1024 * 1024)  # 1MB

    temp_stack = manager.create_stack(name="TempStack", capacity=512 * 1024)  # 512KB

    level_arena = manager.create_arena(
        name="LevelData", capacity=10 * 1024 * 1024  # 10MB
    )

    print(f"\nAllocators registrados: {manager.list_allocators()}")
    print(f"Default allocator: {manager._default_allocator}")

    # Fazer algumas alocações
    print("\n--- Fazendo alocações ---")

    # Pool
    for i in range(50):
        particle_pool.allocate(128)

    # Arena
    for i in range(100):
        frame_arena.allocate(1024)

    # Stack
    for i in range(20):
        temp_stack.allocate(512)

    # Imprimir relatório
    manager.print_report()

    # Relatório detalhado
    print("\n--- Relatório Detalhado ---")
    manager.print_report(detailed=True)

    # Informações globais
    print("\n--- Informações Globais ---")
    print(f"Capacidade total: {manager.get_total_capacity():,} bytes")
    print(f"Memória usada: {manager.get_total_used():,} bytes")
    print(f"Memória livre: {manager.get_total_free():,} bytes")
    print(f"Utilização geral: {manager.get_overall_utilization()*100:.1f}%")


def performance_comparison():
    """Comparar performance dos diferentes allocators"""
    print("\n" + "=" * 70)
    print("PERFORMANCE COMPARISON")
    print("=" * 70)

    num_iterations = 10000
    allocation_size = 64

    # Pool Allocator
    print("\n--- Pool Allocator ---")
    pool = PoolAllocator(allocation_size, num_iterations, name="PerfPool")

    start = time.perf_counter()
    allocations = []
    for _ in range(num_iterations):
        alloc = pool.allocate(allocation_size)
        if alloc:
            allocations.append(alloc)
    allocation_time = time.perf_counter() - start

    start = time.perf_counter()
    for alloc in allocations:
        pool.free(alloc)
    free_time = time.perf_counter() - start

    print(f"Alocações: {num_iterations:,}")
    print(
        f"Tempo de alocação: {allocation_time*1000:.2f}ms ({allocation_time/num_iterations*1000000:.2f}µs/alloc)"
    )
    print(
        f"Tempo de liberação: {free_time*1000:.2f}ms ({free_time/num_iterations*1000000:.2f}µs/free)"
    )
    print(f"Total: {(allocation_time + free_time)*1000:.2f}ms")

    # Arena Allocator
    print("\n--- Arena Allocator ---")
    arena = ArenaAllocator(num_iterations * allocation_size * 2, name="PerfArena")

    start = time.perf_counter()
    for _ in range(num_iterations):
        arena.allocate(allocation_size)
    allocation_time = time.perf_counter() - start

    start = time.perf_counter()
    arena.reset()
    reset_time = time.perf_counter() - start

    print(f"Alocações: {num_iterations:,}")
    print(
        f"Tempo de alocação: {allocation_time*1000:.2f}ms ({allocation_time/num_iterations*1000000:.2f}µs/alloc)"
    )
    print(f"Tempo de reset: {reset_time*1000:.3f}ms")
    print(f"Total: {(allocation_time + reset_time)*1000:.2f}ms")

    # Stack Allocator
    print("\n--- Stack Allocator ---")
    stack = StackAllocator(num_iterations * allocation_size * 2, name="PerfStack")

    start = time.perf_counter()
    stack_allocations = []
    for _ in range(num_iterations):
        alloc = stack.allocate(allocation_size)
        if alloc:
            stack_allocations.append(alloc)
    allocation_time = time.perf_counter() - start

    start = time.perf_counter()
    for alloc in reversed(stack_allocations):
        stack.free(alloc)
    free_time = time.perf_counter() - start

    print(f"Alocações: {num_iterations:,}")
    print(
        f"Tempo de alocação: {allocation_time*1000:.2f}ms ({allocation_time/num_iterations*1000000:.2f}µs/alloc)"
    )
    print(
        f"Tempo de liberação: {free_time*1000:.2f}ms ({free_time/num_iterations*1000000:.2f}µs/free)"
    )
    print(f"Total: {(allocation_time + free_time)*1000:.2f}ms")


def real_world_scenario():
    """Cenário de uso real - game engine frame"""
    print("\n" + "=" * 70)
    print("CENÁRIO REAL - Game Engine Frame")
    print("=" * 70)

    manager = MemoryManager()

    # Setup allocators para diferentes subsistemas
    particle_pool = manager.create_pool("Particles", block_size=64, block_count=1000)
    entity_pool = manager.create_pool("Entities", block_size=256, block_count=500)
    frame_arena = manager.create_arena("FrameTemp", capacity=1024 * 1024)
    render_stack = manager.create_stack("RenderCommands", capacity=512 * 1024)

    print("\n--- Início do Frame ---")
    manager.print_report()

    # Simular frame de jogo
    print("\n--- Processando Frame ---")

    # Spawnar partículas
    particles = []
    for i in range(50):
        p = particle_pool.allocate(64)
        if p:
            particles.append(p)
    print(f"Partículas spawned: {len(particles)}")

    # Criar entidades temporárias
    for i in range(10):
        entity_pool.allocate(256)

    # Alocações temporárias para processamento
    with ScopedArena(frame_arena):
        for i in range(100):
            frame_arena.allocate(512)  # Buffers temporários

    # Comandos de renderização
    with ScopedStack(render_stack):
        for i in range(200):
            render_stack.allocate(128)  # Comandos de draw

    print("\n--- Durante o Frame ---")
    manager.print_report()

    # Fim do frame - cleanup
    print("\n--- Fim do Frame ---")

    # Despawnar algumas partículas
    for i in range(25):
        if particles:
            particle_pool.free(particles.pop())

    # Reset de memória temporária
    frame_arena.reset()
    render_stack.reset()

    print("\n--- Após Cleanup ---")
    manager.print_report()


def main():
    """Executar todos os exemplos"""
    print("\n")
    print("╔" + "=" * 68 + "╗")
    print("║      AVILA ENGINE - MEMORY MANAGEMENT EXAMPLES                   ║")
    print("╚" + "=" * 68 + "╝")

    pool_allocator_example()
    arena_allocator_example()
    stack_allocator_example()
    memory_manager_example()
    performance_comparison()
    real_world_scenario()

    print("\n" + "=" * 70)
    print("TODOS OS EXEMPLOS EXECUTADOS COM SUCESSO!")
    print("=" * 70 + "\n")


if __name__ == "__main__":
    main()
