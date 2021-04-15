// SPDX-License-Identifier: GPL-2.0

//! Rust example module

#![no_std]
#![feature(allocator_api, global_asm)]
#![feature(test)]

use alloc::boxed::Box;
use core::pin::Pin;
use kernel::prelude::*;
use kernel::{chrdev, cstr, file_operations::{FileOperations, File}, user_ptr::UserSlicePtrWriter};

module! {
    type: PrinterFacts,
    name: b"printerfacts",
    author: b"Christine Dodrill <me@christine.website>",
    description: b"/dev/printerfacts support because I can",
    license: b"GPL v2",
    params: {
    },
}

struct RustFile;

impl FileOperations for RustFile {
    type Wrapper = Box<Self>;

    kernel::declare_file_operations!();

    fn open() -> KernelResult<Self::Wrapper> {
        pr_info!("rust file was opened!");
        Ok(Box::try_new(Self)?)
    }

    fn read(&self, file: &File, data: &mut UserSlicePtrWriter, _offset: u64) -> KernelResult<usize> {
        pr_info!("user attempted to read from the file!");

        Ok(0)
    } 
}

struct PrinterFacts {
    message: String,
    _chrdev: Pin<Box<chrdev::Registration<2>>>,
}

impl KernelModule for PrinterFacts {
    fn init() -> KernelResult<Self> {
        pr_info!("printerfacts initialized");
        pr_info!("Am I built-in? {}", !cfg!(MODULE));

        let mut chrdev_reg =
            chrdev::Registration::new_pinned(cstr!("printerfact"), 0, &THIS_MODULE)?;
        chrdev_reg.as_mut().register::<RustFile>()?;
        chrdev_reg.as_mut().register::<RustFile>()?;

        Ok(PrinterFacts {
            message: "Hello world!".to_owned(),
            _chrdev: chrdev_reg,
        })
    }
}

impl Drop for PrinterFacts {
    fn drop(&mut self) {
        pr_info!("My message is {}", self.message);
        pr_info!("printerfacts exiting");
    }
}
