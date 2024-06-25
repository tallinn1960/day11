#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn part1_swift( input: &mut str) -> u64 {
    unsafe { part1_swift_ffi(input.as_mut_ptr(), input.len()as u64) }
}