mod machine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut machine = machine::Machine::new();
    machine.read_bin("challenge.bin")?;

    while !machine.is_halted() {
        machine.next();
    }
    Ok(())
}