mod stack;
pub use crate::stack::stack_mod;
mod vm;
pub use crate::vm::vm_mod;
extern crate byteorder;
use byteorder::{ByteOrder, LittleEndian};
use std::io::prelude::*;
use std::fs::File;
const DEFAULT_RUNTIME_STACK_SIZE: usize = 512;

fn main() {
    let argv: Vec<_> = std::env::args().collect();

    if argv.len() != 2 {
        println!("Please enter the source file...");
        println!("USAGE : stack-vm filename");
        return ();
    }

    let mut file = File::open(argv[1].clone()).unwrap();

    let mut buf = [0; 4];
    let len = file.metadata().unwrap().len();
    let mut l = 0;

    let mut program: Vec<u32> = Vec::new();

    while l < len {
        file.read_exact(&mut buf).unwrap();
        let int = LittleEndian::read_u32(&buf[..]);
        program.push(int);
        println!(" {:?}", program);
        l += 4;
    }

    let mut vm = vm_mod::VM::new(DEFAULT_RUNTIME_STACK_SIZE);
    vm.load_program(&program);
    vm.run()
}

