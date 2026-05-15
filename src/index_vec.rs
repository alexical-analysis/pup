use std::fmt::Debug;
use std::ops::Index;
use std::ops::IndexMut;

/// A strongly-typed index into an `IndexVec`.
///
/// Implement this on a newtype wrapper around `u32` (or `usize`).
/// Use the `define_index!` macro for the boilerplate.
pub trait Idx: Copy + Eq + Debug {
    fn new(idx: usize) -> Self;
    fn index(self) -> usize;
}

/// A `Vec<V>` whose elements are accessed by a strongly-typed index `I`.
///
/// Prevents accidentally indexing a `Vec<BasicBlock>` with a `LocalId`, etc.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IndexVec<I: Idx, V> {
    raw: Vec<V>,
    _marker: std::marker::PhantomData<fn(&I)>,
}

impl<I: Idx, V> IndexVec<I, V> {
    /// Create an empty `IndexVec`.
    pub fn new() -> Self {
        Self {
            raw: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Create with a pre-allocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            raw: Vec::with_capacity(capacity),
            _marker: std::marker::PhantomData,
        }
    }

    /// Push a value and return its typed index.
    pub fn push(&mut self, value: V) -> I {
        let idx = I::new(self.raw.len());
        self.raw.push(value);
        idx
    }

    /// Number of elements.
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    /// Returns `true` if the vec contains no elements.
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    /// Iterate over `(I, &V)` pairs.
    pub fn iter_enumerated(&self) -> impl Iterator<Item = (I, &V)> {
        self.raw.iter().enumerate().map(|(i, v)| (I::new(i), v))
    }

    /// Iterate over `(I, &mut V)` pairs.
    pub fn iter_enumerated_mut(&mut self) -> impl Iterator<Item = (I, &mut V)> {
        self.raw.iter_mut().enumerate().map(|(i, v)| (I::new(i), v))
    }

    /// Returns the index that the *next* pushed element would receive,
    /// without actually pushing anything. Useful for forward-declaring a
    /// block ID before filling it in.
    pub fn next_idx(&self) -> I {
        I::new(self.raw.len())
    }

    /// Get a reference by typed index, returning `None` if out of bounds.
    pub fn get(&self, index: I) -> Option<&V> {
        self.raw.get(index.index())
    }

    /// Get a mutable reference by typed index, returning `None` if out of bounds.
    pub fn get_mut(&mut self, index: I) -> Option<&mut V> {
        self.raw.get_mut(index.index())
    }

    /// Iterate over all values.
    pub fn iter(&self) -> std::slice::Iter<'_, V> {
        self.raw.iter()
    }

    /// Iterate over all values mutably.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, V> {
        self.raw.iter_mut()
    }

    /// Consume the `IndexVec` and return the underlying `Vec`.
    pub fn into_raw(self) -> Vec<V> {
        self.raw
    }

    /// View the underlying slice.
    pub fn as_slice(&self) -> &[V] {
        &self.raw
    }
}

impl<I: Idx, V> Default for IndexVec<I, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: Idx, V> Index<I> for IndexVec<I, V> {
    type Output = V;

    fn index(&self, index: I) -> &Self::Output {
        &self.raw[index.index()]
    }
}

impl<I: Idx, V> IndexMut<I> for IndexVec<I, V> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.raw[index.index()]
    }
}

impl<I: Idx, V> IntoIterator for IndexVec<I, V> {
    type Item = V;
    type IntoIter = std::vec::IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.into_iter()
    }
}

impl<'a, I: Idx, V> IntoIterator for &'a IndexVec<I, V> {
    type Item = &'a V;
    type IntoIter = std::slice::Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.iter()
    }
}

impl<'a, I: Idx, V> IntoIterator for &'a mut IndexVec<I, V> {
    type Item = &'a mut V;
    type IntoIter = std::slice::IterMut<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.iter_mut()
    }
}

impl<I: Idx, V> FromIterator<V> for IndexVec<I, V> {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Self {
            raw: iter.into_iter().collect(),
            _marker: std::marker::PhantomData,
        }
    }
}

