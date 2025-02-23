#[allow(dead_code)]
#[allow(unused_variables)]

#[derive(Clone, Debug, Copy, ParitalEq, Eq)]
struct GF256(u8);

impl GF256 {
    const IRREDUCIBLE: u8 = 0x1B; // This is AES Standard

    // Addition and Subtraction is pretty much the same here
    fn add(&self, other: GF256) -> GF256{
        GF256(self.0 ^ other.0)
    }

    // We'll implement the Russian Peasant Algorithm
    fn mul(&self, other: GF256) {
        let mut a = self.0;
        let mut b = other.0;
        let mut result = 0u8;

        while b > 0 {
            if b & 1 != 0 {
                result ^= a;
            }
            let carry = a & 0x80;
            a <<= 1; // Multiply by 1
        }

        GF256(result)
    }
}

fn main() {

}