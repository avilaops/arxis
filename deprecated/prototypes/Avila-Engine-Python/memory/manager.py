"""
Memory Manager - Central management for all allocators
"""

from typing import Dict, Optional, List
from .allocator import Allocator, AllocationStats
from .pool import PoolAllocator
from .arena import ArenaAllocator
from .stack import StackAllocator


class MemoryManager:
    """
    Central Memory Manager

    Manages multiple allocators and provides:
    - Allocator registry
    - Global statistics
    - Memory budgets
    - Leak detection
    - Performance profiling
    """

    def __init__(self):
        self.allocators: Dict[str, Allocator] = {}
        self._default_allocator: Optional[str] = None

    def register_allocator(
        self, allocator: Allocator, set_as_default: bool = False
    ) -> bool:
        """
        Register an allocator with the manager

        Args:
            allocator: Allocator instance to register
            set_as_default: If True, set as default allocator

        Returns:
            True if successful, False if name already exists
        """
        if allocator.name in self.allocators:
            print(f"[MemoryManager] Allocator '{allocator.name}' already registered!")
            return False

        self.allocators[allocator.name] = allocator

        if set_as_default or self._default_allocator is None:
            self._default_allocator = allocator.name

        return True

    def unregister_allocator(self, name: str) -> bool:
        """
        Unregister an allocator

        Args:
            name: Name of allocator to remove

        Returns:
            True if successful, False if not found
        """
        if name not in self.allocators:
            return False

        del self.allocators[name]

        if self._default_allocator == name:
            self._default_allocator = None
            if self.allocators:
                self._default_allocator = next(iter(self.allocators.keys()))

        return True

    def get_allocator(self, name: str) -> Optional[Allocator]:
        """Get allocator by name"""
        return self.allocators.get(name)

    def get_default_allocator(self) -> Optional[Allocator]:
        """Get the default allocator"""
        if self._default_allocator:
            return self.allocators.get(self._default_allocator)
        return None

    def set_default_allocator(self, name: str) -> bool:
        """Set default allocator by name"""
        if name not in self.allocators:
            return False
        self._default_allocator = name
        return True

    def list_allocators(self) -> List[str]:
        """Get list of registered allocator names"""
        return list(self.allocators.keys())

    def get_total_capacity(self) -> int:
        """Get total capacity across all allocators"""
        return sum(alloc.get_capacity() for alloc in self.allocators.values())

    def get_total_used(self) -> int:
        """Get total used memory across all allocators"""
        return sum(alloc.get_used() for alloc in self.allocators.values())

    def get_total_free(self) -> int:
        """Get total free memory across all allocators"""
        return self.get_total_capacity() - self.get_total_used()

    def get_overall_utilization(self) -> float:
        """Get overall utilization percentage"""
        capacity = self.get_total_capacity()
        if capacity == 0:
            return 0.0
        return self.get_total_used() / capacity

    def get_combined_stats(self) -> AllocationStats:
        """Get combined statistics from all allocators"""
        combined = AllocationStats()

        for allocator in self.allocators.values():
            stats = allocator.get_stats()
            combined.total_allocated += stats.total_allocated
            combined.total_freed += stats.total_freed
            combined.current_usage += stats.current_usage
            combined.peak_usage += stats.peak_usage
            combined.num_allocations += stats.num_allocations
            combined.num_frees += stats.num_frees
            combined.num_resets += stats.num_resets
            combined.allocation_time += stats.allocation_time
            combined.free_time += stats.free_time

        return combined

    def reset_all_stats(self) -> None:
        """Reset statistics for all allocators"""
        for allocator in self.allocators.values():
            allocator.reset_stats()

    def reset_all_allocators(self) -> None:
        """Reset all allocators (free all memory)"""
        for allocator in self.allocators.values():
            allocator.reset()

    def enable_all(self) -> None:
        """Enable all allocators"""
        for allocator in self.allocators.values():
            allocator.enable()

    def disable_all(self) -> None:
        """Disable all allocators"""
        for allocator in self.allocators.values():
            allocator.disable()

    def print_report(self, detailed: bool = False) -> None:
        """
        Print memory usage report

        Args:
            detailed: If True, print detailed stats for each allocator
        """
        print("\n" + "=" * 70)
        print("AVILA ENGINE - MEMORY REPORT")
        print("=" * 70)

        print(f"\nRegistered Allocators: {len(self.allocators)}")
        print(f"Default Allocator: {self._default_allocator or 'None'}")

        print(f"\n{'Allocator':<20} {'Type':<15} {'Used/Capacity':<25} {'Util %':<10}")
        print("-" * 70)

        for name, allocator in self.allocators.items():
            alloc_type = allocator.__class__.__name__
            used = allocator.get_used()
            capacity = allocator.get_capacity()
            utilization = allocator.get_utilization() * 100

            marker = "* " if name == self._default_allocator else "  "

            print(
                f"{marker}{name:<18} {alloc_type:<15} "
                f"{used:>10,}/{capacity:<10,} {utilization:>6.1f}%"
            )

        print("-" * 70)

        total_used = self.get_total_used()
        total_capacity = self.get_total_capacity()
        overall_util = self.get_overall_utilization() * 100

        print(
            f"{'TOTAL':<20} {'':<15} "
            f"{total_used:>10,}/{total_capacity:<10,} {overall_util:>6.1f}%"
        )

        # Combined stats
        combined = self.get_combined_stats()
        print(f"\n{'Combined Statistics:':<30}")
        print(f"  Total Allocated:  {combined.total_allocated:>15,} bytes")
        print(f"  Total Freed:      {combined.total_freed:>15,} bytes")
        print(f"  Current Usage:    {combined.current_usage:>15,} bytes")
        print(f"  Peak Usage:       {combined.peak_usage:>15,} bytes")
        print(f"  Allocations:      {combined.num_allocations:>15,}")
        print(f"  Frees:            {combined.num_frees:>15,}")
        print(f"  Resets:           {combined.num_resets:>15,}")

        if detailed:
            print("\n" + "=" * 70)
            print("DETAILED ALLOCATOR STATS")
            print("=" * 70)

            for name, allocator in self.allocators.items():
                print(f"\n{name} ({allocator.__class__.__name__}):")
                print(allocator.get_stats())

        print("=" * 70 + "\n")

    def create_pool(
        self,
        name: str,
        block_size: int,
        block_count: int,
        alignment: int = 8,
        set_as_default: bool = False,
    ) -> Optional[PoolAllocator]:
        """
        Convenience method to create and register a pool allocator

        Args:
            name: Allocator name
            block_size: Size of each block
            block_count: Number of blocks
            alignment: Memory alignment
            set_as_default: Set as default allocator

        Returns:
            PoolAllocator instance or None if name exists
        """
        if name in self.allocators:
            print(f"[MemoryManager] Allocator '{name}' already exists!")
            return None

        pool = PoolAllocator(block_size, block_count, alignment, name)
        self.register_allocator(pool, set_as_default)
        return pool

    def create_arena(
        self, name: str, capacity: int, set_as_default: bool = False
    ) -> Optional[ArenaAllocator]:
        """
        Convenience method to create and register an arena allocator

        Args:
            name: Allocator name
            capacity: Total capacity in bytes
            set_as_default: Set as default allocator

        Returns:
            ArenaAllocator instance or None if name exists
        """
        if name in self.allocators:
            print(f"[MemoryManager] Allocator '{name}' already exists!")
            return None

        arena = ArenaAllocator(capacity, name)
        self.register_allocator(arena, set_as_default)
        return arena

    def create_stack(
        self, name: str, capacity: int, set_as_default: bool = False
    ) -> Optional[StackAllocator]:
        """
        Convenience method to create and register a stack allocator

        Args:
            name: Allocator name
            capacity: Total capacity in bytes
            set_as_default: Set as default allocator

        Returns:
            StackAllocator instance or None if name exists
        """
        if name in self.allocators:
            print(f"[MemoryManager] Allocator '{name}' already exists!")
            return None

        stack = StackAllocator(capacity, name)
        self.register_allocator(stack, set_as_default)
        return stack

    def __repr__(self) -> str:
        return (
            f"MemoryManager(allocators={len(self.allocators)}, "
            f"capacity={self.get_total_capacity():,}, "
            f"used={self.get_total_used():,}, "
            f"utilization={self.get_overall_utilization()*100:.1f}%)"
        )


# Global memory manager instance
_global_memory_manager: Optional[MemoryManager] = None


def get_memory_manager() -> MemoryManager:
    """Get or create the global memory manager instance"""
    global _global_memory_manager
    if _global_memory_manager is None:
        _global_memory_manager = MemoryManager()
    return _global_memory_manager


def reset_memory_manager() -> None:
    """Reset the global memory manager"""
    global _global_memory_manager
    _global_memory_manager = None
