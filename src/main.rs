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

pub struct Display {}

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
    pub fn run_instruction(&mut self, ram: &mut Ram) {}
}

pub struct Chip8 {
    ram: Ram,
    cpu: Cpu,
}
impl Chip8 {
    pub fn new() -> Self {
        Self {
            ram: Ram::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for (i, v) in data.iter().enumerate() {
            self.ram.write_byte(PROGRAM_OFFSET + (i as u16), *v);
        }
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction(&mut self.ram);
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