/// Macro to define a typed index newtype with minimal boilerplate.
///
/// # Usage
///
/// ```
/// define_index!(pub BlockId);
/// define_index!(pub LocalId);
/// define_index!(ValueId);           // private
/// define_index!(pub NodeId: u32);   // explicit repr (u32 is the default)
/// ```
///
/// Each generated type:
/// - Is a `#[repr(transparent)]` newtype around `u32`.
/// - Implements `Idx`, `Copy`, `Clone`, `Debug`, `PartialEq`, `Eq`,
///   `PartialOrd`, `Ord`, `Hash`.
/// - Has a `const fn from_u32(u32) -> Self` and `fn as_u32(self) -> u32`
///   for when you need the raw value (e.g. storing in a tagged union or
///   emitting LLVM debug info).
#[macro_export]
macro_rules! define_index {
    // pub with explicit repr
    (pub $name:ident : $repr:ty) => {
        $crate::__define_index_inner!(pub, $name, $repr);
    };
    // private with explicit repr
    ($name:ident : $repr:ty) => {
        $crate::__define_index_inner!(, $name, $repr);
    };
    // pub with default repr (u32)
    (pub $name:ident) => {
        $crate::__define_index_inner!(pub, $name, u32);
    };
    // private with default repr (u32)
    ($name:ident) => {
        $crate::__define_index_inner!(, $name, u32);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_index_inner {
    ($vis:vis, $name:ident, $repr:ty) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        $vis struct $name($repr);

        impl $name {
            #[inline]
            pub const fn from_raw(raw: $repr) -> Self {
                Self(raw)
            }

            #[inline]
            pub fn as_raw(self) -> $repr {
                self.0
            }
        }

        impl $crate::index_vec::Idx for $name {
            #[inline]
            fn new(idx: usize) -> Self {
                assert!(
                    idx < (<$repr>::MAX as usize),
                    concat!(stringify!($name), " index overflow"),
                );
                Self(idx as $repr)
            }

            #[inline]
            fn index(self) -> usize {
                self.0 as usize
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    define_index!(pub TestId);

    #[test]
    fn push_and_index() {
        let mut v: IndexVec<TestId, &str> = IndexVec::new();
        let a = v.push("alpha");
        let b = v.push("beta");
        let c = v.push("gamma");

        assert_eq!(v[a], "alpha");
        assert_eq!(v[b], "beta");
        assert_eq!(v[c], "gamma");
    }

    #[test]
    fn next_idx_does_not_push() {
        let mut v: IndexVec<TestId, i32> = IndexVec::new();
        let expected = v.next_idx();
        let actual = v.push(42);
        assert_eq!(expected, actual);
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn iter_enumerated() {
        let mut v: IndexVec<TestId, i32> = IndexVec::new();
        let ids: Vec<TestId> = (0..5).map(|i| v.push(i * 10)).collect();

        for (id, &val) in v.iter_enumerated() {
            assert_eq!(val, ids[id.index()].index() as i32 * 10);
        }
    }

    #[test]
    fn from_iter() {
        let v: IndexVec<TestId, i32> = (0..4).collect();
        assert_eq!(v.len(), 4);
        assert_eq!(v[TestId::from_raw(2)], 2);
    }

    #[test]
    fn get_out_of_bounds_returns_none() {
        let v: IndexVec<TestId, i32> = IndexVec::new();
        assert!(v.get(TestId::from_raw(0)).is_none());
    }

    #[test]
    fn type_safety_at_compile_time() {
        // This test exists to document the compile-time guarantee.
        // If you uncomment the lines below they will fail to compile —
        // BlockId cannot index a Vec<LocalId> and vice versa.
        define_index!(pub BlockId2);
        define_index!(pub LocalId2);

        let mut blocks: IndexVec<BlockId2, &str> = IndexVec::new();
        let b = blocks.push("entry");
        let _: &str = &blocks[b]; // fine

        // let _local = LocalId2::from_raw(0);
        // let _ = &blocks[_local]; // compile error: expected BlockId2, found LocalId2
        let _ = b;
    }
}
