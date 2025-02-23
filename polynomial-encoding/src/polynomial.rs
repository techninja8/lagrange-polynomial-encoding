use std::fmt;
use crate::gf256::GF256;
use std::ops::Neg;

/// Represents a polynomial with coefficients in GF(256).
pub struct Polynomial {
    coeffs: Vec<GF256>, // Stores coefficients in increasing order of powers of x
}

impl Polynomial {
    /// Creates a new polynomial from a vector of coefficients.
    pub fn new(coeffs: Vec<GF256>) -> Self {
        Self { coeffs }
    }

    /// Evaluates the polynomial at a given point `x` using Horner's method.
    ///
    /// Horner's method is used for efficient polynomial evaluation:
    ///     P(x) = a_n * x^n + a_(n-1) * x^(n-1) + ... + a_1 * x + a_0
    ///
    /// Instead of directly computing powers of `x`, we rewrite it as:
    ///     P(x) = (...((a_n * x + a_(n-1)) * x + a_(n-2)) * x ... + a_0)
    ///
    /// This reduces the number of multiplications significantly.
    pub fn evaluate(&self, x: GF256) -> GF256 {
        let mut result = GF256(0);
        for &coeff in self.coeffs.iter().rev() { // Iterate from highest to lowest degree
            result = result.mul(x).add(coeff);
        }
        result
    }

    /// Adds two polynomials together and returns a new polynomial.
    ///
    /// Addition in GF(256) is performed using XOR (⊕).
    /// The resulting polynomial has the maximum length of both input polynomials.
    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut new_coeffs = vec![GF256(0); len];

        // Add corresponding coefficients
        for i in 0..self.coeffs.len() {
            new_coeffs[i] = new_coeffs[i].add(self.coeffs[i]);
        }
        for i in 0..other.coeffs.len() {
            new_coeffs[i] = new_coeffs[i].add(other.coeffs[i]);
        }

        Polynomial::new(new_coeffs)
    }

    /// Multiplies two polynomials and returns a new polynomial.
    ///
    /// Uses the distributive property:
    /// (a_0 + a_1*x + ... + a_n*x^n) * (b_0 + b_1*x + ... + b_m*x^m)
    ///
    /// The resulting polynomial has degree `(n + m) - 1`.
    pub fn mul(&self, other: &Polynomial) -> Polynomial {
        let mut result = vec![GF256(0); self.coeffs.len() + other.coeffs.len() - 1];

        for (i, &a) in self.coeffs.iter().enumerate() {
            for (j, &b) in other.coeffs.iter().enumerate() {
                result[i + j] = result[i + j].add(a.mul(b)); // Multiply coefficients and add to the correct position
            }
        }

        Polynomial::new(result)
    }

    /// Multiplies the polynomial by a scalar value in GF(256).
    pub fn scalar_mul(&self, scalar: GF256) -> Polynomial {
        Polynomial::new(self.coeffs.iter()
            .map(|&c| c.mul(scalar)) // Multiply each coefficient by the scalar
            .collect())
    }

    /// Computes the Lagrange basis polynomial l_i(x) in GF(256).
    ///
    /// The Lagrange basis polynomial l_i(x) is defined as:
    /// l_i(x) = ∏ (x - x_j) / (x_i - x_j)   for all j ≠ i
    ///
    /// This polynomial is used in Lagrange interpolation to construct the full polynomial.
    pub fn lagrange_basis(i: usize, xs: &[GF256]) -> Polynomial {
        let mut numer = Polynomial::new(vec![GF256(1)]); // Start with the constant polynomial 1
        let mut denom = GF256(1);

        for (j, &xj) in xs.iter().enumerate() {
            if i != j {
                let term = Polynomial::new(vec![xj.neg(), GF256(1)]); // Represents (x - x_j)
                numer = numer.mul(&term);
                denom = denom.mul(xs[i].add(xj)); // In GF(256), subtraction is XOR
            }
        }

        // Multiply by the multiplicative inverse of the denominator to scale the polynomial correctly
        numer.scalar_mul(denom.inverse())
    }

    /// Performs Lagrange interpolation to construct a polynomial that passes through given points.
    ///
    /// Given pairs (x_0, y_0), ..., (x_n, y_n), the interpolating polynomial is:
    /// P(x) = ∑ y_i * l_i(x), where l_i(x) are Lagrange basis polynomials.
    pub fn lagrange_interpolation(xs: &[GF256], ys: &[GF256]) -> Polynomial {
        assert_eq!(xs.len(), ys.len(), "Mismatch between x and y values");

        let mut result = Polynomial::new(vec![GF256(0)]);

        for i in 0..xs.len() {
            let li = Polynomial::lagrange_basis(i, xs); // Compute l_i(x)
            let term = li.scalar_mul(ys[i]); // Multiply by corresponding y_i
            result = result.add(&term);
        }

        result
    }
}

impl fmt::Display for Polynomial {
    /// Formats the polynomial as a human-readable string.
    ///
    /// Example:
    /// For coefficients [5, 3, 2] (GF256 values), it returns: "5 + 3x^1 + 2x^2"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let terms: Vec<String> = self.coeffs.iter().enumerate()
            .filter(|(_, &c)| c.0 != 0) // Ignore zero coefficients
            .map(|(i, &c)| {
                if i == 0 {
                    format!("{}", c) // Constant term
                } else {
                    format!("{}x^{}", c, i) // Higher order terms
                }
            }).collect();

        write!(f, "{}", terms.join(" + "))
    }
}
