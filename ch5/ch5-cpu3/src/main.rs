struct CPU {
    registers: [u8; 0x10],
    program_counter: usize,
    memory: [u8; 0x1000],
    stack: [u16; 0x10],
    stack_pointer: usize,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.program_counter;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.program_counter += 2;

            let c = ((opcode & 0xf000) >> 12) as u8;
            let x = ((opcode & 0x0f00) >> 8) as u8;
            let y = ((opcode & 0x00f0) >> 4) as u8;
            let d = ((opcode & 0x000f) >> 0) as u8;

            let nnn = opcode & 0x0fff;  // get address for function

            match (c, x, y, d) {
                (0, 0, 0, 0) => { return; },
                (0, 0, 0xe, 0xe) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack Overflow")
        }

        stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack Underflow")
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.program_counter = addr as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow_detected) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        self.registers[0xf] = if overflow_detected { 1 } else { 0 };
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 0x10],
        memory: [0; 0x1000],
        program_counter: 0,
        stack: [0; 0x10],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[10] = 10;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; mem[0x001] = 0x00;  // call function at 0x100
    mem[0x002] = 0x21; mem[0x003] = 0x00;
    mem[0x004] = 0x00; mem[0x005] = 0x00;

    // above opcode contain the address for this opcode, 0x2100 means call function (byte
    // executable) at 0x100
    mem[0x100] = 0x80; mem[0x101] = 0xa4;  // do the addtion
    mem[0x102] = 0x80; mem[0x103] = 0xa4;
    mem[0x104] = 0x00; mem[0x105] = 0xee;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
