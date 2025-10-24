//! Specialized methods for `Cell` of some specific `!Copy` types,
//! allowing limited access to a value without moving it of the cell.
//!
//! This module contains three extension traits grouped together for cohesion:
//! - `CellOption`
//! - `CellOptionWeak<T>`
//! - `CellOptionRc<T>`
//!
//! These traits are kept in a single file as they are closely related extension
//! traits for Cell operations, following the project's guideline that allows
//! logical grouping of related items.
//!
//!
//! # Soundness
//!
//! These methods use and `Cell::as_ptr` and `unsafe`.
//! Their soundness lies in that:
//!
//! * `Cell<T>: !Sync` for any `T`, so no other thread is accessing this cell.
//! * For the duration of the raw pointer access,
//!   this thread only runs code that is known to not access the same cell again.
//!   In particular, no method of a type paramater is called.
//!   For example, `clone_inner` would be unsound to generalize to any `Cell<T>`
//!   because it would involve running arbitrary code through `T::clone`
//!   and provide that code with a reference to the inside of the cell.
//!
//! ```rust,compile_fail
//! # use std::cell::Cell;
//! # use std::mem;
//! # use std::rc::Rc;
//! struct Evil(Box<u32>, Rc<Cell<Option<Evil>>>);
//! impl Clone for Evil {
//!     fn clone(&self) -> Self {
//!         mem::drop(self.1.take());  // Mess with the "other" node, which might be `self`.
//!         Evil(
//!             self.0.clone(),  // possible use after free!
//!             Rc::new(Cell::new(None))
//!         )
//!     }
//! }
//! let a = Rc::new(Cell::new(None));
//! a.set(Some(Evil(Box::new(5), a.clone())));  // Make a reference cycle.
//! a.clone_inner();
//! ```
//!
//! `Rc<T>::clone` and `Weak<T>::clone` do not have this problem
//! as they only increment reference counts and never call `T::clone`.
//!
//!
//! # Alternative
//!
//! To avoid using `unsafe` entirely, operating on a `T: !Copy` value inside a `Cell<T>`
//! would require temporarily replacing it with a default value:
//!
//! ```rust
//! # use std::cell::Cell;
//! fn option_dance<T, F, R>(cell: &Cell<T>, f: F) -> R
//!     where T: Default, F: FnOnce(&mut T) -> R
//! {
//!     let mut value = cell.take();
//!     let result = f(&mut value);
//!     cell.set(value);
//!     result
//! }
//! ```
//!
//! It would be worth exploring whether LLVM can reliably optimize away these extra moves
//! and compile the `Option` dance to assembly similar to that of the `unsafe` operation.

use std::cell::Cell;
use std::rc::{Rc, Weak};

/// Extension trait for `Cell<Option<T>>` to check if the option is None without moving the value.
pub trait CellOption {
    /// Check if the Cell contains None without taking the value out.
    fn is_none(&self) -> bool;
}

/// Implements CellOption for Cell<Option<T>>.
///
/// Provides efficient checking of Option contents without moving the value
/// out of the cell. Uses unsafe pointer access by default for performance,
/// with a safe fallback when the "safe" feature is enabled.
impl<T> CellOption for Cell<Option<T>> {
    #[inline]
    fn is_none(&self) -> bool {
        #[cfg(not(feature = "safe"))]
        #[allow(unsafe_code)]
        {
            unsafe { (*self.as_ptr()).is_none() }
        }
        #[cfg(feature = "safe")]
        {
            let value = self.take();
            let result = value.is_none();
            self.set(value);
            result
        }
    }
}

/// Extension trait for `Cell<Option<Weak<T>>>` to access weak references without moving them.
pub trait CellOptionWeak<T> {
    /// Upgrade the weak reference to a strong reference without taking it out of the Cell.
    fn upgrade(&self) -> Option<Rc<T>>;
    /// Clone the weak reference without taking it out of the Cell.
    fn clone_inner(&self) -> Option<Weak<T>>;
}

/// Implements CellOptionWeak for Cell<Option<Weak<T>>>.
///
/// Provides operations on weak references contained in a cell without
/// moving them. Uses unsafe pointer access by default, with a safe
/// fallback when the "safe" feature is enabled.
impl<T> CellOptionWeak<T> for Cell<Option<Weak<T>>> {
    #[inline]
    fn upgrade(&self) -> Option<Rc<T>> {
        #[cfg(not(feature = "safe"))]
        #[allow(unsafe_code)]
        {
            unsafe { (*self.as_ptr()).as_ref().and_then(Weak::upgrade) }
        }
        #[cfg(feature = "safe")]
        {
            let value = self.take();
            let result = value.as_ref().and_then(Weak::upgrade);
            self.set(value);
            result
        }
    }

    #[inline]
    fn clone_inner(&self) -> Option<Weak<T>> {
        #[cfg(not(feature = "safe"))]
        #[allow(unsafe_code)]
        {
            unsafe { (*self.as_ptr()).clone() }
        }
        #[cfg(feature = "safe")]
        {
            let value = self.take();
            let result = value.clone();
            self.set(value);
            result
        }
    }
}

/// Extension trait for `Cell<Option<Rc<T>>>` to access strong references without moving them.
pub trait CellOptionRc<T> {
    /// Return `Some` if this `Rc` is the only strong reference count,
    /// even if there are weak references.
    fn take_if_unique_strong(&self) -> Option<Rc<T>>;
    /// Clone the strong reference without taking it out of the Cell.
    fn clone_inner(&self) -> Option<Rc<T>>;
}

