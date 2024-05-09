use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct Flags : u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
    }
}

impl Flags {
    #[allow(dead_code)]
    pub fn as_u32(&self) -> u32 { self.bits() }
}

pub fn main() {
    let x = Flags::A | Flags::C;    // A | C
    let y = x ^ Flags::A;           // C
    let z = y & (Flags::B);         // 0
    println!("{:#?}", z | Flags::A);
}