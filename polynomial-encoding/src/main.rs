use std::fmt;

struct Polynomial {
    coefficients: Vec<f64>
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Polynomial: ")?;

        let polynomial = &self.coefficients;

        for (i, coefficient) in polynomial.iter().enumerate() {
            if i > 0 {
                write!(f, " + ")?;
            }
            write!(f, "{}x^{}", coefficient, i)?;
        }

        Ok(())
    }
}

impl Polynomial {
    fn from_message(message: &str) -> Self {
        let bytes = message.as_bytes();
        let mut coefficients = Vec::new();

        for (_i, bytes) in bytes.iter().enumerate() {
            coefficients.push(*bytes as f64);
        }

        Self {
            coefficients
        }
    }
    fn lagrange(&self, x:f64) -> f64 {
        let mut result = 0.0;

        for (i, coefficient) in self.coefficients.iter().enumerate() {
            let mut term = *coefficient;
            for (j, other_coefficient) in self.coefficients.iter().enumerate() {
                if i != j {
                    term *= (x - *other_coefficient as f64) / (*coefficient as f64 - *other_coefficient as f64);
                }
            }

            result += term;
        }

        result
    }
}

fn main() {
    let message = "Hello, World!";

    let poly = Polynomial::from_message(message);

    let x = 2.0;
    let result = poly.lagrange(x);

    println!("Lagrange Interpolation at x = {}: {} for {}", x, result, poly);

    // println!("{}", poly);

}