/// Implements CellOptionRc for Cell<Option<Rc<T>>>.
///
/// Provides operations on strong references contained in a cell without
/// moving them. Includes specialized logic for handling unique strong
/// references. Uses unsafe pointer access by default, with a safe
/// fallback when the "safe" feature is enabled.
impl<T> CellOptionRc<T> for Cell<Option<Rc<T>>> {
    #[inline]
    fn take_if_unique_strong(&self) -> Option<Rc<T>> {
        #[cfg(not(feature = "safe"))]
        #[allow(unsafe_code)]
        {
            unsafe {
                match *self.as_ptr() {
                    None => None,
                    Some(ref rc) if Rc::strong_count(rc) > 1 => None,
                    // Not borrowing the `Rc<T>` here
                    // as we would be invalidating that borrow while it is outstanding:
                    Some(_) => self.take(),
                }
            }
        }
        #[cfg(feature = "safe")]
        {
            let value = self.take();
            let result = match value {
                None => None,
                Some(ref rc) if Rc::strong_count(rc) > 1 => {
                    self.set(value);
                    None
                }
                Some(rc) => Some(rc),
            };
            result
        }
    }

    #[inline]
    fn clone_inner(&self) -> Option<Rc<T>> {
        #[cfg(not(feature = "safe"))]
        #[allow(unsafe_code)]
        {
            unsafe { (*self.as_ptr()).clone() }
        }
        #[cfg(feature = "safe")]
        {
            let value = self.take();
            let result = value.clone();
            self.set(value);
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::{Rc, Weak};

    /// Tests is_none() when the cell contains None.
    ///
    /// Verifies that CellOption::is_none() returns true when the cell
    /// contains an empty Option.
    #[test]
    fn cell_option_is_none_with_none() {
        let cell: Cell<Option<i32>> = Cell::new(None);
        assert!(cell.is_none());
    }

    /// Tests is_none() when the cell contains Some.
    ///
    /// Verifies that CellOption::is_none() returns false when the cell
    /// contains a value.
    #[test]
    fn cell_option_is_none_with_some() {
        let cell = Cell::new(Some(42));
        assert!(!cell.is_none());
    }

    /// Tests upgrading a weak reference to a strong reference.
    ///
    /// Verifies that CellOptionWeak::upgrade() can successfully upgrade
    /// a weak reference when the strong reference is still alive.
    #[test]
    fn cell_option_weak_upgrade_some() {
        let rc = Rc::new(42);
        let weak = Rc::downgrade(&rc);
        let cell = Cell::new(Some(weak));

        let upgraded = cell.upgrade();
        assert!(upgraded.is_some());
        assert_eq!(*upgraded.unwrap(), 42);
    }

    /// Tests upgrading when the cell contains None.
    ///
    /// Verifies that CellOptionWeak::upgrade() returns None when the
    /// cell contains no weak reference.
    #[test]
    fn cell_option_weak_upgrade_none() {
        let cell: Cell<Option<Weak<i32>>> = Cell::new(None);
        assert!(cell.upgrade().is_none());
    }

    /// Tests cloning a weak reference from within a cell.
    ///
    /// Verifies that CellOptionWeak::clone_inner() produces a new weak
    /// reference that can be independently upgraded to access the value.
    #[test]
    fn cell_option_weak_clone_inner() {
        let rc = Rc::new(42);
        let weak = Rc::downgrade(&rc);
        let cell = Cell::new(Some(weak));

        let cloned = cell.clone_inner();
        assert!(cloned.is_some());
        assert_eq!(*cloned.unwrap().upgrade().unwrap(), 42);
    }

    /// Tests taking a unique strong reference from a cell.
    ///
    /// Verifies that CellOptionRc::take_if_unique_strong() successfully
    /// takes the Rc when it is the only strong reference.
    #[test]
    fn cell_option_rc_take_if_unique_strong() {
        let rc = Rc::new(42);
        let cell = Cell::new(Some(rc));

        let taken = cell.take_if_unique_strong();
        assert!(taken.is_some());
        assert_eq!(*taken.unwrap(), 42);
        assert!(cell.take().is_none());
    }

    /// Tests take_if_unique_strong when multiple references exist.
    ///
    /// Verifies that CellOptionRc::take_if_unique_strong() returns None
    /// when there are multiple strong references to the value, leaving
    /// the cell contents unchanged.
    #[test]
    fn cell_option_rc_take_if_unique_strong_multiple_refs() {
        let rc = Rc::new(42);
        let rc2 = rc.clone();
        let cell = Cell::new(Some(rc));

        let taken = cell.take_if_unique_strong();
        assert!(taken.is_none());
        assert!(cell.take().is_some());
        drop(rc2);
    }

    /// Tests cloning a strong reference from within a cell.
    ///
    /// Verifies that CellOptionRc::clone_inner() produces a new Rc
    /// pointing to the same value.
    #[test]
    fn cell_option_rc_clone_inner() {
        let rc = Rc::new(42);
        let cell = Cell::new(Some(rc));

        let cloned = cell.clone_inner();
        assert!(cloned.is_some());
        assert_eq!(*cloned.unwrap(), 42);
    }
}
