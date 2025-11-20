"""
Base Allocator interface and statistics
"""

from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Any, Optional
import time


@dataclass
class AllocationStats:
    """Statistics for memory allocation tracking"""

    total_allocated: int = 0  # Total bytes allocated
    total_freed: int = 0  # Total bytes freed
    current_usage: int = 0  # Current memory usage
    peak_usage: int = 0  # Peak memory usage
    num_allocations: int = 0  # Number of allocations
    num_frees: int = 0  # Number of frees
    num_resets: int = 0  # Number of resets (for arena/stack)
    allocation_time: float = 0.0  # Total time spent allocating
    free_time: float = 0.0  # Total time spent freeing

    def fragmentation_ratio(self) -> float:
        """Calculate fragmentation (wasted space)"""
        if self.total_allocated == 0:
            return 0.0
        return 1.0 - (self.current_usage / self.total_allocated)

    def average_allocation_size(self) -> float:
        """Average size of allocations"""
        if self.num_allocations == 0:
            return 0.0
        return self.total_allocated / self.num_allocations

    def __str__(self) -> str:
        return (
            f"AllocationStats:\n"
            f"  Total Allocated: {self.total_allocated:,} bytes\n"
            f"  Current Usage: {self.current_usage:,} bytes\n"
            f"  Peak Usage: {self.peak_usage:,} bytes\n"
            f"  Allocations: {self.num_allocations:,}\n"
            f"  Frees: {self.num_frees:,}\n"
            f"  Avg Alloc Size: {self.average_allocation_size():.2f} bytes\n"
            f"  Fragmentation: {self.fragmentation_ratio()*100:.2f}%"
        )


class Allocator(ABC):
    """Base allocator interface"""

    def __init__(self, name: str = "Allocator"):
        self.name = name
        self.stats = AllocationStats()
        self._enabled = True

    @abstractmethod
    def allocate(self, size: int, alignment: int = 8) -> Optional[Any]:
        """
        Allocate memory of given size with specified alignment
        Returns handle/pointer to allocated memory or None if failed
        """
        pass

    @abstractmethod
    def free(self, ptr: Any) -> bool:
        """
        Free previously allocated memory
        Returns True if successful, False otherwise
        """
        pass

    @abstractmethod
    def reset(self) -> None:
        """Reset allocator to initial state (if supported)"""
        pass

    @abstractmethod
    def get_capacity(self) -> int:
        """Get total capacity in bytes"""
        pass

    @abstractmethod
    def get_used(self) -> int:
        """Get currently used bytes"""
        pass

    def get_free(self) -> int:
        """Get available bytes"""
        return self.get_capacity() - self.get_used()

    def get_utilization(self) -> float:
        """Get utilization percentage (0.0 to 1.0)"""
        capacity = self.get_capacity()
        if capacity == 0:
            return 0.0
        return self.get_used() / capacity

    def is_enabled(self) -> bool:
        """Check if allocator is enabled"""
        return self._enabled

    def enable(self) -> None:
        """Enable allocator"""
        self._enabled = True

    def disable(self) -> None:
        """Disable allocator"""
        self._enabled = False

    def get_stats(self) -> AllocationStats:
        """Get allocation statistics"""
        return self.stats

    def reset_stats(self) -> None:
        """Reset statistics"""
        self.stats = AllocationStats()

    def _track_allocation(self, size: int) -> None:
        """Internal: Track allocation in stats"""
        start_time = time.perf_counter()
        self.stats.total_allocated += size
        self.stats.current_usage += size
        self.stats.num_allocations += 1
        if self.stats.current_usage > self.stats.peak_usage:
            self.stats.peak_usage = self.stats.current_usage
        self.stats.allocation_time += time.perf_counter() - start_time

    def _track_free(self, size: int) -> None:
        """Internal: Track free in stats"""
        start_time = time.perf_counter()
        self.stats.total_freed += size
        self.stats.current_usage -= size
        self.stats.num_frees += 1
        self.stats.free_time += time.perf_counter() - start_time

    def _track_reset(self) -> None:
        """Internal: Track reset in stats"""
        self.stats.num_resets += 1
        self.stats.current_usage = 0

    def __repr__(self) -> str:
        return (
            f"{self.__class__.__name__}(name='{self.name}', "
            f"capacity={self.get_capacity():,}, "
            f"used={self.get_used():,}, "
            f"utilization={self.get_utilization()*100:.1f}%)"
        )
