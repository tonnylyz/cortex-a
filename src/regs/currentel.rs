// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2020 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Current Exception Level
//!
//! Holds the current Exception level.

use register::{cpu::RegisterReadOnly, register_bitfields};

register_bitfields! {u32,
    pub CurrentEL [
        /// Current Exception level. Possible values of this field are:
        ///
        /// 00 EL0
        /// 01 EL1
        /// 10 EL2
        /// 11 EL3
        ///
        /// When the HCR_EL2.NV bit is 1, Non-secure EL1 read accesses to the CurrentEL register
        /// return the value of 0x2 in this field.
        ///
        /// This field resets to a value that is architecturally UNKNOWN.
        EL OFFSET(2) NUMBITS(2) [
            EL0 = 0,
            EL1 = 1,
            EL2 = 2,
            EL3 = 3
        ]
    ]
}

pub struct Reg;

impl RegisterReadOnly<u32, CurrentEL::Register> for Reg {
    sys_coproc_read_raw!(u32, "CurrentEL");
}

#[allow(non_upper_case_globals)]
pub static CurrentEL: Reg = Reg {};
