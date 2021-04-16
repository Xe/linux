// SPDX-License-Identifier: GPL-2.0

#![no_std]
#![feature(allocator_api, global_asm)]
#![feature(test)]

use alloc::boxed::Box;
use core::pin::Pin;
use kernel::prelude::*;
use kernel::{chrdev, cstr, file_operations::{FileOperations, FileOpener, File}, user_ptr::UserSlicePtrWriter};

module! {
    type: PrinterFacts,
    name: b"printerfacts",
    author: b"Christine Dodrill <me@christine.website>",
    description: b"/dev/printerfact support because I can",
    license: b"GPL v2",
    params: {
    },
}

struct RustFile;

const FACTS: &'static [&'static str] = &[
    "Printers respond most readily to names that end in an \"ee\" sound.",
	  "Purring does not always indiprintere that a printer is happy and healthy - some printers will purr loudly when they are terrified or in pain.",
	  "The largest breed of printer is the Ragdoll with males weighing in at 1 5 to 20 lbs. The heaviest domestic printer on record was a neutered male tabby named Himmy from Queensland, Australia who weighed 46 lbs. 1 5 oz.",
	  "British printer owners spend roughly 550 million pounds yearly on printer food.",
	  "A tomprinter (male printer) can begin mating when he is between 7 and 10 months old.",
	  "Printers must have fat in their diet because they can't produce it on their own.",
	  "The oldest printer on record was probably \"Puss\", a tabby owned by Mrs. Holway of Clayhidon, Devon. Having celebrated his 36th birthday on November 28, 1939, Puss died the following day.",
	  "The Pilgrims were the first to introduce printers to North America.",
];

impl RustFile {
    fn get_fact(&self) -> KernelResult<&'static str> {
        let mut ent: &[u8; 1] = &[0];
        kernel::random::getrandom(&mut ent)?;

        Ok(FACTS[ent[0] % FACTS.len()])
    }
}

impl FileOpener<()> for RustFile {
    fn open(_ctx: &()) -> KernelResult<Self::Wrapper> {
        pr_info!("rust file was opened!\n");
        Ok(Box::try_new(Self)?)
    }
}

impl FileOperations for RustFile {
    type Wrapper = Box<Self>;

    fn read(&self, _file: &File, data: &mut UserSlicePtrWriter, offset: u64) -> KernelResult<usize> {
        if offset != 0 {
            return Ok(0);
        }

        let fact = self.get_fact()?;
        data.write_slice(fact.as_bytes())?;
        Ok(fact.len())
    }

    kernel::declare_file_operations!(read);
}

struct PrinterFacts {
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
            _chrdev: chrdev_reg,
        })
    }
}

impl Drop for PrinterFacts {
    fn drop(&mut self) {
        pr_info!("printerfacts exiting");
    }
}
