// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2020 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! Counter-timer Physical Timer TimerValue register - EL0
//!
//! Holds the timer value for the EL1 physical timer.

use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u32, ()> for Reg {
    sys_coproc_read_raw!(u32, "CNTP_TVAL_EL0");
    sys_coproc_write_raw!(u32, "CNTP_TVAL_EL0");
}

pub static CNTP_TVAL_EL0: Reg = Reg {};
