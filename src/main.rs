use std::{fs::File, io::Read};

#[derive(Debug)]
pub struct Ram {
    mem: [u8; 4096],
}
impl Ram {
    pub fn new() -> Self {
        Self { mem: [0; 4096] }
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        return self.mem[address as usize];
    }
}

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
    let size = file.read_to_end(&mut data).unwrap();

    let mut     chip8 = Chip8::new();
    chip8.load_rom(&data);
    println!("Ram dump: {:?}", chip8.ram.mem);
}
