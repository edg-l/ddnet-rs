use std::{cell::RefCell, marker::PhantomData, mem::ManuallyDrop, num::NonZeroUsize, rc::Rc};

use crate::{recycle::Recycle, traits::Recyclable};

#[cfg_attr(feature = "enable_hiarc", derive(hiarc::Hiarc))]
#[derive(Debug)]
pub(crate) struct PoolInner<T: Recyclable> {
    pool: RefCell<Vec<T>>,
    max_items: Option<NonZeroUsize>,
}

impl<T: Recyclable> PoolInner<T> {
    pub(crate) fn push(&self, item: T) {
        let mut pool = self.pool.borrow_mut();
        pool.push(item);
        if let Some(max_items) = self.max_items {
            pool.truncate(max_items.get());
        }
    }

    pub(crate) fn get(&self) -> T {
        self.pool.borrow_mut().pop().unwrap_or_else(|| T::new())
    }

    pub(crate) fn append(&self, other: &mut Vec<T>) {
        let mut pool = self.pool.borrow_mut();
        pool.append(other);
        if let Some(max_items) = self.max_items {
            pool.truncate(max_items.get());
        }
    }
}

/// Allows for a controlled allocation of the pool
#[cfg_attr(feature = "enable_hiarc", derive(hiarc::Hiarc))]
#[derive(Debug)]
pub struct PoolBuilder<T: Recyclable> {
    capacity: Option<usize>,
    max_items: Option<NonZeroUsize>,

    p: PhantomData<T>,
}

impl<T: Recyclable> Default for PoolBuilder<T> {
    fn default() -> Self {
        Self {
            capacity: Default::default(),
            max_items: Default::default(),
            p: Default::default(),
        }
    }
}

impl<T: Recyclable> PoolBuilder<T> {
    pub fn build(self) -> Pool<T> {
        let pool = if let Some(capacity) = self.capacity {
            Vec::with_capacity(capacity)
        } else {
            Vec::new()
        };

        Pool {
            pool: Rc::new(PoolInner {
                pool: RefCell::new(pool),
                max_items: self.max_items,
            }),
        }
    }

    pub fn build_sized<F>(self, new_size: usize, item_constructor: F) -> Pool<T>
    where
        F: FnMut() -> T,
    {
        let mut pool = if let Some(capacity) = self.capacity {
            Vec::with_capacity(capacity.max(new_size))
        } else {
            Vec::with_capacity(new_size)
        };
        pool.resize_with(new_size, item_constructor);
        Pool {
            pool: Rc::new(PoolInner {
                pool: RefCell::new(pool),
                max_items: self.max_items,
            }),
        }
    }

    /// If capacity is 0, the pool will not allocate memory for any elements, but will still create heap memory.
    pub fn with_capacity(self, capacity: usize) -> Self {
        Self {
            capacity: Some(capacity),
            ..self
        }
    }

    /// If the limit is reached, then recycled items will not be pushed into the pool again.
    pub fn with_limit(self, max_items: NonZeroUsize) -> Self {
        Self {
            max_items: Some(max_items),
            ..self
        }
    }
}

// No crate fulfilled our requirements => so own implementation.
/// We want a pool with elements where T is trivially creatable,
/// so that we can store the whole object and pool as object
/// with automatic cleanup, no lifetimes etc.
///
/// Additionally it supports having no pool to recycle to.
#[cfg_attr(feature = "enable_hiarc", derive(hiarc::Hiarc))]
#[derive(Debug)]
pub struct Pool<T: Recyclable> {
    pub(crate) pool: Rc<PoolInner<T>>,
}

impl<T: Recyclable> Pool<T> {
    pub fn builder() -> PoolBuilder<T> {
        PoolBuilder::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            pool: Rc::new(PoolInner {
                pool: RefCell::new(Vec::with_capacity(capacity)),
                max_items: None,
            }),
        }
    }

    pub fn with_sized<F>(new_size: usize, item_constructor: F) -> Self
    where
        F: FnMut() -> T,
    {
        let mut pool = Vec::with_capacity(new_size);
        pool.resize_with(new_size, item_constructor);
        Self {
            pool: Rc::new(PoolInner {
                pool: RefCell::new(pool),
                max_items: None,
            }),
        }
    }

    pub fn new(&self) -> Recycle<T> {
        let mut pool = self.pool.pool.borrow_mut();
        if let Some(item) = pool.pop() {
            Recycle {
                pool: Some(self.pool.clone()),
                item: ManuallyDrop::new(item),
            }
        } else {
            Recycle {
                pool: Some(self.pool.clone()),
                item: ManuallyDrop::new(T::new()),
            }
        }
    }

    pub fn items_in_pool(&self) -> usize {
        self.pool.pool.borrow().len()
    }
}

impl<T: Recyclable> Clone for Pool<T> {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}
