#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]

use std::fmt;
//use std::result;
use::std::ops::Neg;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
struct GF256(u8);

struct Polynomial {
    coeffs: Vec<GF256>,
}

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

impl Neg for GF256 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self // Negation is simply the value itself so, yh, nothing happens here
    }
}

impl Polynomial {
    fn new(coeffs: Vec<GF256>) -> Self {
        Self { coeffs }
    }

    fn evaluate(&self, x: GF256) -> GF256 {
        let mut result = GF256(0);
        for &coeff in self.coeffs.iter().rev() {
            result = result.mul(x).add(coeff);
        }

        result
    }

    fn add(&self, other: &Polynomial) -> Polynomial {
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut new_coeffs = vec![GF256(0); len];

        for i in 0..self.coeffs.len() {
            new_coeffs[i] = new_coeffs[i].add(self.coeffs[i]);
        }
        for i in 0..other.coeffs.len() {
            new_coeffs[i] = new_coeffs[i].add(other.coeffs[i]);
        }

        Polynomial::new(new_coeffs)
    }

    fn mul(&self, other: &Polynomial) -> Polynomial {
        let mut result = vec![GF256(0); self.coeffs.len() + other.coeffs.len() - 1];

        for (i, &a) in self.coeffs.iter().enumerate() {
            for (j, &b) in other.coeffs.iter().enumerate() {
                result[i + j] = result[i + j].add(a.mul(b));
            }
        }

        Polynomial::new(result)
    }

    fn scalar_mul(&self, scalar: GF256) -> Polynomial {
        Polynomial::new(self.coeffs.iter()
            .map(|&c| c.mul(scalar)).collect())
    }
    // We compute the lagrange basis polynomial l_i(x) in GF(256)
    fn lagrange_basis(i: usize, xs: &[GF256]) -> Polynomial {
        let mut numer = Polynomial::new(vec![GF256(1)]);
        let mut denom = GF256(1);

        for (j, &xj) in xs.iter().enumerate() {
            if i != j {
                let term = Polynomial::new(vec![xj.neg(), GF256(1)]); // basc=ically means (x - xj) on top
                numer = numer.mul(&term);
                denom = denom.mul(xs[i].add(xj)); // this should mean x_i - x_j (in GF(256), addition and subtraction are treated with the same XOR)
            }
        }

        // Multiply by the multiplcative inverse of the denominator
        numer.scalar_mul(denom.inverse())
    }

    // We'll perform the lagrange interpolation to find P(x) that passes through given points (our encoded message in this case)
    fn lagrange_interpolation(xs: &[GF256], ys: &[GF256]) -> Polynomial {
        assert_eq!(xs.len(), ys.len(), "Mismatch between x and y values");

        let mut result = Polynomial::new(vec![GF256(0)]);

        for i in 0..xs.len() {
            let li = Polynomial::lagrange_basis(i, xs);
            let term = li.scalar_mul(ys[i]);
            result = result.add(&term);
        }

        result

    }
    
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let terms: Vec<String> = self.coeffs.iter().enumerate()
            .filter(|(_, &c)| c.0 != 0)
            .map(|(i, &c)| {
                if i == 0 {
                    format!("{}", c)
                } else {
                    format!("{}x^{}", c, i)
                }
            }).collect();

            write!(f, "{}", terms.join(" + "))
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

    let p1 = Polynomial::new(vec![GF256(3), GF256(7), GF256(7)]); // should return 3 + 5x + 7x²
    let p2 = Polynomial::new(vec![GF256(1), GF256(2)]); // should return 1 + 2x

    println!("p1 = {}", p1);
    println!("p2 = {}", p2);
    println!("p1 + p2 = {}", p1.add(&p2));
    println!("p1 * p2 = {}", p1.mul(&p2));
    println!("p1(2) = {}", p1.evaluate(GF256(2)));

    let xs = vec![GF256(1), GF256(2), GF256(3)];
    let ys = vec![GF256(5), GF256(9), GF256(17)];

    let poly = Polynomial::lagrange_interpolation(&xs, &ys);
    println!("Interpolation Polynomials: {}", poly);

    for i in 0..xs.len() {
        assert_eq!(poly.evaluate(xs[i]), ys[i]);
    }

    println!("Interpolation Verified!");

}