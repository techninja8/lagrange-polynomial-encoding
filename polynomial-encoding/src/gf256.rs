// MODULE FOR IMPLEMENTING GF256 (Finite Field Arithmetic in GF(2^8))

// We allow dead_code warnings since this is an experimental module
#![allow(dead_code)]

use std::fmt;
use std::ops::Neg;

/// Represents an element in GF(2^8), the finite field of 256 elements.
/// This is used in cryptographic applications, error correction, and more.
/// The field is defined by the irreducible polynomial x^8 + x^4 + x^3 + x + 1 (0x1B in hex).
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct GF256(pub u8);

impl GF256 {
    /// Irreducible polynomial used for multiplication in GF(2^8), following the AES standard.
    const IRREDUCIBLE: u8 = 0x1B;

    /// Addition in GF(256) is simply XOR of the two values, since addition and subtraction are identical in characteristic-2 fields.
    /// Example:
    /// ```
    /// let a = GF256(0x57);
    /// let b = GF256(0x83);
    /// let result = a.add(b); // GF256(0xD4)
    /// ```
    pub fn add(&self, other: GF256) -> GF256 {
        GF256(self.0 ^ other.0)
    }

    /// Multiplication in GF(2^8) using the Russian Peasant Multiplication algorithm.
    /// This method performs polynomial multiplication modulo the irreducible polynomial.
    /// 
    /// Steps:
    /// - If the least significant bit (LSB) of `b` is 1, XOR `result` with `a`.
    /// - Shift `a` left (equivalent to multiplying by `x` in polynomial form).
    /// - If `a` overflows (MSB is set), reduce it using the irreducible polynomial.
    /// - Shift `b` right to process the next bit.
    ///
    /// Example:
    /// ```
    /// let a = GF256(0x57);
    /// let b = GF256(0x83);
    /// let result = a.mul(b); // GF256(0xC1)
    /// ```
    pub fn mul(&self, other: GF256) -> GF256 {
        let mut a = self.0;
        let mut b = other.0;
        let mut result = 0u8;

        while b > 0 {
            if b & 1 != 0 {
                result ^= a; // Add (XOR) the current a if LSB of b is 1
            }
            let carry = a & 0x80; // Check if MSB is set (overflow detection)
            a <<= 1; // Multiply a by x (shift left)
            if carry != 0 {
                a ^= Self::IRREDUCIBLE; // Reduce modulo the irreducible polynomial
            }

            b >>= 1; // Divide b by 2 (shift right)
        }

        GF256(result)
    }

    /// Computes the multiplicative inverse in GF(2^8) using exponentiation by squaring.
    /// Since GF(256) forms a finite field, each nonzero element has a unique inverse.
    /// The inverse is computed using the fact that `a^(2^8 - 1) = 1`, so `a^(2^8 - 2) = a^(-1)`.
    ///
    /// Example:
    /// ```
    /// let a = GF256(0x53);
    /// let inv = a.inverse(); // GF256(0xCA)
    /// ```
    pub fn inverse(&self) -> GF256 {
        let mut result = GF256(1);
        let mut base = *self;
        let mut exp = 254; // Fermat's Little Theorem: a^(p-2) ≡ a^(-1) mod p

        while exp > 0 {
            if exp & 1 != 0 {
                result = result.mul(base);
            }
            base = base.mul(base); // Square the base
            exp >>= 1; // Reduce exponent
        }

        result
    }

    /// Alternative multiplication function using a direct bitwise approach.
    /// This is similar to `mul`, but written as a standalone function.
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
                x ^= 0x1B; // Reduce with AES irreducible polynomial
            }
            y >>= 1;
        }
        result
    }
}

/// Implements `Display` to format GF256 elements as hexadecimal numbers.
impl fmt::Display for GF256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02X}", self.0)
    }
}

/// Implements negation for GF256.
/// Since addition and subtraction are identical in GF(2^8), negation is a no-op.
impl Neg for GF256 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self // Negation does nothing because x + x = 0 in characteristic-2 fields
    }
}
