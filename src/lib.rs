#![feature(bigint_helper_methods)]
#![feature(unboxed_closures)]
#![feature(trait_upcasting)]
#![cfg_attr(coverage_nightly, feature(no_coverage))]
pub mod gameboy;
pub mod display;
pub mod emu;
pub mod controller;
pub mod rom;
pub mod utils;
