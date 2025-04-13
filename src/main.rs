use std::{fs::File, io::Read};

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

pub struct Chip8 {
    ram: Ram,
}
impl Chip8 {
    pub fn new() -> Self {
        Self { ram: Ram::new() }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        let offset = 0x200;
        for (i, v) in data.iter().enumerate() {
            self.ram.write_byte((offset + i) as u16, *v);
        }
    }
}

fn main() {
    let mut file = File::open("roms/programs/IBM Logo.ch8").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).unwrap();

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);
    println!("Ram dump: {:x?}", chip8.ram.mem);
}
