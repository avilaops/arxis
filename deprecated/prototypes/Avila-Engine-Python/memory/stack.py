"""
Stack Allocator - LIFO memory allocation
Ideal for hierarchical/nested allocations
"""

from typing import Optional, List
from .allocator import Allocator


class StackAllocation:
    """Handle to a stack allocation"""

    __slots__ = ["offset", "size", "padding", "allocator"]

    def __init__(
        self, offset: int, size: int, padding: int, allocator: "StackAllocator"
    ):
        self.offset = offset
        self.size = size
        self.padding = padding  # Padding added for alignment
        self.allocator = allocator

    def get_total_size(self) -> int:
        """Get total size including padding"""
        return self.size + self.padding

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
        return f"StackAllocation(offset={self.offset}, size={self.size}, padding={self.padding})"


class StackAllocator(Allocator):
    """
    Stack Allocator - LIFO (Last In, First Out) allocation

    Best for:
    - Nested/hierarchical allocations
    - Function call stacks
    - Scoped allocations
    - Recursive algorithms

    Characteristics:
    - O(1) allocation and deallocation
    - Must free in reverse order (LIFO)
    - No fragmentation
    - Excellent cache locality
    - Memory alignment support

    Usage Pattern:
        stack = StackAllocator(1024 * 1024)  # 1MB stack

        obj1 = stack.allocate(256)
        obj2 = stack.allocate(512)
        obj3 = stack.allocate(128)

        # Must free in reverse order
        stack.free(obj3)
        stack.free(obj2)
        stack.free(obj1)
    """

    def __init__(self, capacity: int, name: str = "StackAllocator"):
        """
        Create a stack allocator

        Args:
            capacity: Total size in bytes
            name: Allocator name for debugging
        """
        super().__init__(name)

        if capacity <= 0:
            raise ValueError("Capacity must be positive")

        self.capacity = capacity
        self.buffer = bytearray(capacity)
        self.top = 0  # Top of stack (current allocation point)
        self.peak_usage = 0  # High water mark
        self.allocation_stack: List[int] = []  # Track allocation offsets for validation

    def allocate(self, size: int, alignment: int = 8) -> Optional[StackAllocation]:
        """
        Allocate memory from stack

        Args:
            size: Number of bytes to allocate
            alignment: Memory alignment (default 8 bytes)

        Returns:
            StackAllocation or None if not enough space
        """
        if not self._enabled:
            return None

        if size <= 0:
            return None

        # Calculate padding needed for alignment
        current_address = self.top
        aligned_address = self._align(current_address, alignment)
        padding = aligned_address - current_address

        total_size = padding + size

        # Check if we have enough space
        if self.top + total_size > self.capacity:
            available = self.capacity - self.top
            print(
                f"[{self.name}] Stack overflow! "
                f"Requested: {size} bytes (+{padding} padding), "
                f"Available: {available} bytes"
            )
            return None

        # Allocate
        allocation = StackAllocation(aligned_address, size, padding, self)
        self.top += total_size

        # Track this allocation
        self.allocation_stack.append(aligned_address)

        # Update peak usage
        if self.top > self.peak_usage:
            self.peak_usage = self.top

        self._track_allocation(size)

        return allocation

    def free(self, ptr: StackAllocation) -> bool:
        """
        Free allocation from stack
        Must be called in LIFO order!

        Args:
            ptr: StackAllocation to free

        Returns:
            True if successful, False if LIFO order violated
        """
        if ptr is None:
            return False

        if not isinstance(ptr, StackAllocation):
            print(f"[{self.name}] Invalid pointer type: {type(ptr)}")
            return False

        if not self.allocation_stack:
            print(f"[{self.name}] Stack underflow! No allocations to free.")
            return False

        # Check LIFO order
        expected_offset = self.allocation_stack[-1]
        if ptr.offset != expected_offset:
            print(
                f"[{self.name}] LIFO order violation! "
                f"Expected to free offset {expected_offset}, "
                f"but got {ptr.offset}"
            )
            return False

        # Pop from stack
        self.allocation_stack.pop()
        total_size = ptr.get_total_size()
        self.top -= total_size

        self._track_free(ptr.size)

        return True

    def free_to_marker(self, marker: int) -> bool:
        """
        Free all allocations back to a specific marker
        Useful for scope-based allocation patterns
        """
        if marker < 0 or marker > self.top:
            print(f"[{self.name}] Invalid marker: {marker}")
            return False

        # Pop all allocations until we reach the marker
        while self.allocation_stack and self.allocation_stack[-1] >= marker:
            self.allocation_stack.pop()

        freed_bytes = self.top - marker
        self.top = marker

        if freed_bytes > 0:
            self._track_free(freed_bytes)

        return True

    def reset(self) -> None:
        """Reset stack to initial state - frees all allocations"""
        self.top = 0
        self.allocation_stack.clear()
        self._track_reset()

    def get_capacity(self) -> int:
        """Get total capacity in bytes"""
        return self.capacity

    def get_used(self) -> int:
        """Get currently used bytes"""
        return self.top

    def get_peak_usage(self) -> int:
        """Get peak memory usage (high water mark)"""
        return self.peak_usage

    def reset_peak(self) -> None:
        """Reset peak usage counter"""
        self.peak_usage = self.top

    def get_marker(self) -> int:
        """
        Get current stack position as a marker
        Can be used with free_to_marker() for scoped allocations
        """
        return self.top

    def get_allocation_count(self) -> int:
        """Get number of active allocations"""
        return len(self.allocation_stack)

    def _align(self, offset: int, alignment: int) -> int:
        """Align offset to specified boundary"""
        return ((offset + alignment - 1) // alignment) * alignment

    def __repr__(self) -> str:
        return (
            f"StackAllocator(name='{self.name}', "
            f"capacity={self.capacity:,}, "
            f"used={self.top:,}, "
            f"peak={self.peak_usage:,}, "
            f"allocations={len(self.allocation_stack)}, "
            f"utilization={self.get_utilization()*100:.1f}%)"
        )


class ScopedStack:
    """
    RAII-style scoped stack allocation
    Automatically frees to marker when scope exits

    Usage:
        stack = StackAllocator(1024)

        with ScopedStack(stack) as scoped:
            temp1 = stack.allocate(128)
            temp2 = stack.allocate(256)
            # ... use temp allocations ...
        # All allocations freed here automatically
    """

    def __init__(self, stack: StackAllocator):
        self.stack = stack
        self.marker = 0

    def __enter__(self) -> "ScopedStack":
        self.marker = self.stack.get_marker()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.stack.free_to_marker(self.marker)
        return False
