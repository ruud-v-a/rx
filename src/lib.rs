// Rx -- Reactive programming for Rust
// Copyright 2016 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

//! Rx, a library for reactive programming in Rust.
//! Inspired by [Reactive Extensions for C#](https://rx.codeplex.com/).
//!
//! TODO: Add intro to observables for people who are unfamiliar with them.
//!
//! TODO: Add examples.

#![warn(missing_docs)]
#![feature(fn_traits, unboxed_closures)]

use std::iter::IntoIterator;

mod generate;
mod observable;
mod observer;
mod subject;
mod transform;

pub use generate::error;
pub use observable::Observable;
pub use observer::Observer;
pub use subject::Subject;

/// A subscription where `drop()` is a no-op.
pub struct UncancellableSubscription;

impl Drop for UncancellableSubscription {
    fn drop(&mut self) { }
}

/// Observable implementation for types that can be converted into an iterator.
///
/// Upon subscription, this pushes a value for every value returned by the
/// iterator and then completes (if the iterator is finite). The returned
/// subscription is not cancellable: if the observable completes, it completes
/// before the call to `subscribe()` returns.
impl<'i, I> Observable for &'i I where &'i I: IntoIterator, <&'i I as IntoIterator>::Item: Clone {
    type Item = <&'i I as IntoIterator>::Item;
    type Error = ();
    type Subscription = UncancellableSubscription;

    fn subscribe<O>(&mut self, mut observer: O) -> UncancellableSubscription
        where O: Observer<Self::Item, Self::Error> {
        for x in self.into_iter() {
            observer.on_next(x);
        }
        observer.on_completed();
        UncancellableSubscription
    }
}

/// Observable implementation for `Result`.
///
/// Upon subscription, this pushes either the result and completes, or the
/// observable fails with the error. The returned subscription is not
/// cancellable: the observable completes or fails before the call to
/// `subscribe()` returns.
impl<T: Clone, E: Clone> Observable for Result<T, E> {
    type Item = T;
    type Error = E;
    type Subscription = UncancellableSubscription;

    fn subscribe<O>(&mut self, mut observer: O) -> UncancellableSubscription
        where O: Observer<Self::Item, Self::Error> {
        match *self {
            Ok(ref item) => {
                observer.on_next(item.clone());
                observer.on_completed();
            }
            Err(ref error) => {
                observer.on_error(error.clone());
            }
        }
        UncancellableSubscription
    }
}
