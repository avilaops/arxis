"""
Arena Allocator (Linear Allocator) - Sequential memory allocation
Ideal for temporary allocations within a frame or scope
"""

from typing import Optional, Any
from .allocator import Allocator


class ArenaAllocation:
    """Handle to an arena allocation"""

    __slots__ = ["offset", "size", "allocator"]

    def __init__(self, offset: int, size: int, allocator: "ArenaAllocator"):
        self.offset = offset
        self.size = size
        self.allocator = allocator

    def get_data(self) -> bytearray:
        """Get the data for this allocation"""
        return self.allocator.buffer[self.offset : self.offset + self.size]

    def write(self, data: bytes, offset: int = 0) -> bool:
        """Write data to this allocation"""
        if offset + len(data) > self.size:
            return False
        self.allocator.buffer[
            self.offset + offset : self.offset + offset + len(data)
        ] = data
        return True

    def __repr__(self) -> str:
        return f"ArenaAllocation(offset={self.offset}, size={self.size})"


class ArenaAllocator(Allocator):
    """
    Arena/Linear Allocator - Sequential bump allocation

    Best for:
    - Per-frame allocations
    - Temporary string operations
    - Level loading
    - Batch operations with known lifetime

    Characteristics:
    - O(1) allocation (just bump pointer)
    - No individual deallocation
    - Must reset entire arena
    - Very fast allocation
    - No fragmentation
    - Excellent cache locality

    Usage Pattern:
        arena = ArenaAllocator(1024 * 1024)  # 1MB arena

        # Use for frame
        obj1 = arena.allocate(256)
        obj2 = arena.allocate(512)
        # ... use objects ...

        # End of frame
        arena.reset()  # Free everything at once
    """

    def __init__(self, capacity: int, name: str = "ArenaAllocator"):
        """
        Create an arena allocator

        Args:
            capacity: Total size in bytes
            name: Allocator name for debugging
        """
        super().__init__(name)

        if capacity <= 0:
            raise ValueError("Capacity must be positive")

        self.capacity = capacity
        self.buffer = bytearray(capacity)
        self.offset = 0  # Current allocation offset
        self.peak_offset = 0  # High water mark

    def allocate(self, size: int, alignment: int = 8) -> Optional[ArenaAllocation]:
        """
        Allocate memory from arena

        Args:
            size: Number of bytes to allocate
            alignment: Memory alignment (default 8 bytes)

        Returns:
            ArenaAllocation or None if not enough space
        """
        if not self._enabled:
            return None

        if size <= 0:
            return None

        # Align current offset
        aligned_offset = self._align(self.offset, alignment)

        # Check if we have enough space
        if aligned_offset + size > self.capacity:
            available = self.capacity - aligned_offset
            print(
                f"[{self.name}] Out of memory! "
                f"Requested: {size} bytes, Available: {available} bytes, "
                f"Usage: {self.offset}/{self.capacity} ({self.get_utilization()*100:.1f}%)"
            )
            return None

        # Allocate
        allocation = ArenaAllocation(aligned_offset, size, self)
        self.offset = aligned_offset + size

        # Update peak usage
        if self.offset > self.peak_offset:
            self.peak_offset = self.offset

        self._track_allocation(size)

        return allocation

    def free(self, ptr: Any) -> bool:
        """
        Arena allocator doesn't support individual frees
        Use reset() to free all allocations at once
        """
        print(
            f"[{self.name}] Warning: Arena allocator doesn't support individual free(). Use reset()."
        )
        return False

    def reset(self) -> None:
        """Reset arena to initial state - frees all allocations"""
        self.offset = 0
        self._track_reset()

    def clear(self) -> None:
        """Alias for reset"""
        self.reset()

    def get_capacity(self) -> int:
        """Get total capacity in bytes"""
        return self.capacity

    def get_used(self) -> int:
        """Get currently used bytes"""
        return self.offset

    def get_peak_usage(self) -> int:
        """Get peak memory usage (high water mark)"""
        return self.peak_offset

    def reset_peak(self) -> None:
        """Reset peak usage counter"""
        self.peak_offset = self.offset

    def _align(self, offset: int, alignment: int) -> int:
        """Align offset to specified boundary"""
        return ((offset + alignment - 1) // alignment) * alignment

    def save_state(self) -> int:
        """
        Save current state (offset) for later restoration
        Allows temporary allocations within a scope
        """
        return self.offset

    def restore_state(self, state: int) -> None:
        """
        Restore previously saved state
        Frees all allocations made after save_state()
        """
        if state < 0 or state > self.offset:
            print(f"[{self.name}] Invalid state: {state}")
            return

        freed_bytes = self.offset - state
        self.offset = state

        if freed_bytes > 0:
            self._track_free(freed_bytes)

    def __repr__(self) -> str:
        return (
            f"ArenaAllocator(name='{self.name}', "
            f"capacity={self.capacity:,}, "
            f"used={self.offset:,}, "
            f"peak={self.peak_offset:,}, "
            f"utilization={self.get_utilization()*100:.1f}%)"
        )


class ScopedArena:
    """
    RAII-style scoped arena allocation
    Automatically restores arena state when scope exits

    Usage:
        arena = ArenaAllocator(1024)

        with ScopedArena(arena) as scoped:
            temp1 = arena.allocate(128)
            temp2 = arena.allocate(256)
            # ... use temp allocations ...
        # All allocations freed here automatically
    """

    def __init__(self, arena: ArenaAllocator):
        self.arena = arena
        self.saved_state = 0

    def __enter__(self) -> "ScopedArena":
        self.saved_state = self.arena.save_state()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.arena.restore_state(self.saved_state)
        return False
