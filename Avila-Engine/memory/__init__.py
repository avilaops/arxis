"""
Avila Engine Memory Management System
Custom allocators, pools, and arenas for efficient memory management
"""

from .allocator import Allocator, AllocationStats
from .pool import PoolAllocator
from .arena import ArenaAllocator
from .stack import StackAllocator
from .manager import MemoryManager

__all__ = [
    "Allocator",
    "AllocationStats",
    "PoolAllocator",
    "ArenaAllocator",
    "StackAllocator",
    "MemoryManager",
]

__version__ = "0.1.0"
