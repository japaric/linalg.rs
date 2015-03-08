#![feature(custom_attribute)]
#![feature(plugin)]
#![plugin(quickcheck_macros)]

extern crate approx;
extern crate linalg;
extern crate onezero;
extern crate quickcheck;
extern crate rand;

#[macro_use]
mod setup;

mod trans_view_mul;
