//! Parallel iterator traits and implementations

use crate::thread_pool::global_pool;
use std::sync::Mutex;

/// Trait for parallel iterators
pub trait ParallelIterator: Sized {
    /// The type of item being iterated
    type Item: Send;

    /// Execute a function on each item in parallel
    fn for_each<F>(self, f: F)
    where
        F: Fn(Self::Item) + Send + Sync;

    /// Map each item in parallel
    fn map<F, R>(self, f: F) -> Map<Self, F>
    where
        F: Fn(Self::Item) -> R + Send + Sync,
        R: Send;

    /// Filter items in parallel
    fn filter<F>(self, f: F) -> Filter<Self, F>
    where
        F: Fn(&Self::Item) -> bool + Send + Sync;

    /// Collect into a collection
    fn collect<C>(self) -> C
    where
        C: FromParallelIterator<Self::Item>;

    /// Sum all items
    fn sum<S>(self) -> S
    where
        S: Send + std::iter::Sum<Self::Item>;
}

/// Trait for types that can be converted into parallel iterators
pub trait IntoParallelIterator {
    /// The parallel iterator type
    type Iter: ParallelIterator<Item = Self::Item>;
    /// The item type
    type Item: Send;

    /// Convert into a parallel iterator
    fn into_par_iter(self) -> Self::Iter;
}

/// Extension trait for Vec and slices
pub trait ParallelSlice<T: Sync> {
    /// Create a parallel iterator over shared references
    fn par_iter(&self) -> ParIter<'_, T>;
}

impl<T: Sync> ParallelSlice<T> for Vec<T> {
    fn par_iter(&self) -> ParIter<'_, T> {
        ParIter { slice: self }
    }
}

impl<T: Sync> ParallelSlice<T> for [T] {
    fn par_iter(&self) -> ParIter<'_, T> {
        ParIter { slice: self }
    }
}

/// Parallel iterator over slice references
pub struct ParIter<'a, T: Sync> {
    slice: &'a [T],
}

impl<'a, T: Sync + 'a> ParallelIterator for ParIter<'a, T> {
    type Item = &'a T;

    fn for_each<F>(self, f: F)
    where
        F: Fn(Self::Item) + Send + Sync,
    {
        let pool = global_pool();
        let f = &f;

        for item in self.slice {
            pool.execute(move || {
                f(item);
            });
        }

        pool.wait();
    }

    fn map<F, R>(self, f: F) -> Map<Self, F>
    where
        F: Fn(Self::Item) -> R + Send + Sync,
        R: Send,
    {
        Map { iter: self, f }
    }

    fn filter<F>(self, f: F) -> Filter<Self, F>
    where
        F: Fn(&Self::Item) -> bool + Send + Sync,
    {
        Filter { iter: self, f }
    }

    fn collect<C>(self) -> C
    where
        C: FromParallelIterator<Self::Item>,
    {
        C::from_par_iter(self)
    }

    fn sum<S>(self) -> S
    where
        S: Send + std::iter::Sum<Self::Item>,
    {
        // Simple implementation - in production would use work stealing
        self.slice.iter().copied().sum()
    }
}

/// Map adapter for parallel iterators
pub struct Map<I, F> {
    iter: I,
    f: F,
}

impl<I, F, R> ParallelIterator for Map<I, F>
where
    I: ParallelIterator,
    F: Fn(I::Item) -> R + Send + Sync,
    R: Send,
{
    type Item = R;

    fn for_each<G>(self, g: G)
    where
        G: Fn(Self::Item) + Send + Sync,
    {
        let f = &self.f;
        let g = &g;
        self.iter.for_each(move |item| {
            g(f(item));
        });
    }

    fn map<G, S>(self, g: G) -> Map<Self, G>
    where
        G: Fn(Self::Item) -> S + Send + Sync,
        S: Send,
    {
        Map { iter: self, f: g }
    }

    fn filter<G>(self, g: G) -> Filter<Self, G>
    where
        G: Fn(&Self::Item) -> bool + Send + Sync,
    {
        Filter { iter: self, f: g }
    }

    fn collect<C>(self) -> C
    where
        C: FromParallelIterator<Self::Item>,
    {
        C::from_par_iter(self)
    }

    fn sum<S>(self) -> S
    where
        S: Send + std::iter::Sum<Self::Item>,
    {
        // Stub implementation
        unimplemented!("Sum for Map not yet implemented")
    }
}

/// Filter adapter for parallel iterators
pub struct Filter<I, F> {
    iter: I,
    f: F,
}

impl<I, F> ParallelIterator for Filter<I, F>
where
    I: ParallelIterator,
    F: Fn(&I::Item) -> bool + Send + Sync,
{
    type Item = I::Item;

    fn for_each<G>(self, g: G)
    where
        G: Fn(Self::Item) + Send + Sync,
    {
        let f = &self.f;
        let g = &g;
        self.iter.for_each(move |item| {
            if f(&item) {
                g(item);
            }
        });
    }

    fn map<G, R>(self, g: G) -> Map<Self, G>
    where
        G: Fn(Self::Item) -> R + Send + Sync,
        R: Send,
    {
        Map { iter: self, f: g }
    }

    fn filter<G>(self, g: G) -> Filter<Self, G>
    where
        G: Fn(&Self::Item) -> bool + Send + Sync,
    {
        Filter { iter: self, f: g }
    }

    fn collect<C>(self) -> C
    where
        C: FromParallelIterator<Self::Item>,
    {
        C::from_par_iter(self)
    }

    fn sum<S>(self) -> S
    where
        S: Send + std::iter::Sum<Self::Item>,
    {
        unimplemented!("Sum for Filter not yet implemented")
    }
}

/// Trait for collecting from parallel iterators
pub trait FromParallelIterator<T>: Sized {
    /// Create from parallel iterator
    fn from_par_iter<I>(iter: I) -> Self
    where
        I: ParallelIterator<Item = T>;
}

impl<T: Send> FromParallelIterator<T> for Vec<T> {
    fn from_par_iter<I>(iter: I) -> Self
    where
        I: ParallelIterator<Item = T>,
    {
        let results = Mutex::new(Vec::new());
        iter.for_each(|item| {
            results.lock().unwrap().push(item);
        });
        results.into_inner().unwrap()
    }
}
