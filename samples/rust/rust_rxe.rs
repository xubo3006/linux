// SPDX-License-Identifier: GPL-2.0

//! Rust infiniband Soft-RoCE driver sample.

use kernel::prelude::*;

module! {
    type: RustRxe,
    name: "rust_rxe",
    author: "Rust for Linux Contributors",
    description: "Rust infiniband rxe driver sample",
    license: "GPL",
}

struct RustRxe {
    _dev: Pin<Box<rxe::Registration>>,
}

impl kernel::Module for RustRxe{
    fn init(name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust Soft-RoCE driver sample (init)\n");

        Ok(RustRxe{
            _dev:rxe_reg=rxe::Registration::new_pinned(name),
        })
    }
}

impl Drop for RustChrdev {
    fn drop(&mut self) {
        pr_info!("Rust Soft-RoCE driver sample (exit)\n");
    }
}