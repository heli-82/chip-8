use std::{fs::File, io::Read};

const PROGRAM_OFFSET: u16 = 0x200;

#[derive(Debug)]
pub struct Ram {
    mem: [u8; 4096],
}
impl Ram {
    pub fn new() -> Self {
        let mut mem = [0; 4096];
        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0],
            [0x20, 0x60, 0x20, 0x20, 0x70],
            [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80],
        ];
        let mut i = 0;
        for sprite in sprites {
            for ch in sprite {
                mem[i] = ch;
                i += 1;
            }
        }

        Self { mem: mem }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        return self.mem[address as usize];
    }
}

pub struct Display {
    matrix: [[bool; 64]; 32],
}
impl Display {
    pub fn new() -> Self {
        Self {
            matrix: [[false; 64]; 32],
        }
    }
}

#[derive(Debug)]
pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            vx: [0; 16],
            pc: PROGRAM_OFFSET,
            i: 0,
        }
    }
    pub fn run_instruction(&mut self, ram: &mut Ram, display: &mut Display) {
        let cmd = [
            ram.mem[self.pc as usize] / 16,
            ram.mem[self.pc as usize] % 16,
            ram.mem[(self.pc + 1) as usize] / 16,
            ram.mem[(self.pc + 1) as usize] % 16,
        ];

        match cmd[0] {
            //0NNN 	Execute machine language subroutine at address NNN
            //00E0 	Clear the screen
            //00EE 	Return from a subroutine
            0 => {
                if cmd == [0, 0, 14, 14] {
                    todo!()
                } else if cmd == [0, 0, 14, 0] {
                    display.matrix = [[false; 64]; 32];
                } else {
                }
            }
            //1NNN 	Jump to address NNN
            1 => {}
            //2NNN 	Execute subroutine starting at address NNN
            2 => {}
            //3XNN 	Skip the following instruction if the value of register VX equals NN
            3 => {}
            //4XNN 	Skip the following instruction if the value of register VX is not equal to NN
            4 => {
                if self.vx[cmd[1] as usize] != cmd[2] * 16 + cmd[3] {
                    self.pc += 2;
                }
            }
            //5XY0 	Skip the following instruction if the value of register VX is equal to the value of register VY
            5 => {
                if self.vx[cmd[1] as usize] != self.vx[cmd[2] as usize] {
                    self.pc += 2;
                }
            }
            //6XNN 	Store number NN in register VX
            6 => {
                self.vx[cmd[1] as usize] = cmd[2] * 16 + cmd[3];
            }
            //7XNN 	Add the value NN to register VX
            7 => {
                self.vx[cmd[1] as usize] += cmd[2] * 16 + cmd[3];
            }
            8 => match cmd[3] {
                //8XY0 	Store the value of register VY in register VX
                0 => {
                    self.vx[cmd[1] as usize] = self.vx[cmd[2] as usize];
                }
                //8XY1 	Set VX to VX OR VY
                1 => {
                    self.vx[cmd[1] as usize] = self.vx[cmd[1] as usize] | self.vx[cmd[2] as usize];
                }
                //8XY2 	Set VX to VX AND VY
                2 => {
                    self.vx[cmd[1] as usize] = self.vx[cmd[1] as usize] & self.vx[cmd[2] as usize];
                }
                //8XY3 	Set VX to VX XOR VY
                3 => {
                    self.vx[cmd[1] as usize] = self.vx[cmd[1] as usize] ^ self.vx[cmd[2] as usize];
                }
                //8XY4 	Add the value of register VY to register VX
                //Set VF to 01 if a carry occurs
                //Set VF to 00 if a carry does not occur
                4 => {
                    let carry: u8 = if (self.vx[cmd[1] as usize] as u16)
                        + (self.vx[cmd[2] as usize] as u16)
                        > 255
                    {
                        1
                    } else {
                        0
                    };
                    self.vx[cmd[1] as usize] += self.vx[cmd[2] as usize];
                    self.vx[15] = carry;
                }
                //8XY5 	Subtract the value of register VY from register VX
                //Set VF to 00 if a borrow occurs
                //Set VF to 01 if a borrow does not occur
                5 => {
                    let carry: u8 = if (self.vx[cmd[1] as usize] as i16)
                        - (self.vx[cmd[2] as usize] as i16)
                        < 0
                    {
                        1
                    } else {
                        0
                    };
                    self.vx[cmd[1] as usize] += self.vx[cmd[2] as usize];
                    self.vx[15] = carry;
                }
                //8XY6 	Store the value of register VY shifted right one bit in register VX¹
                //Set register VF to the least significant bit prior to the shift
                //VY is unchanged
                6 => {}
                //8XY7 	Set register VX to the value of VY minus VX
                //Set VF to 00 if a borrow occurs
                //Set VF to 01 if a borrow does not occur
                7 => {}
                //8XYE 	Store the value of register VY shifted left one bit in register VX¹
                //Set register VF to the most significant bit prior to the shift
                //VY is unchanged
                14 => {}
                _ => {}
            },
            //9XY0 	Skip the following instruction if the value of register VX is not equal to the value of register VY
            //Set VF to 01 if any set pixels are changed to unset, and 00 otherwise
            9 => {}
            //ANNN 	Store memory address NNN in register I
            10 => {}
            //BNNN 	Jump to address NNN + V0
            11 => {}
            //CXNN 	Set VX to a random number with a mask of NN
            12 => {}
            //DXYN 	Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in I
            13 => {}
            //EX9E 	Skip the following instruction if the key corresponding to the hex value currently stored in register VX is pressed
            //EXA1 	Skip the following instruction if the key corresponding to the hex value currently stored in register VX is not pressed
            14 => {}
            //FX07 	Store the current value of the delay timer in register VX
            //FX0A 	Wait for a keypress and store the result in register VX
            //FX15 	Set the delay timer to the value of register VX
            //FX18 	Set the sound timer to the value of register VX
            //FX1E 	Add the value stored in register VX to register I
            //FX29 	Set I to the memory address of the sprite data corresponding to the hexadecimal digit stored in register VX
            //FX33 	Store the binary-coded decimal equivalent of the value stored in register VX at addresses I, I + 1, and I + 2
            //FX55 	Store the values of registers V0 to VX inclusive in memory starting at address I
            //I is set to I + X + 1 after operation²
            //FX65 	Fill registers V0 to VX inclusive with the values stored in memory starting at address I
            //I is set to I + X + 1 after operation²
            15 => {}
            _ => {}
        }
        self.pc += 2;
    }
}

pub struct Chip8 {
    ram: Ram,
    cpu: Cpu,
    display: Display,
}
impl Chip8 {
    pub fn new() -> Self {
        Self {
            ram: Ram::new(),
            cpu: Cpu::new(),
            display: Display::new(),
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for (i, v) in data.iter().enumerate() {
            self.ram.write_byte(PROGRAM_OFFSET + (i as u16), *v);
        }
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction(&mut self.ram, &mut self.display);
    }
}

fn main() {
    let mut file = File::open("chip8-roms/programs/IBM Logo.ch8").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).unwrap();

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);
    println!("Cpu state: {:x?}", chip8.cpu);
    println!("Ram dump: {:x?}", chip8.ram.mem);
}
