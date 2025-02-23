// MODULE FOR IMPLEMENTING POLYNOMIALS

use std::fmt;
use crate::gf256::GF256;
use::std::ops::Neg;

pub struct Polynomial {
    coeffs: Vec<GF256>,
}

impl Polynomial {
    pub fn new(coeffs: Vec<GF256>) -> Self {
        Self { coeffs }
    }

    pub fn evaluate(&self, x: GF256) -> GF256 {
        let mut result = GF256(0);
        for &coeff in self.coeffs.iter().rev() {
            result = result.mul(x).add(coeff);
        }

        result
    }

    pub fn add(&self, other: &Polynomial) -> Polynomial {
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

    pub fn mul(&self, other: &Polynomial) -> Polynomial {
        let mut result = vec![GF256(0); self.coeffs.len() + other.coeffs.len() - 1];

        for (i, &a) in self.coeffs.iter().enumerate() {
            for (j, &b) in other.coeffs.iter().enumerate() {
                result[i + j] = result[i + j].add(a.mul(b));
            }
        }

        Polynomial::new(result)
    }

    pub fn scalar_mul(&self, scalar: GF256) -> Polynomial {
        Polynomial::new(self.coeffs.iter()
            .map(|&c| c.mul(scalar)).collect())
    }
    // We compute the lagrange basis polynomial l_i(x) in GF(256)
    pub fn lagrange_basis(i: usize, xs: &[GF256]) -> Polynomial {
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
    pub fn lagrange_interpolation(xs: &[GF256], ys: &[GF256]) -> Polynomial {
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