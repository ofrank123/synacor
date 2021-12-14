use std::convert::TryInto;
use super::Machine;

impl Machine {
    pub fn init_table(self: &mut Self) {
        let table = &mut self.opcode_table;
        table.insert(0, halt);
        table.insert(1, unknown);
        table.insert(2, unknown);
        table.insert(3, unknown);
        table.insert(4, unknown);
        table.insert(5, unknown);
        table.insert(6, jmp);
        table.insert(7, jt);
        table.insert(8, jf);
        table.insert(9, unknown);
        table.insert(10, unknown);
        table.insert(11, unknown);
        table.insert(12, unknown);
        table.insert(13, unknown);
        table.insert(14, unknown);
        table.insert(15, unknown);
        table.insert(16, unknown);
        table.insert(17, unknown);
        table.insert(18, unknown);
        table.insert(19, out);
        table.insert(20, unknown);
        table.insert(21, noop);
    }
}

fn halt(mach: &mut Machine) {
    mach.halt();
}

fn jmp(mach: &mut Machine) {
    mach.set_ins_ptr(mach.read_arg(1));
}

fn jt(mach: &mut Machine) {
    if mach.read_arg(1) != 0 {
        mach.set_ins_ptr(mach.read_arg(2));
    } else {
        mach.advance(2);
    }
}

fn jf(mach: &mut Machine) {
    if mach.read_arg(1) == 0 {
        mach.set_ins_ptr(mach.read_arg(2));
    } else {
        mach.advance(2);
    }
}

fn out(mach: &mut Machine) {
    let c: u8 = mach.read_arg(1).try_into().unwrap();
    print!("{}", String::from_utf8(vec![c]).unwrap());

    mach.advance(1);
}

fn noop(mach: &mut Machine) {
    // Do nothing :)
    mach.advance(0);
}

fn unknown(mach: &mut Machine) {
    println!("Operation not implemented yet!");
    println!("Operation: {}", mach.read_opcode());
    mach.halt();
}