// Copyright 2022 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause

use crate::common::error::Error;

/// An irq allocator which gives next available irq.
/// It is mainly used for non-legacy devices.
// There are a few reserved irq's on x86_64. We just skip all the inital
// reserved irq to make the implementaion simple. This could be later extended
// to cater more complex scenario.
#[derive(Debug)]
pub struct IrqAllocator {
    // Tracks the last allocated irq
    last_used_irq: u32,
    last_irq: u32,
}

impl IrqAllocator {
    pub fn new(last_used_irq: u32, last_irq: u32) -> Result<Self, Error> {
        if last_used_irq >= last_irq {
            return Err(Error::InvalidValue);
        }
        Ok(IrqAllocator {
            last_used_irq,
            last_irq,
        })
    }

    pub fn next_irq(&mut self) -> Result<u32, Error> {
        self.last_used_irq
            .checked_add(1)
            .ok_or(Error::IRQOverflowed)
            .and_then(|irq| {
                if irq > self.last_irq {
                    Err(Error::MaxIrq)
                } else {
                    self.last_used_irq = irq;
                    Ok(irq)
                }
            })
    }
}
