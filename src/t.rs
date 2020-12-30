//! Central map trait to ease modifications and extensions down the road.

use crate::iter::{Iter, IterMut};
use crate::lock::{RwLockReadGuard, RwLockWriteGuard};
use crate::mapref::entry::Entry;
use crate::mapref::one::{Ref, RefMut};
use crate::HashMap;
use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};

/// Implementation detail that is exposed due to generic constraints in public types.
pub trait Map<'a, K: 'a + Eq + Hash, V: 'a, S: 'a + Clone + BuildHasher> {
    fn _shard_count(&self) -> usize;

    /// # Safety
    ///
    /// The index must not be out of bounds.
    unsafe fn _get_read_shard(&'a self, i: usize) -> &'a HashMap<K, V, S>;

    /// # Safety
    ///
    /// The index must not be out of bounds.
    unsafe fn _yield_read_shard(&'a self, i: usize) -> RwLockReadGuard<'a, HashMap<K, V, S>>;

    /// # Safety
    ///
    /// The index must not be out of bounds.
    unsafe fn _yield_write_shard(&'a self, i: usize) -> RwLockWriteGuard<'a, HashMap<K, V, S>>;

    fn _insert(&self, key: K, value: V) -> Option<V>;

    fn _insert_with<T, E>(
        &self,
        key: K,
        value: V,
        f: impl FnOnce() -> Result<T, E>,
    ) -> (Option<V>, Result<T, E>);

    fn _insert_and_post_process<T1, E1, T2, E2, T3, E3>(
        &self,
        key: K,
        value: V,
        key_exists_func: impl FnOnce(&V) -> Result<T1, E1>,
        not_exists_func: impl FnOnce() -> Result<T2, E2>,
        post_func: Option<impl FnOnce() -> Result<T3, E3>>,
    ) -> (
        Option<V>,
        Option<Result<T1, E1>>,
        Option<Result<T2, E2>>,
        Option<Result<T3, E3>>,
    );

    fn _remove<Q>(&self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;

    fn _remove_if<Q>(&self, key: &Q, f: impl FnOnce(&K, &V) -> bool) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;

    fn _remove_and_post_process<Q, T1, E1, T2, E2>(
        &self,
        key: &Q,
        key_exists_func: impl FnOnce(&Q, &V) -> Result<T1, E1>,
        not_exists_func: Option<impl FnOnce() -> Result<T2, E2>>,
    ) -> (
        Option<(K, V)>,
        Option<Result<T1, E1>>,
        Option<Result<T2, E2>>,
    )
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;

    fn _iter(&'a self) -> Iter<'a, K, V, S, Self>
    where
        Self: Sized;

    fn _iter_mut(&'a self) -> IterMut<'a, K, V, S, Self>
    where
        Self: Sized;

    fn _get<Q>(&'a self, key: &Q) -> Option<Ref<'a, K, V, S>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;

    fn _get_with<Q, T, E>(
        &'a self,
        key: &Q,
        post_func: impl FnOnce() -> Result<T, E>,
    ) -> (Option<Ref<'a, K, V, S>>, Result<T, E>)
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;

    fn _get_and_post_process<Q, T, E>(
        &'a self,
        key: &Q,
        key_exists_func: impl FnOnce(&V) -> Result<T, E>,
        not_exists_func: impl FnOnce() -> Result<T, E>,
    ) -> (Option<Ref<'a, K, V, S>>, Result<T, E>)
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;

    fn _get_mut<Q>(&'a self, key: &Q) -> Option<RefMut<'a, K, V, S>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;

    fn _shrink_to_fit(&self);

    fn _retain(&self, f: impl FnMut(&K, &mut V) -> bool);

    fn _len(&self) -> usize;

    fn _capacity(&self) -> usize;

    fn _alter<Q>(&self, key: &Q, f: impl FnOnce(&K, V) -> V)
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;

    fn _alter_all(&self, f: impl FnMut(&K, V) -> V);

    fn _entry(&'a self, key: K) -> Entry<'a, K, V, S>;

    fn _hasher(&self) -> S;

    // provided
    fn _clear(&self) {
        self._retain(|_, _| false)
    }

    fn _contains_key<Q>(&'a self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self._get(key).is_some()
    }

    fn _is_empty(&self) -> bool {
        self._len() == 0
    }
}
