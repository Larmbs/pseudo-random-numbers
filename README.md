# Pseudo Random Number Generators (PRNG)

## Intro
PRNG's is a category for random-like number generators where outputs are seemingly random. These generators take in an initial random value and generate a string a of values stemming from that one. PRNG's output is completely determined by its ***initial value or seed***.

## Benchmarking
### BSI evaluation criteria
The German Federal Office for Information Security (German: Bundesamt für Sicherheit in der Informationstechnik, BSI) has established four criteria for quality of deterministic random number generators.[21] They are summarized here:

- K1 – There should be a high probability that generated sequences of random numbers are different from each other.
- K2 – A sequence of numbers is indistinguishable from "truly random" numbers according to specified statistical tests. These requirements are a test of how well a bit sequence: has zeros and ones equally often.
- K3 – It should be impossible for an attacker to calculate, or otherwise guess, from any given subsequence, any previous or future values in the sequence, nor any inner state of the generator.
- K4 – It should be impossible, for all practical purposes, for an attacker to calculate, or guess from an inner state of the generator, any previous numbers in the sequence or any previous inner generator states.

For cryptographic applications, only generators meeting the K3 or K4 standards are acceptable. 

## Common Modern Generators
Over the years there have been advancements in the means of generating random values. 

### Middle Squared Method
One of the earliest apprchoes came from John von Neumann is known as the [Middle Squared Method](https://en.wikipedia.org/wiki/Middle-square_method). You needed to provide an initial value, say 1111. Then you would square it and the number would contain 8 digits 01234321, finally pick out the middle three digits and that would be your random number. To get better results try using number with more digits. While this method worked at the time, it contained a major flaw, at a certain point these digits would begin repeating themselves ruining the effect.

### XorShift
Xorshift random number generators, also called shift-register generators, were invented by George Marsaglia. They generate the next number in their sequence by repeatedly taking the [exclusive or](https://en.wikipedia.org/wiki/Bitwise_operation#XOR) of a number with a bit-shifted version of itself. This makes execution extremely efficient on modern computer architectures, but it does not benefit efficiency in a hardware implementation. Like all LFSRs, the parameters have to be chosen very carefully in order to achieve a long period.

For execution in software, xorshift generators are among the fastest PRNGs, requiring very small code and state. However, they do not pass every statistical test without further refinement. This weakness is amended by combining them with a non-linear function, as described in the original paper.

#### XorShift Examples
Examples for 32, 64, and 128 bit integers
``` Rust
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
```

## Sources
Parts of Wikipedia content was pasted in making of this for accuracy.
- Wikipedia: Xorshift https://en.wikipedia.org/wiki/Xorshift
- Wikipedia: Pseudorandom_number_generator https://en.wikipedia.org/wiki/Pseudorandom_number_generator
- Wikipedia: Random_number_generation https://en.wikipedia.org/wiki/Random_number_generation
- Wikipedia: Middle-square_method (https://en.wikipedia.org/wiki/Middle-square_method)
- Wikipedia: Bitwise_operation#XOR https://en.wikipedia.org/wiki/Bitwise_operation#XOR
