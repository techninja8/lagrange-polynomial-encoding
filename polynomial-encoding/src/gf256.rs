// MODULE FOR IMPLEMENTING GF256

// Experimental testing
#![allow(dead_code)]

use std::fmt;
use::std::ops::Neg;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct GF256(pub u8);

impl GF256 {
    const IRREDUCIBLE: u8 = 0x1B; // This is AES Standard

    // Addition and Subtraction is pretty much the same here
    pub fn add(&self, other: GF256) -> GF256{
        GF256(self.0 ^ other.0)
    }

    // We'll implement the Russian Peasant Algorithm
    pub fn mul(&self, other: GF256)  -> GF256 {
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

    pub fn inverse(&self) -> GF256 {
        let mut result = GF256(1);
        let mut base = *self;
        let mut exp = 254; // a^(2^8 - 2) is the inverse in this case

        while exp > 0 {
            if exp & 1 != 0 {
                result = result.mul(base);
            }
            base = base.mul(base);
            exp >>= 1;
        }

        result
    }

    pub fn gf_mul(a: u8, b: u8) -> u8 {
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

impl Neg for GF256 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self // Negation is simply the value itself so, yh, nothing happens here
    }
}