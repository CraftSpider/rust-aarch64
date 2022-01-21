// Copyright (c) 2020 Stefan Lankes, RWTH Aachen University
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(stable_features)]
#![feature(core_intrinsics)]
#![no_std]

use core::arch::asm;

pub mod paging;
pub mod regs;

/// The halt function stops the processor until the next interrupt arrives
#[inline(always)]
pub unsafe fn halt() {
    asm!("wfi", options(nostack, nomem),);
}
