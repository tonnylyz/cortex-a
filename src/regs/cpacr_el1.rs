// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2019 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! CPACR - EL1

use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
    sys_coproc_read_raw!(u64, "CPACR_EL1");
    sys_coproc_write_raw!(u64, "CPACR_EL1");
}

pub static CPACR_EL1: Reg = Reg {};
