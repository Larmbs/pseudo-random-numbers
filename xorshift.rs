struct XorShift32_State {
    pub a: u32,
}
impl XorShift32_State { 
    pub fn generate(&mut self) -> u32 {
        let mut x = self.a;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.a = x;
        x
    }
}

struct XorShift64_State {
    pub a: u64,
}
impl XorShift64_State { 
    pub fn generate(&mut self) -> u64 {
        let mut x = self.a;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.a = x;
        x
    }
}

fn main() {
    let mut gen = XorShift32_State {
        a: 500,
    };
    println!("{}", gen.generate());
    println!("{}", gen.generate());
    println!("{}", gen.generate());
    println!("{}", gen.generate());
    println!("{}", gen.generate());
}