// SPDX-License-Identifier: GPL-2.0

//! Rust infiniband mls4 device sample.

use kernel::prelude::*;
use kernel::{
    file::{self, File},
    io_buffer::{IoBufferReader, IoBufferWriter},
    mlx4,
    sync::{Arc, ArcBorrow, CondVar, Mutex, UniqueArc},
};

module! {
    type: RustMiscdev,
    name: "rust_mlx4",
    author: "Rust for Linux Contributors",
    description: "Rust infiniband mlx4 device sample",
    license: "GPL",
}

struct RustMlx4 {
    _dev: Pin<Box<mlx4::Registration>>,
}

impl kernel::Module for RustRxe {
    fn init(name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust infiniband mlx4 driver sample (init)\n");

        Ok(RustRxe {
            _dev: _reg = mlx4::Registration::new_pinned(name),
        })
    }
}

impl Drop for RustChrdev {
    fn drop(&mut self) {
        pr_info!("Rust infiniband mlx4 driver sample (exit)\n");
    }
}
