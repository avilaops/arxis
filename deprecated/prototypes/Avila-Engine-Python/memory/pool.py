"""
Pool Allocator - Fixed-size block allocation
Ideal for objects of the same size (particles, bullets, enemies, etc.)
"""

from typing import Optional, List, Any
from .allocator import Allocator


class PoolBlock:
    """Individual block in the pool"""

    __slots__ = ["data", "next_free", "is_free", "index"]

    def __init__(self, size: int, index: int):
        self.data = bytearray(size)
        self.next_free: Optional[int] = None
        self.is_free = True
        self.index = index


class PoolAllocator(Allocator):
    """
    Pool Allocator - Pre-allocates fixed-size blocks

    Best for:
    - Same-sized objects (particles, entities, pooled objects)
    - Frequent allocation/deallocation
    - Avoiding fragmentation
    - Cache-friendly access patterns

    Characteristics:
    - O(1) allocation and deallocation
    - No fragmentation
    - Fixed block size
    - Memory overhead: one pointer per block
    """

    def __init__(
        self,
        block_size: int,
        block_count: int,
        alignment: int = 8,
        name: str = "PoolAllocator",
    ):
        """
        Create a pool allocator

        Args:
            block_size: Size of each block in bytes
            block_count: Number of blocks to pre-allocate
            alignment: Memory alignment (default 8 bytes)
            name: Allocator name for debugging
        """
        super().__init__(name)

        if block_size <= 0:
            raise ValueError("Block size must be positive")
        if block_count <= 0:
            raise ValueError("Block count must be positive")

        # Align block size
        self.block_size = self._align(block_size, alignment)
        self.block_count = block_count
        self.alignment = alignment
        self.capacity = self.block_size * self.block_count

        # Create all blocks
        self.blocks: List[PoolBlock] = []
        for i in range(block_count):
            self.blocks.append(PoolBlock(self.block_size, i))

        # Initialize free list (linked list of free blocks)
        self.first_free: Optional[int] = 0
        for i in range(block_count - 1):
            self.blocks[i].next_free = i + 1
        self.blocks[block_count - 1].next_free = None

        self.used_blocks = 0

    def _align(self, size: int, alignment: int) -> int:
        """Align size to specified boundary"""
        return ((size + alignment - 1) // alignment) * alignment

    def allocate(self, size: int, alignment: int = 8) -> Optional[PoolBlock]:
        """
        Allocate a block from the pool

        Args:
            size: Requested size (must be <= block_size)
            alignment: Alignment (ignored, uses pool alignment)

        Returns:
            PoolBlock or None if pool is full or size too large
        """
        if not self._enabled:
            return None

        if size > self.block_size:
            print(
                f"[{self.name}] Requested size {size} exceeds block size {self.block_size}"
            )
            return None

        if self.first_free is None:
            print(
                f"[{self.name}] Pool exhausted! All {self.block_count} blocks allocated."
            )
            return None

        # Get first free block
        block_index = self.first_free
        block = self.blocks[block_index]

        # Update free list
        self.first_free = block.next_free
        block.next_free = None
        block.is_free = False

        self.used_blocks += 1
        self._track_allocation(self.block_size)

        return block

    def free(self, ptr: PoolBlock) -> bool:
        """
        Free a block back to the pool

        Args:
            ptr: PoolBlock to free

        Returns:
            True if successful, False otherwise
        """
        if ptr is None:
            return False

        if not isinstance(ptr, PoolBlock):
            print(f"[{self.name}] Invalid pointer type: {type(ptr)}")
            return False

        if ptr.is_free:
            print(f"[{self.name}] Double free detected for block {ptr.index}!")
            return False

        # Add to free list
        ptr.next_free = self.first_free
        ptr.is_free = True
        self.first_free = ptr.index

        self.used_blocks -= 1
        self._track_free(self.block_size)

        return True

    def reset(self) -> None:
        """Reset pool - mark all blocks as free"""
        self.first_free = 0
        for i in range(self.block_count - 1):
            self.blocks[i].next_free = i + 1
            self.blocks[i].is_free = True
        self.blocks[self.block_count - 1].next_free = None
        self.blocks[self.block_count - 1].is_free = True

        self.used_blocks = 0
        self._track_reset()

    def get_capacity(self) -> int:
        """Get total capacity in bytes"""
        return self.capacity

    def get_used(self) -> int:
        """Get currently used bytes"""
        return self.used_blocks * self.block_size

    def get_free_blocks(self) -> int:
        """Get number of free blocks"""
        return self.block_count - self.used_blocks

    def get_used_blocks(self) -> int:
        """Get number of used blocks"""
        return self.used_blocks

    def is_full(self) -> bool:
        """Check if pool is full"""
        return self.first_free is None

    def is_empty(self) -> bool:
        """Check if pool is empty (all blocks free)"""
        return self.used_blocks == 0

    def get_block_info(self, block: PoolBlock) -> dict:
        """Get information about a specific block"""
        if not isinstance(block, PoolBlock):
            return {}

        return {
            "index": block.index,
            "is_free": block.is_free,
            "size": self.block_size,
            "data_length": len(block.data),
        }

    def __repr__(self) -> str:
        return (
            f"PoolAllocator(name='{self.name}', "
            f"block_size={self.block_size}, "
            f"blocks={self.used_blocks}/{self.block_count}, "
            f"utilization={self.get_utilization()*100:.1f}%)"
        )
