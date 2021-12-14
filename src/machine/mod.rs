use std::fs;
use std::collections::HashMap;
use std::error::Error;

pub mod opcode;

pub struct Machine {
    memory: Vec<u16>,
    ins_ptr: u16,
    halted: bool,
    opcode_table: HashMap<u16, fn(&mut Machine)>
}

impl Machine {
    pub fn new() -> Self {
        let mut m = Self { 
            memory: vec!(), 
            ins_ptr: 0,
            halted: false,
            opcode_table: HashMap::new()
        };
        m.init_table();
        m
    }
    pub fn read_bin(self: &mut Self, filename: &str) -> Result<(), Box<dyn Error>> {
        let data = &mut fs::read(filename)?
            .chunks(2)
            .map(|chunk| (chunk[0] as u16) + ((chunk[1] as u16) << 8)).collect();
        self.memory.append(data);
        Ok(())
    }
    pub fn next(self: &mut Self) {
        let opcode = self.read_opcode();
        match self.opcode_table.get(&opcode) {
            Some(op) => op(self),
            None => panic!("Invalid Opcode: {}", opcode)
        }
    }
    pub fn is_halted(self: &Self) -> bool {
        return self.halted;
    }
    pub fn halt(self: &mut Self) {
        self.halted = true;
    }
    pub fn advance(self: &mut Self, args: u16) {
        self.ins_ptr += 1 + args;
    }
    pub fn read_opcode(self: &Self) -> u16 {
        self.read_mem(self.ins_ptr)
    }
    pub fn read_arg(self: &Self, arg: u16) -> u16 {
        self.read_mem(self.ins_ptr + arg)
    }
    pub fn read_mem(self: &Self, loc: u16) -> u16 {
        if loc <= 32767 {
            return self.memory[loc as usize];
        }
        panic!("Segmentation Fault! :)");
    }
    pub fn set_ins_ptr(self: &mut Self, loc: u16) {
        self.ins_ptr = loc;
    }
}