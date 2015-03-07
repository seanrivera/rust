// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:coherence-lib.rs

// Test that it's not ok for U to appear uncovered

extern crate "coherence-lib" as lib;
use lib::{Remote,Pair};

pub struct Cover<T>(T);

impl<T,U> Remote for Pair<Cover<T>,U> { }
//~^ ERROR type parameter `U` must be used as the type parameter for some local type

fn main() { }
