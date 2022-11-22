struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            // println!("program counter: {}", self.position_in_memory);

            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xf000) >> 12) as u8;
            let x = ((opcode & 0x0f00) >> 8) as u8;
            let y = ((opcode & 0x00f0) >> 4) as u8;
            let d = ((opcode & 0x000f) >> 0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => { return; },
                (0x8, _, _, 0x4) => {
                    // println!("(c, x, y, d) = {:?}", (c, x, y, d));
                    self.add_xy(x, y)
                },
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        // println!("arg1: {}, arg2: {}", arg1, arg2);

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            /* println!("reached overflow");
            println!("before: {:?}", self.registers); */
            self.registers[0xf] = 1;
            // println!("after: {:?}", self.registers);
        } else {
            self.registers[0xf] = 0;
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;

    // tried addtional to book
    /* cpu.registers[3] = 10;
    cpu.registers[4] = 20;
    cpu.registers[5] = 30;
    cpu.registers[6] = 40;
    cpu.registers[7] = 50;
    cpu.registers[8] = 60;
    cpu.registers[9] = 70;
    cpu.registers[10] = 80;
    cpu.registers[11] = 90;
    cpu.registers[12] = 100;
    cpu.registers[13] = 110;
    cpu.registers[14] = 120;
    cpu.registers[15] = 30;  // this on will be overwritten for 'carry flag' */

    // println!("{:#?}", cpu.registers);

    let mem = &mut cpu.memory;
    mem[0] = 0x80; mem[1] = 0x14;
    mem[2] = 0x80; mem[3] = 0x24;
    mem[4] = 0x80; mem[5] = 0x34;

    // tried additional to book
    /* mem[6] = 0x80; mem[7] = 0x44;
    mem[8] = 0x80; mem[9] = 0x54;
    mem[10] = 0x80; mem[11] = 0x64;
    mem[12] = 0x80; mem[13] = 0x74;
    mem[14] = 0x80; mem[15] = 0x84; */
    /* mem[16] = 0x80; mem[17] = 0x94;  // overflowed from here
    mem[18] = 0x80; mem[19] = 0xa4;
    mem[20] = 0x80; mem[21] = 0xb4;
    mem[22] = 0x80; mem[23] = 0xc4;
    mem[24] = 0x80; mem[25] = 0xd4;
    mem[26] = 0x80; mem[27] = 0xe4;
    mem[28] = 0x80; mem[29] = 0xf4; */

    // println!("{:?}", mem);

    cpu.run();

    // assert_eq!(cpu.registers[0], 35);

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
