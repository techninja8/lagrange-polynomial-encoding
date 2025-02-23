#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]

use std::fmt;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
struct GF256(u8);

impl GF256 {
    const IRREDUCIBLE: u8 = 0x1B; // This is AES Standard

    // Addition and Subtraction is pretty much the same here
    fn add(&self, other: GF256) -> GF256{
        GF256(self.0 ^ other.0)
    }

    // We'll implement the Russian Peasant Algorithm
    fn mul(&self, other: GF256)  -> GF256 {
        let mut a = self.0;
        let mut b = other.0;
        let mut result = 0u8;

        while b > 0 {
            if b & 1 != 0 {
                result ^= a;
            }
            let carry = a & 0x80;
            a <<= 1; // Multiply by 1
            if carry != 0 {
                a ^= Self::IRREDUCIBLE;
            }

            b >>= 1;
        }

        GF256(result)
    }

    fn inverse(&self) -> GF256 {
        let mut a: u16 = self.0 as u16;
        let mut b: u16 = 0x11B; // Modulus rrstriction
        let mut x0: u16 = 1;
        let mut x1: u16 = 0;

        while a > 1 {
            let q = a / b; // Quotient
            let (new_a, new_b) = (b, a%b);
            let (new_x0, new_x1) = (x1, x0 ^ Self::gf_mul(q as u8, x1 as u8) as u16);

            a = new_a;
            b = new_b;
            x0 = new_x0;
            x1 = new_x1;
        }

        GF256(x0 as u8)
    }

    fn gf_mul(a: u8, b: u8) -> u8 {
        let mut x = a;
        let mut y = b;
        let mut result = 0u8;

        while y > 0 {
            if y & 1 != 0 {
                result ^= x;
            }
            let carry = x & 0x80;
            x <<= 1;
            if carry != 0 {
                x ^= 0x1B;
            }
            y >>= 1;
        }
        result
    }
}

impl fmt::Display for GF256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02X}", self.0)
    }
} 

fn main() {
    let a = GF256(7);
    let b = GF256(3);

    println!("a = {}", a);
    println!("b = {}", b);
    println!("a + b = {}", a.add(b));
    println!("a * b = {}", a.mul(b));
    println!("a⁻¹ = {}", a.inverse());
    println!("b⁻¹ = {}", b.inverse());
}