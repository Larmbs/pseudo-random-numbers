struct XorShift32_State {
    a: u32,
}
impl XorShift32_State { 
    pub fn generate(&mut self) -> u32 {
        let x = self.a;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.a = x;
        x
    }
}

struct XorShift64_State {
    a: u64,
}
impl XorShift64_State { 
    pub fn generate(&mut self) -> u64 {
        let x = self.a;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.a = x;
        x
    }
}

/* Can optionally be represented as two u64 values */
struct XorShift128_State {
    a: [4; u32],
}
impl XorShift128_State { 
    pub fn generate(&mut self) -> u128 {
        let t = self.a[3];
        let s = self.a[0]

        self.a[3] = self.a[2]
        self.a[2] = self.a[1]
        self.a[1] = s;

        t ^= t << 11;
	    t ^= t >> 8;
        self.a[0] = t ^ s ^ (s >> 19);

        self.a[0]
    }
}
