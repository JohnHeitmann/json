#![cfg_attr(not(feature = "with-syntex"), feature(custom_attribute, custom_derive, plugin))]
#![cfg_attr(not(feature = "with-syntex"), plugin(serde_macros))]

#![feature(test)]

extern crate num;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
extern crate test;

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/bench.rs"));

#[cfg(not(feature = "with-syntex"))]
include!("bench.rs.in");
