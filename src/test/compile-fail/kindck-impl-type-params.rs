// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Issue #14061: tests the interaction between generic implementation
// parameter bounds and trait objects.

struct S<T>;

trait Gettable<T> {}

impl<T: Send + Copy> Gettable<T> for S<T> {}

fn f<T>(val: T) {
    let t: S<T> = S;
    let a = &t as &Gettable<T>;
    //~^ ERROR the trait `core::kinds::Send` is not implemented
    //~^^ ERROR the trait `core::kinds::Copy` is not implemented
    let a: &Gettable<T> = &t;
    //~^ ERROR the trait `core::kinds::Send` is not implemented
    //~^^ ERROR the trait `core::kinds::Copy` is not implemented
}

fn foo<'a>() {
    let t: S<&'a int> = S;
    let a = &t as &Gettable<&'a int>;
    let t: Box<S<String>> = box S;
    let a = t as Box<Gettable<String>>;
    //~^ ERROR the trait `core::kinds::Copy` is not implemented
    let t: Box<S<String>> = box S;
    let a: Box<Gettable<String>> = t;
    //~^ ERROR the trait `core::kinds::Copy` is not implemented
}

fn main() { }
